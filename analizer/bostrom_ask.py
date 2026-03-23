# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
bostrom_ask.py — ask the Bostrom knowledge graph via Ollama

Takes a text query, finds relevant CIDs via graph embeddings,
injects graph context into Ollama's bostrom model, returns answer.

Usage:
  python3 analizer/bostrom_ask.py "what is cyber?"
  python3 analizer/bostrom_ask.py "bitcoin"
  python3 analizer/bostrom_ask.py   # interactive mode
"""

import json
import os
import sys
import urllib.request
import numpy as np

DATA_DIR = os.path.expanduser("~/git/cyber/data")
OLLAMA_URL = "http://localhost:11434/api/generate"


def load_all():
    print("Loading model...", flush=True)
    data = np.load(os.path.join(DATA_DIR, "bostrom_model.npz"), allow_pickle=True)
    E = data["embeddings"]
    pi = data["focus"]
    cids = list(data["particle_cids"])

    norms = np.linalg.norm(E, axis=1, keepdims=True)
    norms[norms == 0] = 1
    E_norm = E / norms

    with open(os.path.join(DATA_DIR, "cid_index.json")) as f:
        index = json.load(f)

    # reverse index
    idx_to_text = {v["idx"]: k for k, v in index.items()}

    print(f"  {len(cids):,} particles, {len(index)} indexed")
    return E_norm, pi, cids, index, idx_to_text


STOPWORDS = {"what", "is", "the", "a", "an", "of", "in", "to", "for", "and", "or", "how", "why", "where", "who", "does", "do", "can", "about", "tell", "me"}

def search(query, index):
    q = query.lower().strip().rstrip("?!.")
    # exact match
    if q in index:
        return index[q]
    # exact substring
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


def neighbors(idx, E_norm, pi, cids, idx_to_text, k=10):
    q = E_norm[idx]
    sims = E_norm @ q
    top = np.argsort(-sims)[1:k+1]  # skip self
    results = []
    for n_idx in top:
        label = idx_to_text.get(int(n_idx))
        results.append({
            "cid": cids[n_idx],
            "sim": float(sims[n_idx]),
            "focus": float(pi[n_idx]),
            "label": label
        })
    return results


def build_context(query, E_norm, pi, cids, index, idx_to_text):
    match = search(query, index)
    if not match:
        return f"No particle found for '{query}'. Try: " + ", ".join(list(index.keys())[:20])

    nbrs = neighbors(match["idx"], E_norm, pi, cids, idx_to_text, k=15)

    lines = [f"Query particle: {match.get('cid', '?')}"]
    # find text label
    for k, v in index.items():
        if v["idx"] == match["idx"]:
            lines.insert(0, f"Matched: '{k}'")
            break
    lines.append(f"Focus: {match['focus']:.6f}")
    lines.append("")
    lines.append("Top 15 graph neighbors by embedding similarity:")
    for n in nbrs:
        label = n["label"] or n["cid"][:40]
        lines.append(f"  sim={n['sim']:.3f} focus={n['focus']:.6f} | {label} ({n['cid'][:20]}...)")

    return "\n".join(lines)


def ask_ollama(query, context):
    prompt = f"""Graph context for your answer:
---
{context}
---

User question: {query}

Answer using the CID particles above. Reference specific CIDs. Explain what the graph structure reveals."""

    body = json.dumps({
        "model": "bostrom",
        "prompt": prompt,
        "stream": True
    }).encode()

    req = urllib.request.Request(OLLAMA_URL, data=body,
                                 headers={"Content-Type": "application/json"})

    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            for line in resp:
                chunk = json.loads(line)
                if "response" in chunk:
                    print(chunk["response"], end="", flush=True)
                if chunk.get("done"):
                    print()
                    return
    except Exception as e:
        print(f"\nOllama error: {e}")
        print("Is Ollama running? Try: ollama serve")


def main():
    E_norm, pi, cids, index, idx_to_text = load_all()

    if len(sys.argv) > 1:
        query = " ".join(sys.argv[1:])
        context = build_context(query, E_norm, pi, cids, index, idx_to_text)
        print(f"\n--- graph context ---\n{context}\n--- end context ---\n")
        ask_ollama(query, context)
        return

    # interactive mode
    print("\nBostrom Knowledge Graph — interactive mode")
    print("Type a question, get CID-based answers. Ctrl+C to exit.\n")

    while True:
        try:
            query = input("🔵 > ").strip()
            if not query:
                continue
            context = build_context(query, E_norm, pi, cids, index, idx_to_text)
            print(f"\n{context}\n")
            ask_ollama(query, context)
            print()
        except (KeyboardInterrupt, EOFError):
            print("\nDon't trust. Don't fear. Don't beg.")
            break


if __name__ == "__main__":
    main()
