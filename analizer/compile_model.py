# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
compile_model.py — compile Bostrom cyberlinks into a graph-native transformer

Pipeline from bostrom-to-onnx-pipeline.md:
  1. Load cyberlinks from JSONL
  2. Build sparse adjacency matrix (CSR)
  3. Compute focus distribution (PageRank)
  4. Compute spectral gap (Lanczos)
  5. Randomized SVD → embedding matrix
  6. Derive architecture parameters (d*, h*, L*)

Output: numpy .npz with embeddings, focus, architecture params.
ONNX assembly is step 8 (separate, needs onnx package).

Usage:
  python3 analizer/compile_model.py data/cyberlinks.jsonl
  python3 analizer/compile_model.py data/cyberlinks.jsonl --max-links 100000  # sample
"""

import json
import sys
import os
import time
import numpy as np
from scipy.sparse import csr_matrix, diags
from scipy.sparse.linalg import svds, eigsh
from collections import defaultdict


def load_cyberlinks(path, max_links=None):
    """Step 1: Load edge list from JSONL"""
    print(f"Step 1: Loading cyberlinks from {path}...")
    t0 = time.time()

    links = []
    with open(path) as f:
        for i, line in enumerate(f):
            if max_links and i >= max_links:
                break
            row = json.loads(line)
            links.append((row["particle_from"], row["particle_to"], row.get("neuron", "")))

    # build particle index
    particles = {}
    for p_from, p_to, _ in links:
        if p_from not in particles:
            particles[p_from] = len(particles)
        if p_to not in particles:
            particles[p_to] = len(particles)

    print(f"  {len(links):,} links, {len(particles):,} particles in {time.time()-t0:.1f}s")
    return links, particles


def build_adjacency(links, particles):
    """Step 2: Sparse adjacency matrix (CSR)"""
    print("Step 2: Building sparse adjacency matrix...")
    t0 = time.time()

    rows, cols, vals = [], [], []
    # aggregate duplicate edges
    edge_weights = defaultdict(float)
    for p_from, p_to, neuron in links:
        i = particles[p_from]
        j = particles[p_to]
        edge_weights[(i, j)] += 1.0  # uniform weight (no stake data yet)

    for (i, j), w in edge_weights.items():
        rows.append(i)
        cols.append(j)
        vals.append(w)

    n = len(particles)
    A = csr_matrix((vals, (rows, cols)), shape=(n, n))

    nnz = A.nnz
    density = nnz / (n * n) if n > 0 else 0
    mem_mb = (nnz * 16) / 1024 / 1024
    print(f"  {n:,} x {n:,} matrix, {nnz:,} nonzeros, density={density:.2e}, ~{mem_mb:.1f} MB")
    print(f"  Built in {time.time()-t0:.1f}s")
    return A


def compute_focus(A, alpha=0.85, iterations=29, tol=1e-6):
    """Step 3: Focus distribution (PageRank)"""
    print(f"Step 3: Computing focus (PageRank, alpha={alpha}, max_iter={iterations})...")
    t0 = time.time()

    n = A.shape[0]
    # column-normalize: M = D^{-1} A
    out_degree = np.array(A.sum(axis=1)).flatten()
    out_degree[out_degree == 0] = 1  # avoid division by zero (dangling nodes)
    D_inv = diags(1.0 / out_degree)
    M = D_inv @ A

    pi = np.ones(n) / n
    teleport = (1 - alpha) / n

    for t in range(iterations):
        pi_new = alpha * (M.T @ pi) + teleport
        # handle dangling nodes
        dangling_mass = alpha * pi[out_degree == 1].sum() / n  # approximate
        pi_new += dangling_mass
        pi_new /= pi_new.sum()

        diff = np.abs(pi_new - pi).sum()
        pi = pi_new
        if diff < tol:
            print(f"  Converged at iteration {t+1}, diff={diff:.2e}")
            break

    # stats
    top_idx = np.argsort(-pi)[:10]
    print(f"  Focus computed in {time.time()-t0:.1f}s")
    print(f"  Max focus: {pi[top_idx[0]]:.6f}, min: {pi.min():.2e}")
    print(f"  Entropy: {-np.sum(pi * np.log(pi + 1e-15)):.2f} bits")
    return pi


def compute_spectral_gap(A):
    """Step 4: Spectral gap via Lanczos"""
    print("Step 4: Computing spectral gap...")
    t0 = time.time()

    n = A.shape[0]
    # normalized Laplacian: L = I - D^{-1/2} A D^{-1/2}
    degree = np.array(A.sum(axis=1)).flatten()
    degree[degree == 0] = 1
    D_inv_sqrt = diags(1.0 / np.sqrt(degree))
    L = diags(np.ones(n)) - D_inv_sqrt @ A @ D_inv_sqrt

    # find smallest eigenvalues (lambda_1 ≈ 0, lambda_2 = spectral gap)
    try:
        eigenvalues, _ = eigsh(L, k=min(6, n-1), which='SM', maxiter=100)
        eigenvalues = np.sort(eigenvalues)
        lambda2 = eigenvalues[1] if len(eigenvalues) > 1 else 0
    except Exception as e:
        print(f"  Warning: eigsh failed ({e}), estimating lambda2=0.001")
        lambda2 = 0.001

    kappa = 0.85 * (1 - lambda2)
    T_converge = int(np.ceil(np.log(100) / np.log(1 / kappa))) if kappa < 1 else 100

    print(f"  λ₂ = {lambda2:.6f}")
    print(f"  κ (contraction) = {kappa:.4f}")
    print(f"  T_converge = {T_converge} iterations")
    print(f"  Computed in {time.time()-t0:.1f}s")
    return lambda2, kappa, T_converge


def compute_embeddings(A, pi, target_d=None, oversampling=10):
    """Step 5: Randomized SVD → embedding matrix"""
    print("Step 5: Computing embeddings (randomized SVD)...")
    t0 = time.time()

    n = A.shape[0]
    # π-weighted adjacency
    pi_sqrt = np.sqrt(pi)
    A_weighted = diags(pi_sqrt) @ A

    # compute SVD — request enough components to determine d*
    k = min(100, n - 2)  # request up to 100 singular values
    try:
        U, sigma, Vt = svds(A_weighted, k=k)
    except Exception as e:
        print(f"  Warning: svds failed ({e}), trying with fewer components")
        k = min(20, n - 2)
        U, sigma, Vt = svds(A_weighted, k=k)

    # sort descending
    idx = np.argsort(-sigma)
    U, sigma = U[:, idx], sigma[idx]

    # effective rank d* from entropy of normalized singular values
    sigma_norm = sigma / (sigma.sum() + 1e-15)
    H = -np.sum(sigma_norm * np.log(sigma_norm + 1e-15))
    d_star = max(int(np.exp(H)), 2)

    if target_d:
        d_star = target_d

    d_star = min(d_star, k)
    E = U[:, :d_star]

    print(f"  Singular values: {sigma[:5].round(2)}...")
    print(f"  Entropy H = {H:.2f}")
    print(f"  Effective dimension d* = {d_star}")
    print(f"  Embedding shape: {E.shape}")
    print(f"  Computed in {time.time()-t0:.1f}s")
    return E, d_star, sigma


def estimate_architecture(d_star, lambda2, kappa, n_particles, n_links):
    """Step 6: Architecture parameters"""
    print("Step 6: Architecture parameters...")

    # attention heads from semcon estimate (minimum 4)
    h_star = max(4, int(np.sqrt(d_star)))

    # layers from diameter × convergence
    # estimate diameter as log(n) for sparse graph
    diameter = max(int(np.log(n_particles) / np.log(10)), 5)
    T_converge = int(np.ceil(np.log(100) / np.log(1 / kappa))) if kappa < 1 else 30
    L_star = min(diameter * T_converge, 200)  # cap at 200

    # parameter count
    emb_params = n_particles * d_star
    attn_params = h_star * 3 * d_star * d_star * L_star
    mlp_params = 2 * 4 * d_star * d_star * L_star
    out_params = d_star * n_particles
    total_params = emb_params + attn_params + mlp_params + out_params
    size_gb = total_params * 4 / 1024**3

    print(f"  d* = {d_star}")
    print(f"  h* = {h_star} (attention heads)")
    print(f"  L* = {L_star} (layers, diameter={diameter} × T={T_converge})")
    print(f"  Estimated diameter: {diameter}")
    print(f"")
    print(f"  Parameter count:")
    print(f"    Embedding:   {emb_params:>15,}")
    print(f"    Attention:   {attn_params:>15,}")
    print(f"    MLP:         {mlp_params:>15,}")
    print(f"    Output:      {out_params:>15,}")
    print(f"    Total:       {total_params:>15,}")
    print(f"    Size:        {size_gb:.2f} GB")

    return {
        "d_star": d_star,
        "h_star": h_star,
        "L_star": L_star,
        "diameter": diameter,
        "total_params": total_params,
        "size_gb": size_gb,
    }


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 compile_model.py <cyberlinks.jsonl> [--max-links N]")
        sys.exit(1)

    path = sys.argv[1]
    max_links = None
    if "--max-links" in sys.argv:
        idx = sys.argv.index("--max-links")
        max_links = int(sys.argv[idx + 1])

    t_total = time.time()

    # Pipeline
    links, particles = load_cyberlinks(path, max_links)
    A = build_adjacency(links, particles)
    pi = compute_focus(A)
    lambda2, kappa, T_conv = compute_spectral_gap(A)
    E, d_star, sigma = compute_embeddings(A, pi)
    arch = estimate_architecture(d_star, lambda2, kappa, len(particles), len(links))

    # Save results
    out_dir = os.path.dirname(path) or "."
    out_path = os.path.join(out_dir, "bostrom_model.npz")

    # particle index → CID mapping (reverse lookup)
    idx_to_particle = {v: k for k, v in particles.items()}
    particle_list = [idx_to_particle[i] for i in range(len(particles))]

    np.savez_compressed(out_path,
        embeddings=E,
        focus=pi,
        sigma=sigma,
        particle_cids=particle_list,
        **arch
    )

    print(f"\n{'='*60}")
    print(f"Compilation complete in {time.time()-t_total:.1f}s")
    print(f"Saved to {out_path}")
    print(f"  Particles:  {len(particles):,}")
    print(f"  Links:      {len(links):,}")
    print(f"  d* = {d_star}, h* = {arch['h_star']}, L* = {arch['L_star']}")
    print(f"  Model size: {arch['size_gb']:.2f} GB ({arch['total_params']:,} params)")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
