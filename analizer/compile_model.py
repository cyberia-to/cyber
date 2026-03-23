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
from scipy.sparse.linalg import svds
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


def load_stakes(stakes_path):
    """Load neuron stake weights from JSON"""
    if not stakes_path or not os.path.exists(stakes_path):
        return {}
    with open(stakes_path) as f:
        stakes = json.load(f)
    nonzero = sum(1 for v in stakes.values() if v > 0)
    total = sum(stakes.values())
    print(f"  Loaded {len(stakes)} neuron stakes ({nonzero} nonzero, {total:,.0f} total BOOT)")
    return stakes


def build_adjacency(links, particles, stakes=None):
    """Step 2: Sparse adjacency matrix (CSR)"""
    print("Step 2: Building sparse adjacency matrix...")
    t0 = time.time()

    if stakes:
        # normalize stakes to [0,1] range with log scaling
        max_stake = max(stakes.values()) if stakes else 1
        print(f"  Using stake weights (max: {max_stake:,.0f} BOOT)")

    rows, cols, vals = [], [], []
    # aggregate duplicate edges weighted by neuron stake
    edge_weights = defaultdict(float)
    for p_from, p_to, neuron in links:
        i = particles[p_from]
        j = particles[p_to]
        if stakes and neuron in stakes and stakes[neuron] > 0:
            # log-scale stake to prevent domination by whales
            w = np.log1p(stakes[neuron])
        else:
            w = 1.0  # fallback: uniform weight
        edge_weights[(i, j)] += w

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


def compute_focus(A, alpha=0.85, iterations=50, tol=1e-8):
    """Step 3+4: Focus distribution (PageRank) + spectral gap from convergence rate

    The convergence rate of PageRank IS the spectral gap, observed empirically.
    Between iterations: ||π^(t+1) - π*|| / ||π^(t) - π*|| → κ = α(1-λ₂)
    So we track the ratio of successive diffs to extract κ without eigsh.
    """
    print(f"Step 3+4: Computing focus + spectral gap (PageRank, alpha={alpha})...")
    t0 = time.time()

    n = A.shape[0]
    out_degree = np.array(A.sum(axis=1)).flatten()
    out_degree[out_degree == 0] = 1
    D_inv = diags(1.0 / out_degree)
    M = D_inv @ A

    pi = np.ones(n) / n
    teleport = (1 - alpha) / n

    diffs = []
    convergence_iter = iterations

    for t in range(iterations):
        pi_new = alpha * (M.T @ pi) + teleport
        dangling_mass = alpha * pi[out_degree == 1].sum() / n
        pi_new += dangling_mass
        pi_new /= pi_new.sum()

        diff = np.abs(pi_new - pi).sum()
        diffs.append(diff)
        pi = pi_new

        if diff < tol:
            convergence_iter = t + 1
            print(f"  Converged at iteration {t+1}, diff={diff:.2e}")
            break

    # Extract spectral gap from convergence rate
    # κ = ratio of successive diffs in the tail (where it's geometric)
    # Use last 5 ratios for stability
    ratios = []
    for i in range(max(3, len(diffs) - 5), len(diffs)):
        if diffs[i-1] > 1e-15:
            ratios.append(diffs[i] / diffs[i-1])

    if ratios:
        kappa = np.median(ratios)  # median is robust to outliers
        # κ = α(1 - λ₂)  →  λ₂ = 1 - κ/α
        lambda2 = max(0, 1 - kappa / alpha)
    else:
        kappa = alpha  # worst case: no gap
        lambda2 = 0

    T_converge = convergence_iter

    # stats
    top_idx = np.argsort(-pi)[:10]
    print(f"  Focus computed in {time.time()-t0:.1f}s")
    print(f"  Max focus: {pi[top_idx[0]]:.6f}, min: {pi.min():.2e}")
    print(f"  Entropy: {-np.sum(pi * np.log(pi + 1e-15)):.2f} bits")
    print(f"  Convergence diffs (last 5): {[f'{d:.2e}' for d in diffs[-5:]]}")
    print(f"  Contraction ratios (last 5): {[f'{r:.4f}' for r in ratios[-5:]]}")
    print(f"  κ (measured contraction) = {kappa:.6f}")
    print(f"  λ₂ (spectral gap) = {lambda2:.6f}")
    print(f"  T_converge = {T_converge} iterations")

    return pi, lambda2, kappa, T_converge


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


def assemble_onnx(E, pi, arch, out_path):
    """Step 8: Assemble ONNX transformer model"""
    try:
        import onnx
        from onnx import helper, TensorProto, numpy_helper
    except ImportError:
        print("  onnx package not installed, skipping ONNX assembly")
        print("  pip3 install onnx")
        return None

    print("Step 8: Assembling ONNX model...")
    t0 = time.time()

    n_particles = E.shape[0]
    d_star = arch["d_star"]
    h_star = arch["h_star"]
    # cap layers for practical model — full L* creates huge model
    L_star = min(arch["L_star"], 12)  # cap at 12 layers for tractability
    d_head = max(d_star // h_star, 1)
    d_ff = 4 * d_star  # MLP hidden dimension

    print(f"  Architecture: d={d_star}, h={h_star}, L={L_star} (capped), d_ff={d_ff}")

    nodes = []
    initializers = []
    rng = np.random.RandomState(42)  # deterministic

    # Embedding table — from compiled SVD
    E_float = E.astype(np.float32)
    initializers.append(numpy_helper.from_array(E_float, name="embedding_table"))

    # Input: token_ids [batch, seq_len]
    input_ids = helper.make_tensor_value_info("input_ids", TensorProto.INT64, [None, None])

    # Gather embeddings
    nodes.append(helper.make_node("Gather", ["embedding_table", "input_ids"], ["hidden_0"], axis=0))

    for l in range(L_star):
        h_in = f"hidden_{l}"
        h_out = f"hidden_{l+1}"

        # Simplified attention: W_QKV projection → MatMul → softmax → MatMul
        # QKV combined: [d_star, 3*d_star]
        W_qkv = (rng.randn(d_star, 3 * d_star) * 0.02).astype(np.float32)
        # Initialize Q,K from SVD structure: project through pi-weighted space
        # This is a simplification — full pipeline would use per-semcon SVD
        initializers.append(numpy_helper.from_array(W_qkv, name=f"W_qkv_{l}"))

        # QKV projection
        nodes.append(helper.make_node("MatMul", [h_in, f"W_qkv_{l}"], [f"qkv_{l}"]))

        # Split into Q, K, V using explicit split sizes
        split_sizes = np.array([d_star, d_star, d_star], dtype=np.int64)
        initializers.append(numpy_helper.from_array(split_sizes, name=f"split_{l}"))
        nodes.append(helper.make_node("Split", [f"qkv_{l}", f"split_{l}"], [f"q_{l}", f"k_{l}", f"v_{l}"],
                                       axis=-1))

        # Attention scores: Q @ K^T / sqrt(d)
        nodes.append(helper.make_node("Transpose", [f"k_{l}"], [f"kt_{l}"], perm=[0, 2, 1]))
        nodes.append(helper.make_node("MatMul", [f"q_{l}", f"kt_{l}"], [f"scores_{l}"]))

        scale = np.array(1.0 / np.sqrt(d_star), dtype=np.float32)
        initializers.append(numpy_helper.from_array(scale, name=f"scale_{l}"))
        nodes.append(helper.make_node("Mul", [f"scores_{l}", f"scale_{l}"], [f"scaled_{l}"]))
        nodes.append(helper.make_node("Softmax", [f"scaled_{l}"], [f"attn_{l}"], axis=-1))

        # Attention output: attn @ V
        nodes.append(helper.make_node("MatMul", [f"attn_{l}", f"v_{l}"], [f"attn_out_{l}"]))

        # Residual + LayerNorm (simplified as Add)
        nodes.append(helper.make_node("Add", [h_in, f"attn_out_{l}"], [f"res1_{l}"]))

        # MLP: W1 [d_star, d_ff], W2 [d_ff, d_star]
        W1 = (rng.randn(d_star, d_ff) * 0.02).astype(np.float32)
        W2 = (rng.randn(d_ff, d_star) * 0.02).astype(np.float32)
        initializers.append(numpy_helper.from_array(W1, name=f"W1_{l}"))
        initializers.append(numpy_helper.from_array(W2, name=f"W2_{l}"))

        nodes.append(helper.make_node("MatMul", [f"res1_{l}", f"W1_{l}"], [f"ff1_{l}"]))
        nodes.append(helper.make_node("Relu", [f"ff1_{l}"], [f"ff_act_{l}"]))
        nodes.append(helper.make_node("MatMul", [f"ff_act_{l}", f"W2_{l}"], [f"ff2_{l}"]))

        # Residual
        nodes.append(helper.make_node("Add", [f"res1_{l}", f"ff2_{l}"], [h_out]))

    # Output projection: [d_star, n_particles] — too large for full vocab
    # Use a smaller projection head for tractability
    vocab_proj_size = min(n_particles, 50000)  # cap output vocab
    W_out = (rng.randn(d_star, vocab_proj_size) * 0.02).astype(np.float32)
    initializers.append(numpy_helper.from_array(W_out, name="W_out"))

    nodes.append(helper.make_node("MatMul", [f"hidden_{L_star}", "W_out"], ["logits"]))

    output = helper.make_tensor_value_info("logits", TensorProto.FLOAT, [None, None, vocab_proj_size])

    graph = helper.make_graph(nodes, "bostrom_transformer",
                               inputs=[input_ids],
                               outputs=[output],
                               initializer=initializers)

    model = helper.make_model(graph, opset_imports=[helper.make_opsetid("", 18)])
    model.ir_version = 8

    onnx.save(model, out_path)
    size_mb = os.path.getsize(out_path) / 1024 / 1024

    print(f"  ONNX saved: {out_path} ({size_mb:.1f} MB)")
    print(f"  Layers: {L_star}, Vocab projection: {vocab_proj_size}")
    print(f"  Assembled in {time.time()-t0:.1f}s")
    return out_path


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 compile_model.py <cyberlinks.jsonl> [--max-links N] [--stakes <stakes.json>] [--onnx]")
        sys.exit(1)

    path = sys.argv[1]
    max_links = None
    stakes_path = None
    do_onnx = "--onnx" in sys.argv
    if "--max-links" in sys.argv:
        idx = sys.argv.index("--max-links")
        max_links = int(sys.argv[idx + 1])
    if "--stakes" in sys.argv:
        idx = sys.argv.index("--stakes")
        stakes_path = sys.argv[idx + 1]

    t_total = time.time()

    # Pipeline
    links, particles = load_cyberlinks(path, max_links)
    stakes = load_stakes(stakes_path) if stakes_path else None
    A = build_adjacency(links, particles, stakes)
    pi, lambda2, kappa, T_conv = compute_focus(A)

    E, d_star, sigma = compute_embeddings(A, pi)
    arch = estimate_architecture(d_star, lambda2, kappa, len(particles), len(links))

    # Save npz
    out_dir = os.path.dirname(path) or "."
    npz_path = os.path.join(out_dir, "bostrom_model.npz")

    idx_to_particle = {v: k for k, v in particles.items()}
    particle_list = [idx_to_particle[i] for i in range(len(particles))]

    np.savez_compressed(npz_path,
        embeddings=E,
        focus=pi,
        sigma=sigma,
        particle_cids=particle_list,
        **arch
    )

    # ONNX assembly
    onnx_path = None
    if do_onnx:
        onnx_path = os.path.join(out_dir, "bostrom_transformer.onnx")
        assemble_onnx(E, pi, arch, onnx_path)

    print(f"\n{'='*60}")
    print(f"Compilation complete in {time.time()-t_total:.1f}s")
    print(f"Saved to {npz_path}")
    if onnx_path:
        print(f"ONNX model: {onnx_path}")
    print(f"  Particles:  {len(particles):,}")
    print(f"  Links:      {len(links):,}")
    print(f"  Stake-weighted: {'yes' if stakes else 'no (uniform)'}")
    print(f"  d* = {d_star}, h* = {arch['h_star']}, L* = {arch['L_star']}")
    print(f"  Model size: {arch['size_gb']:.2f} GB ({arch['total_params']:,} params)")
    print(f"{'='*60}")


if __name__ == "__main__":
    main()
