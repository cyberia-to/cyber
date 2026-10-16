#!/usr/bin/env python3
"""File-digest deploy of a static directory to Netlify — stdlib only.

Sends the SHA-1 of every file, uploads only the files Netlify does not
already hold, then waits until the deploy is ready. Same protocol the
Netlify CLI uses, without Node.

    NETLIFY_AUTH_TOKEN=… NETLIFY_SITE_ID=… \\
      python3 scripts/netlify_deploy.py build --message "cyber abc12345" [--draft]

--draft publishes a preview deploy and leaves production untouched.
--dry-run hashes the directory and prints the manifest size only.
"""
import argparse
import concurrent.futures
import hashlib
import json
import os
import sys
import threading
import time
import urllib.error
import urllib.parse
import urllib.request

API = "https://api.netlify.com/api/v1"
SAFE = "/;,:@&=+$-_.!~*'()"


def manifest(root):
    files, skipped = {}, []
    for dirpath, dirnames, filenames in os.walk(root):
        rel_dir = os.path.relpath(dirpath, root)
        parts = [] if rel_dir == "." else rel_dir.split(os.sep)
        dirnames[:] = sorted(d for d in dirnames if not d.startswith(".") or d == ".well-known")
        for name in sorted(filenames):
            if name.startswith(".") and ".well-known" not in parts:
                continue
            rel = "/".join(parts + [name])
            if "#" in rel or "?" in rel:
                skipped.append(rel)
                continue
            h = hashlib.sha1()
            with open(os.path.join(dirpath, name), "rb") as f:
                for chunk in iter(lambda: f.read(1 << 20), b""):
                    h.update(chunk)
            files["/" + rel] = h.hexdigest()
    return files, skipped


class Client:
    def __init__(self, token):
        self.token = token
        self.lock = threading.Lock()
        self.pause_until = 0.0

    def request(self, method, path, body=None, content_type="application/json", attempts=8):
        url = path if path.startswith("http") else API + path
        data = json.dumps(body).encode() if content_type == "application/json" and body is not None else body
        for attempt in range(attempts):
            with self.lock:
                wait = self.pause_until - time.time()
            if wait > 0:
                time.sleep(wait)
            req = urllib.request.Request(url, data=data, method=method, headers={
                "Authorization": f"Bearer {self.token}",
                "Content-Type": content_type,
                "User-Agent": "cyber-netlify-deploy/1",
            })
            try:
                with urllib.request.urlopen(req, timeout=300) as r:
                    raw = r.read()
                    return json.loads(raw) if raw else {}
            except urllib.error.HTTPError as e:
                detail = e.read()[:300].decode(errors="replace")
                if e.code == 429 or e.code >= 500:
                    reset = e.headers.get("X-RateLimit-Reset")
                    retry = e.headers.get("Retry-After")
                    delay = float(retry) if retry and retry.isdigit() else min(60.0, 2.0 ** attempt)
                    if reset and reset.isdigit():
                        delay = max(delay, int(reset) - time.time() + 1)
                    with self.lock:
                        self.pause_until = max(self.pause_until, time.time() + delay)
                    print(f"  {method} {path[:80]}: HTTP {e.code}, retry in {delay:.0f}s", file=sys.stderr)
                    continue
                raise RuntimeError(f"{method} {path}: HTTP {e.code} {detail}") from None
            except (urllib.error.URLError, TimeoutError, ConnectionError) as e:
                delay = min(60.0, 2.0 ** attempt)
                print(f"  {method} {path[:80]}: {e}, retry in {delay:.0f}s", file=sys.stderr)
                time.sleep(delay)
        raise RuntimeError(f"{method} {path}: gave up after {attempts} attempts")


def wait_state(client, deploy_id, states, deadline):
    while True:
        d = client.request("GET", f"/deploys/{deploy_id}")
        if d.get("state") == "error":
            raise RuntimeError(f"deploy {deploy_id} failed: {d.get('error_message')}")
        if d.get("state") in states:
            return d
        if time.time() > deadline:
            raise RuntimeError(f"deploy {deploy_id} stuck in state {d.get('state')}")
        time.sleep(3)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("dir")
    ap.add_argument("--message", default="")
    ap.add_argument("--draft", action="store_true", help="preview deploy, production untouched")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--timeout", type=int, default=1800, help="seconds for the whole deploy")
    ap.add_argument("--jobs", type=int, default=12, help="parallel uploads")
    a = ap.parse_args()

    started = time.time()
    files, skipped = manifest(a.dir)
    for s in skipped:
        print(f"skip (# or ? in path): {s}", file=sys.stderr)
    print(f"files: {len(files)}  distinct digests: {len(set(files.values()))}")
    if a.dry_run:
        return 0

    token, site = os.environ.get("NETLIFY_AUTH_TOKEN"), os.environ.get("NETLIFY_SITE_ID")
    if not token or not site:
        sys.exit("NETLIFY_AUTH_TOKEN and NETLIFY_SITE_ID are required")
    client = Client(token)
    deadline = started + a.timeout

    body = {"files": files, "async": True, "draft": a.draft}
    if a.message:
        body["title"] = a.message
    deploy = client.request("POST", f"/sites/{site}/deploys", body)
    deploy_id = deploy["id"]
    print(f"deploy {deploy_id} created ({'draft' if a.draft else 'production'})")
    deploy = wait_state(client, deploy_id, {"prepared", "uploading", "uploaded", "ready"}, deadline)

    by_sha = {}
    for path, sha in files.items():
        by_sha.setdefault(sha, []).append(path)
    uploads = [p for sha in deploy.get("required") or [] for p in by_sha.get(sha, [])]
    print(f"upload: {len(uploads)} files (Netlify already holds the rest)")

    def put(path):
        with open(os.path.join(a.dir, *path.lstrip("/").split("/")), "rb") as f:
            payload = f.read()
        quoted = urllib.parse.quote(path.lstrip("/"), safe=SAFE)
        client.request("PUT", f"/deploys/{deploy_id}/files/{quoted}", payload, "application/octet-stream")

    done = 0
    with concurrent.futures.ThreadPoolExecutor(max_workers=a.jobs) as pool:
        for fut in concurrent.futures.as_completed([pool.submit(put, p) for p in uploads]):
            fut.result()
            done += 1
            if done % 500 == 0 or done == len(uploads):
                print(f"  uploaded {done}/{len(uploads)}")

    deploy = wait_state(client, deploy_id, {"ready"}, deadline)
    url = deploy.get("deploy_ssl_url") or deploy.get("deploy_url")
    print(f"ready: {url}  ({time.time() - started:.0f}s)")
    if not a.draft:
        print(f"production: {deploy.get('ssl_url') or deploy.get('url')}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except RuntimeError as e:
        print(f"netlify_deploy: {e}", file=sys.stderr)
        sys.exit(1)
