# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
bostrom_serve.py — serve compiled Bostrom model as OpenAI-compatible API

Receives text queries, resolves to CIDs via graph embeddings,
returns CID-based responses. Compatible with Ollama's API format.

Usage:
  python3 analizer/bostrom_serve.py [--port 11435] [--build-index]

First run with --build-index to resolve top CIDs via IPFS gateway.
Subsequent runs load cached index from data/cid_index.json.
"""

import json
import os
import sys
import ssl
import urllib.request
import numpy as np
from http.server import HTTPServer, BaseHTTPRequestHandler

from analizer.bostrom_lib import (
    DATA_DIR, load_model, search, label, embedding_neighbors,
)

INDEX_PATH = os.path.join(DATA_DIR, "cid_index.json")

# SSL context for IPFS gateway
CTX = ssl.create_default_context()
CTX.check_hostname = False
CTX.verify_mode = ssl.CERT_NONE


def resolve_cid(cid):
    try:
        url = f"https://gateway.ipfs.cybernode.ai/ipfs/{cid}"
        req = urllib.request.Request(url)
        with urllib.request.urlopen(req, timeout=5, context=CTX) as resp:
            content = resp.read(500).decode("utf-8", errors="replace").strip()
            if content.startswith(("\x89PNG", "\xff\xd8", "{", "[")):
                return None  # skip binary/json
            return content[:200]
    except:
        return None


def build_index(cids, pi, top_n=5000):
    """Resolve top CIDs and build text->index mapping"""
    print(f"Building index from top {top_n} particles...")
    top_idx = np.argsort(-pi)[:top_n]
    index = {}  # text -> {"idx": int, "cid": str, "focus": float}

    for i, idx in enumerate(top_idx):
        content = resolve_cid(cids[idx])
        if content and len(content) < 200:
            key = content.lower().strip()
            index[key] = {"idx": int(idx), "cid": cids[idx], "focus": float(pi[idx])}
            if (i + 1) % 100 == 0:
                print(f"  {i+1}/{top_n} resolved, {len(index)} indexed")

    with open(INDEX_PATH, "w") as f:
        json.dump(index, f)
    print(f"Index saved: {len(index)} text->CID mappings -> {INDEX_PATH}")
    return index


def generate_response(query, E_norm, pi, cids, index, idx_to_text):
    """Process a text query and return CID-based response"""
    match = search(query, index)

    if not match:
        # try each word
        words = query.lower().split()
        for word in words:
            match = search(word, index)
            if match:
                break

    if not match:
        return f"no particle found for '{query}'. the graph has {len(cids):,} particles but only {len(index):,} are text-indexed."

    neighbors = embedding_neighbors(match["idx"], E_norm, pi, k=10)

    lines = [f"particle: {match.get('cid', '?')}"]
    lines.append(f"focus: {match['focus']:.6f}")
    lines.append("")
    lines.append("graph neighbors:")
    for n_idx, sim, focus in neighbors:
        lbl = label(n_idx, idx_to_text, cids)
        lines.append(f"  sim={sim:.3f} focus={focus:.6f} -> {lbl}")
        lines.append(f"    cid: {cids[n_idx]}")

    return "\n".join(lines)


class BostromHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path in ("/api/generate", "/v1/chat/completions", "/api/chat"):
            length = int(self.headers.get("Content-Length", 0))
            body = json.loads(self.rfile.read(length))

            # extract query from various formats
            if "prompt" in body:
                query = body["prompt"]
            elif "messages" in body:
                query = body["messages"][-1].get("content", "")
            else:
                query = str(body)

            response = generate_response(query, self.server.E_norm, self.server.pi,
                                          self.server.cids, self.server.index,
                                          self.server.idx_to_text)

            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.end_headers()

            if self.path == "/api/generate":
                # Ollama format
                result = {"model": "bostrom", "response": response, "done": True}
            else:
                # OpenAI format
                result = {
                    "choices": [{"message": {"role": "assistant", "content": response}}],
                    "model": "bostrom"
                }
            self.wfile.write(json.dumps(result).encode())
        else:
            self.send_response(404)
            self.end_headers()

    def log_message(self, format, *args):
        query = args[0] if args else ""
        if "POST" in str(query):
            print(f"  -> {query}")


def main():
    port = 11435
    do_build = "--build-index" in sys.argv
    if "--port" in sys.argv:
        port = int(sys.argv[sys.argv.index("--port") + 1])

    E_norm, pi, cids, index, idx_to_text = load_model()

    if do_build:
        index = build_index(cids, pi, top_n=5000)
        idx_to_text = {v["idx"]: k for k, v in index.items()}

    if not index:
        print("No index found. Run with --build-index first.")
        print("  python3 analizer/bostrom_serve.py --build-index")
        sys.exit(1)

    print(f"Index: {len(index)} text->CID mappings")

    server = HTTPServer(("localhost", port), BostromHandler)
    server.E_norm = E_norm
    server.pi = pi
    server.cids = cids
    server.index = index
    server.idx_to_text = idx_to_text

    print(f"\nBostrom model serving on http://localhost:{port}")
    print(f"  curl -X POST http://localhost:{port}/api/generate -d '{{\"prompt\": \"cyber\"}}'")
    print(f"  Compatible with Ollama API format.\n")

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down.")


if __name__ == "__main__":
    main()
