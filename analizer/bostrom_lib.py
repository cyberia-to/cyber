# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
bostrom_lib.py — shared module for Bostrom graph scripts

Extracted from bostrom_ask.py, bostrom_serve.py, bostrom_graph.py.
Provides model loading, text search, label resolution, and embedding neighbors.
"""

import json
import os
import numpy as np

DATA_DIR = os.path.expanduser("~/git/cyber/data")

STOPWORDS = {
    "what", "is", "the", "a", "an", "of", "in", "to", "for",
    "and", "or", "how", "why", "where", "who", "does", "do",
    "can", "about", "tell", "me",
}


def load_model():
    """Load compiled Bostrom model.

    Returns (E_norm, pi, cids, index, idx_to_text):
        E_norm     — L2-normalized embeddings  [N, d]
        pi         — focus (PageRank) vector    [N]
        cids       — list of CID strings        [N]
        index      — text→{"idx","cid","focus"} dict
        idx_to_text — particle index → text label dict
    """
    print("Loading model...", flush=True)
    data = np.load(os.path.join(DATA_DIR, "bostrom_model.npz"), allow_pickle=True)
    E = data["embeddings"]
    pi = data["focus"]
    cids = list(data["particle_cids"])

    norms = np.linalg.norm(E, axis=1, keepdims=True)
    norms[norms == 0] = 1
    E_norm = E / norms

    idx_path = os.path.join(DATA_DIR, "cid_index.json")
    if os.path.exists(idx_path):
        with open(idx_path) as f:
            index = json.load(f)
    else:
        index = {}

    idx_to_text = {v["idx"]: k for k, v in index.items()}

    print(f"  {len(cids):,} particles, {len(index)} indexed")
    return E_norm, pi, cids, index, idx_to_text


def search(query, index):
    """Find best matching particle by text query.

    Returns the index entry dict {"idx", "cid", "focus"} or None.
    Tries exact match, substring match, then per-word with stopword filter.
    """
    q = query.lower().strip().rstrip("?!.")
    # exact match
    if q in index:
        return index[q]
    # substring match
    for k, v in index.items():
        if q in k:
            return v
    # try meaningful words (skip stopwords), prefer longer matches
    words = [w for w in q.split() if w not in STOPWORDS and len(w) > 2]
    for word in sorted(words, key=len, reverse=True):
        if word in index:
            return index[word]
        for k, v in index.items():
            if word in k:
                return v
    return None


def label(idx, idx_to_text, cids):
    """Human-readable label for a particle index."""
    text = idx_to_text.get(idx)
    if text:
        return text
    return cids[idx][:16] + "..."


def embedding_neighbors(idx, E_norm, pi, k=10):
    """Find k nearest neighbors by cosine similarity in SVD space.

    Returns list of (neighbor_idx, similarity, focus).
    """
    q = E_norm[idx]
    sims = E_norm @ q
    top = np.argsort(-sims)[1:k + 1]  # skip self
    return [(int(i), float(sims[i]), float(pi[i])) for i in top]
