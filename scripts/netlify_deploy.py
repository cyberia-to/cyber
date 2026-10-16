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
    """Netlify API client shared by the upload threads.

    Requests are paced (one start per `interval` seconds across all threads)
    and a 429 pauses every thread: the pause doubles on consecutive 429s up
    to two minutes and resets after a success. 429s retry until the deploy
    deadline; 5xx and network errors retry `attempts` times.
    """

    def __init__(self, token, deadline, interval=0.15):
        self.token = token
        self.deadline = deadline
        self.interval = interval
        self.lock = threading.Lock()
        self.next_start = 0.0
        self.pause_until = 0.0
        self.backoff = 2.0
        self.limited = 0
        self.shown_headers = False

    def _slot(self):
        with self.lock:
            now = time.time()
            start = max(now, self.next_start, self.pause_until)
            self.next_start = start + self.interval
        if start > now:
            time.sleep(start - now)

    def _rate_limited(self, headers):
        delay = None
        retry = headers.get("Retry-After")
        reset = headers.get("X-RateLimit-Reset")
        try:
            if retry:
                delay = float(retry)
            elif reset:
                r = float(reset)
                delay = r - time.time() if r > 1e9 else r
        except ValueError:
            pass
        with self.lock:
            self.limited += 1
            if not self.shown_headers:
                self.shown_headers = True
                rl = {k: v for k, v in headers.items() if "rate" in k.lower() or k.lower() == "retry-after"}
                print(f"  rate limited (headers: {rl or 'none'})", file=sys.stderr)
            if time.time() >= self.pause_until:
                wait = delay if delay and delay > 0 else self.backoff
                self.backoff = min(120.0, self.backoff * 2)
                self.pause_until = time.time() + wait + 0.5
                print(f"  429: all requests pause {wait:.0f}s", file=sys.stderr)

    def request(self, method, path, body=None, content_type="application/json", attempts=8):
        url = path if path.startswith("http") else API + path
        data = json.dumps(body).encode() if content_type == "application/json" and body is not None else body
        failures = 0
        while True:
            if time.time() > self.deadline:
                raise RuntimeError(f"{method} {path}: deadline passed")
            self._slot()
            req = urllib.request.Request(url, data=data, method=method, headers={
                "Authorization": f"Bearer {self.token}",
                "Content-Type": content_type,
                "User-Agent": "cyber-netlify-deploy/1",
            })
            try:
                with urllib.request.urlopen(req, timeout=300) as r:
                    raw = r.read()
                with self.lock:
                    self.backoff = 2.0
                return json.loads(raw) if raw else {}
            except urllib.error.HTTPError as e:
                detail = e.read()[:300].decode(errors="replace")
                if e.code == 429:
                    self._rate_limited(e.headers)
                    continue
                if e.code < 500:
                    raise RuntimeError(f"{method} {path}: HTTP {e.code} {detail}") from None
                reason = f"HTTP {e.code}"
            except (urllib.error.URLError, TimeoutError, ConnectionError) as e:
                reason = str(e)
            failures += 1
            if failures >= attempts:
                raise RuntimeError(f"{method} {path}: gave up after {attempts} attempts ({reason})")
            delay = min(60.0, 2.0 ** failures)
            print(f"  {method} {path[:80]}: {reason}, retry in {delay:.0f}s", file=sys.stderr)
            time.sleep(delay)


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
    ap.add_argument("--jobs", type=int, default=6, help="parallel uploads")
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
    deadline = started + a.timeout
    client = Client(token, deadline)

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
    print(f"ready: {url}  ({time.time() - started:.0f}s, {client.limited} rate-limited requests retried)")
    if not a.draft:
        print(f"production: {deploy.get('ssl_url') or deploy.get('url') or '(site url)'}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except RuntimeError as e:
        print(f"netlify_deploy: {e}", file=sys.stderr)
        sys.exit(1)
