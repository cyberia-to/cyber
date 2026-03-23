# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
bostrom_graph.py — pure graph intelligence, no LLM

Three reasoning modes combined:
  1. Embedding retrieval — cosine similarity in 26-dim SVD space
  2. Graph walk — actual cyberlink traversal weighted by focus
  3. Spectral reasoning — project through singular vectors to find
     deep structural roles (hub, bridge, leaf, cluster center)

Input: text or CID
Output: ranked CID sequences — the graph's own answer

Usage:
  python3 analizer/bostrom_graph.py "dog"           # single query
  python3 analizer/bostrom_graph.py                  # interactive
  python3 analizer/bostrom_graph.py --resolve        # resolve CIDs via IPFS
"""

import json
import os
import sys
import time
import numpy as np
from scipy.sparse import csr_matrix
from collections import defaultdict

from analizer.bostrom_lib import (
    DATA_DIR, search, label as lib_label, embedding_neighbors as lib_embedding_neighbors,
)


class BostromGraph:
    def __init__(self, resolve=False):
        t0 = time.time()
        print("Loading graph...", flush=True)

        data = np.load(os.path.join(DATA_DIR, "bostrom_model.npz"), allow_pickle=True)
        self.E = data["embeddings"]          # [N, 26] SVD embeddings
        self.pi = data["focus"]              # [N] PageRank
        self.sigma = data["sigma"]           # [100] singular values
        self.cids = list(data["particle_cids"])
        self.d_star = int(data["d_star"])
        self.N = len(self.cids)

        # normalize embeddings
        norms = np.linalg.norm(self.E, axis=1, keepdims=True)
        norms[norms == 0] = 1
        self.E_norm = self.E / norms

        # CID lookup
        self.cid_to_idx = {c: i for i, c in enumerate(self.cids)}

        # load adjacency for graph walks
        self.A = None
        self._load_adjacency()

        # load text index
        idx_path = os.path.join(DATA_DIR, "cid_index.json")
        if os.path.exists(idx_path):
            with open(idx_path) as f:
                self.index = json.load(f)
        else:
            self.index = {}

        # reverse index
        self.idx_to_text = {v["idx"]: k for k, v in self.index.items()}
        self.resolve = resolve
        print(f"  {self.N:,} particles, {len(self.index)} labeled, d*={self.d_star}")
        print(f"  Loaded in {time.time()-t0:.1f}s\n")

    def _load_adjacency(self):
        links_path = os.path.join(DATA_DIR, "cyberlinks.jsonl")
        if not os.path.exists(links_path):
            print("  No cyberlinks.jsonl — graph walk disabled")
            return

        print("  Loading adjacency...", flush=True)
        rows, cols, vals = [], [], []
        edge_count = 0
        with open(links_path) as f:
            for line in f:
                r = json.loads(line)
                i = self.cid_to_idx.get(r["particle_from"])
                j = self.cid_to_idx.get(r["particle_to"])
                if i is not None and j is not None:
                    rows.append(i)
                    cols.append(j)
                    vals.append(1.0)
                    edge_count += 1

        self.A = csr_matrix((vals, (rows, cols)), shape=(self.N, self.N))
        print(f"  Adjacency: {edge_count:,} edges")

    def label(self, idx):
        """Get human-readable label for particle index"""
        return lib_label(idx, self.idx_to_text, self.cids)

    def find(self, query):
        """Text -> particle index"""
        match = search(query, self.index)
        if match is not None:
            return match["idx"]
        return None

    # === MODE 1: Embedding retrieval ===
    def embedding_neighbors(self, idx, k=20):
        """Cosine similarity in SVD space"""
        return lib_embedding_neighbors(idx, self.E_norm, self.pi, k)

    # === MODE 2: Graph walk ===
    def graph_walk(self, idx, steps=50, walks=100):
        """Focus-weighted random walk from particle. Returns visit frequency."""
        if self.A is None:
            return []

        visit_count = defaultdict(int)
        rng = np.random.RandomState(idx)  # deterministic per particle

        for _ in range(walks):
            current = idx
            for _ in range(steps):
                # get outgoing edges
                row = self.A.getrow(current)
                neighbors = row.indices
                if len(neighbors) == 0:
                    # teleport to random high-focus particle
                    current = rng.choice(np.argsort(-self.pi)[:100])
                    continue

                # weight by neighbor focus
                weights = self.pi[neighbors]
                total = weights.sum()
                if total > 0:
                    weights = weights / total
                else:
                    weights = np.ones(len(neighbors)) / len(neighbors)

                current = rng.choice(neighbors, p=weights)
                visit_count[current] += 1

        # normalize and sort
        total_visits = sum(visit_count.values())
        results = [(idx, count / total_visits, float(self.pi[idx]))
                    for idx, count in visit_count.items()
                    if idx != idx]  # exclude self
        results.sort(key=lambda x: -x[1])
        return results[:20]

    # === MODE 3: Spectral role analysis ===
    def spectral_role(self, idx):
        """Analyze particle's role via its embedding vector.

        Each dimension of the embedding corresponds to a singular vector.
        The magnitude in each dimension reveals structural role:
        - Large in dim 0 (dominant SV): hub/authority
        - Large in dim 1-2: bridge between major clusters
        - Uniform across dims: well-connected generalist
        - Sparse (few large dims): specialist
        """
        vec = self.E[idx]
        abs_vec = np.abs(vec)
        total = abs_vec.sum() + 1e-10

        # concentration: how many dimensions dominate?
        sorted_abs = np.sort(abs_vec)[::-1]
        cumulative = np.cumsum(sorted_abs) / total
        effective_dims = np.searchsorted(cumulative, 0.9) + 1

        # role classification
        dim0_share = abs_vec[0] / total if total > 0 else 0
        max_dim = np.argmax(abs_vec)
        focus_rank = int(np.searchsorted(np.sort(-self.pi), -self.pi[idx]))

        return {
            "effective_dims": int(effective_dims),
            "dominant_dim": int(max_dim),
            "dim0_share": float(dim0_share),
            "focus": float(self.pi[idx]),
            "focus_rank": focus_rank,
            "embedding_norm": float(np.linalg.norm(vec)),
            "role": self._classify_role(effective_dims, dim0_share, float(self.pi[idx]))
        }

    def _classify_role(self, eff_dims, dim0_share, focus):
        if focus > 0.001:
            return "HUB" if dim0_share > 0.3 else "AUTHORITY"
        if eff_dims <= 3:
            return "SPECIALIST"
        if eff_dims >= self.d_star * 0.7:
            return "BRIDGE"
        return "MEMBER"

    # === COMBINED: Full graph reasoning ===
    def reason(self, query):
        """Combined multi-mode reasoning about a query"""
        idx = self.find(query)
        if idx is None:
            return f"'{query}' not found in graph. Known: {', '.join(list(self.index.keys())[:15])}..."

        lines = []
        name = self.label(idx)
        cid = self.cids[idx]

        # Header
        lines.append(f"=== {name} ===")
        lines.append(f"CID: {cid}")
        lines.append("")

        # Spectral role
        role = self.spectral_role(idx)
        lines.append(f"role: {role['role']}")
        lines.append(f"focus: {role['focus']:.6f} (rank #{role['focus_rank']:,} of {self.N:,})")
        lines.append(f"embedding norm: {role['embedding_norm']:.4f}")
        lines.append(f"effective dimensions: {role['effective_dims']}/{self.d_star}")
        lines.append(f"dominant dimension: {role['dominant_dim']} ({role['dim0_share']:.1%} of energy)")
        lines.append("")

        # Embedding neighbors (structural similarity)
        lines.append("-- structural neighbors (embedding cosine) --")
        emb_nbrs = self.embedding_neighbors(idx, k=15)
        for i, (n_idx, sim, focus) in enumerate(emb_nbrs):
            lbl = self.label(n_idx)
            marker = "+" if focus > 0.0001 else "o"
            lines.append(f"  {marker} {sim:.3f}  {lbl}")

        # Graph walk (actual cyberlink paths)
        if self.A is not None:
            lines.append("")
            lines.append("-- graph walk (focus-weighted, 100 walks x 50 steps) --")
            walk_results = self.graph_walk(idx, steps=50, walks=100)
            if walk_results:
                for n_idx, visit_freq, focus in walk_results[:15]:
                    lbl = self.label(n_idx)
                    bar = "#" * int(visit_freq * 100)
                    lines.append(f"  {visit_freq:.3f} {bar} {lbl}")
            else:
                lines.append("  (no outgoing links — isolated particle)")

        # Cross-reference: what appears in BOTH embedding neighbors AND walk?
        emb_set = {n_idx for n_idx, _, _ in emb_nbrs}
        walk_set = {n_idx for n_idx, _, _ in (self.graph_walk(idx, 50, 100) if self.A is not None else [])}
        overlap = emb_set & walk_set
        if overlap:
            lines.append("")
            lines.append("-- confirmed (both structural + walk) --")
            for n_idx in overlap:
                lbl = self.label(n_idx)
                lines.append(f"  * {lbl}")

        # Focus neighborhood: highest-focus particles nearby
        lines.append("")
        lines.append("-- focus gravity (high-focus particles in neighborhood) --")
        focus_weighted = [(n_idx, sim * self.pi[n_idx], self.pi[n_idx])
                          for n_idx, sim, _ in emb_nbrs if self.pi[n_idx] > 0.00001]
        focus_weighted.sort(key=lambda x: -x[1])
        for n_idx, gravity, focus in focus_weighted[:10]:
            lbl = self.label(n_idx)
            lines.append(f"  gravity={gravity:.6f} focus={focus:.6f} {lbl}")

        if not focus_weighted:
            lines.append("  (no high-focus neighbors — peripheral particle)")

        return "\n".join(lines)


def main():
    resolve = "--resolve" in sys.argv
    args = [a for a in sys.argv[1:] if not a.startswith("--")]

    g = BostromGraph(resolve=resolve)

    if args:
        query = " ".join(args)
        print(g.reason(query))
        return

    # interactive
    print("Bostrom Graph Intelligence — pure graph, no LLM")
    print("Type a concept. Get graph structure. Ctrl+C to exit.\n")

    while True:
        try:
            query = input("@ > ").strip()
            if not query:
                continue
            print()
            print(g.reason(query))
            print()
        except (KeyboardInterrupt, EOFError):
            print("\nDon't trust. Don't fear. Don't beg.")
            break


if __name__ == "__main__":
    main()
