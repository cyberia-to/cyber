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
import sys
import urllib.request

from analizer.bostrom_lib import load_model, search, label, embedding_neighbors

OLLAMA_URL = "http://localhost:11434/api/generate"


def build_context(query, E_norm, pi, cids, index, idx_to_text):
    match = search(query, index)
    if not match:
        return f"No particle found for '{query}'. Try: " + ", ".join(list(index.keys())[:20])

    nbrs = embedding_neighbors(match["idx"], E_norm, pi, k=15)

    lines = [f"Query particle: {match.get('cid', '?')}"]
    # find text label
    for k, v in index.items():
        if v["idx"] == match["idx"]:
            lines.insert(0, f"Matched: '{k}'")
            break
    lines.append(f"Focus: {match['focus']:.6f}")
    lines.append("")
    lines.append("Top 15 graph neighbors by embedding similarity:")
    for n_idx, sim, focus in nbrs:
        lbl = label(n_idx, idx_to_text, cids)
        cid = cids[n_idx]
        lines.append(f"  sim={sim:.3f} focus={focus:.6f} | {lbl} ({cid[:20]}...)")

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
    E_norm, pi, cids, index, idx_to_text = load_model()

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
            query = input("\U0001f535 > ").strip()
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
