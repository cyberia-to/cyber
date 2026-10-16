#!/usr/bin/env python3
"""Headless browser check over the Chrome DevTools Protocol — stdlib only.

Opens a URL in a throwaway headless Chromium-family browser (Chrome, Brave,
Chromium, Edge), waits for load, then reports the title, console errors and
uncaught exceptions, the result of each --eval expression and the match
count of each --selectors entry; --shot writes a PNG.

    python3 cdp.py http://127.0.0.1:8790/ --eval 'document.title' --shot /tmp/a.png
    python3 cdp.py URL --size 390x844 --mobile --selectors .deck .slide
    python3 cdp.py URL --eval 'await new Promise(r => setTimeout(r, 500)); return innerWidth'

Browser: $CDP_BROWSER or the first of Chrome / Brave / Chromium / Edge.
For plain DOM or a screenshot without scripting, the browser CLI is enough:
    "$CDP_BROWSER" --headless=new --dump-dom URL
    "$CDP_BROWSER" --headless=new --screenshot=out.png --window-size=1440,900 URL
Exit status: 0 clean, 1 page errors seen, 2 harness failure.
"""
import argparse
import base64
import json
import os
import shutil
import socket
import struct
import subprocess
import sys
import tempfile
import time
import urllib.parse
import urllib.request

CANDIDATES = [
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
    "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
    "google-chrome",
    "google-chrome-stable",
    "chromium",
    "chromium-browser",
    "brave-browser",
]


def find_browser():
    env = os.environ.get("CDP_BROWSER")
    if env:
        return env
    for c in CANDIDATES:
        if os.path.isabs(c) and os.access(c, os.X_OK):
            return c
        found = shutil.which(c)
        if found:
            return found
    sys.exit("cdp: no Chromium-family browser found; set CDP_BROWSER")


class WebSocket:
    """Minimal RFC 6455 client: text frames out (masked), any frames in."""

    def __init__(self, url, timeout=30.0):
        u = urllib.parse.urlparse(url)
        self.sock = socket.create_connection((u.hostname, u.port or 80), timeout=timeout)
        key = base64.b64encode(os.urandom(16)).decode()
        path = u.path + ("?" + u.query if u.query else "")
        req = (
            f"GET {path} HTTP/1.1\r\nHost: {u.hostname}:{u.port}\r\n"
            "Upgrade: websocket\r\nConnection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n\r\n"
        )
        self.sock.sendall(req.encode())
        head = b""
        while b"\r\n\r\n" not in head:
            chunk = self.sock.recv(4096)
            if not chunk:
                raise ConnectionError("websocket handshake closed")
            head += chunk
        status, _, self.buf = head.partition(b"\r\n\r\n")
        if b" 101 " not in status.split(b"\r\n", 1)[0]:
            raise ConnectionError(status.decode(errors="replace"))

    def _read(self, n):
        while len(self.buf) < n:
            chunk = self.sock.recv(max(65536, n - len(self.buf)))
            if not chunk:
                raise ConnectionError("websocket closed")
            self.buf += chunk
        out, self.buf = self.buf[:n], self.buf[n:]
        return out

    def _frame(self, opcode, payload):
        n = len(payload)
        if n < 126:
            head = struct.pack("!BB", 0x80 | opcode, 0x80 | n)
        elif n < 1 << 16:
            head = struct.pack("!BBH", 0x80 | opcode, 0x80 | 126, n)
        else:
            head = struct.pack("!BBQ", 0x80 | opcode, 0x80 | 127, n)
        mask = os.urandom(4)
        body = bytes(b ^ mask[i % 4] for i, b in enumerate(payload))
        self.sock.sendall(head + mask + body)

    def send(self, text):
        self._frame(0x1, text.encode())

    def recv(self):
        message = b""
        while True:
            b0, b1 = self._read(2)
            opcode, n = b0 & 0x0F, b1 & 0x7F
            if n == 126:
                n = struct.unpack("!H", self._read(2))[0]
            elif n == 127:
                n = struct.unpack("!Q", self._read(8))[0]
            mask = self._read(4) if b1 & 0x80 else None
            data = self._read(n)
            if mask:
                data = bytes(b ^ mask[i % 4] for i, b in enumerate(data))
            if opcode == 0x8:
                raise ConnectionError("websocket closed by peer")
            if opcode == 0x9:
                self._frame(0xA, data)
                continue
            if opcode == 0xA:
                continue
            message += data
            if b0 & 0x80:
                return message.decode()

    def close(self):
        try:
            self.sock.close()
        except OSError:
            pass


class Page:
    def __init__(self, ws_url):
        self.ws = WebSocket(ws_url)
        self.next_id = 0
        self.errors = []
        self.events = []

    def _dispatch(self, msg):
        method, p = msg.get("method"), msg.get("params", {})
        if method == "Runtime.exceptionThrown":
            d = p["exceptionDetails"]
            text = (d.get("exception") or {}).get("description") or d.get("text", "")
            self.errors.append("exception: " + text[:500])
        elif method == "Runtime.consoleAPICalled" and p.get("type") in ("error", "assert"):
            args = " ".join(str(a.get("value", a.get("description", ""))) for a in p.get("args", []))
            self.errors.append("console: " + args[:500])
        elif method == "Log.entryAdded" and p["entry"].get("level") == "error":
            e = p["entry"]
            self.errors.append(f"log: {e.get('text', '')[:400]} {e.get('url', '')}".strip())
        elif method:
            self.events.append(method)

    def call(self, method, params=None, timeout=60.0):
        self.next_id += 1
        mid = self.next_id
        self.ws.send(json.dumps({"id": mid, "method": method, "params": params or {}}))
        deadline = time.time() + timeout
        while time.time() < deadline:
            msg = json.loads(self.ws.recv())
            if msg.get("id") == mid:
                if "error" in msg:
                    raise RuntimeError(f"{method}: {msg['error']}")
                return msg.get("result", {})
            self._dispatch(msg)
        raise TimeoutError(method)

    def pump(self, seconds, until=None):
        end = time.time() + seconds
        while time.time() < end:
            self.ws.sock.settimeout(max(0.05, end - time.time()))
            try:
                self._dispatch(json.loads(self.ws.recv()))
            except (socket.timeout, TimeoutError):
                break
            if until and until in self.events:
                break
        self.ws.sock.settimeout(30.0)

    def evaluate(self, expression):
        # bodies with an explicit `return` run as an async function, so they
        # may hold several statements and `await`
        body = f"(async () => {{ {expression} }})()" if "return" in expression else expression
        r = self.call("Runtime.evaluate", {"expression": body, "returnByValue": True, "awaitPromise": True})
        if "exceptionDetails" in r:
            d = r["exceptionDetails"]
            return {"error": (d.get("exception") or {}).get("description") or d.get("text")}
        return r.get("result", {}).get("value")


def launch(browser, width, height):
    profile = tempfile.mkdtemp(prefix="cdp-")
    proc = subprocess.Popen(
        [browser, "--headless=new", "--remote-debugging-port=0", f"--user-data-dir={profile}",
         "--no-first-run", "--no-default-browser-check", "--disable-gpu",
         f"--window-size={width},{height}", "about:blank"],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )
    port_file = os.path.join(profile, "DevToolsActivePort")
    for _ in range(200):
        if os.path.exists(port_file):
            lines = open(port_file).read().split()
            if lines:
                return proc, profile, int(lines[0])
        if proc.poll() is not None:
            break
        time.sleep(0.05)
    proc.kill()
    shutil.rmtree(profile, ignore_errors=True)
    raise RuntimeError(f"browser did not open a debugging port: {browser}")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("url")
    ap.add_argument("--eval", action="append", default=[], metavar="JS", help="expression to evaluate (repeatable)")
    ap.add_argument("--selectors", nargs="*", default=[], help="CSS selectors to count")
    ap.add_argument("--shot", help="write a PNG screenshot here")
    ap.add_argument("--full", action="store_true", help="full-page screenshot")
    ap.add_argument("--size", default="1440x900", help="viewport WxH (default 1440x900)")
    ap.add_argument("--mobile", action="store_true", help="emulate a touch phone at --size")
    ap.add_argument("--wait", type=float, default=2.0, help="seconds to settle after load (default 2)")
    ap.add_argument("--browser", help="browser binary (default $CDP_BROWSER or autodetect)")
    a = ap.parse_args()

    width, height = (int(v) for v in a.size.lower().split("x"))
    proc, profile, port = launch(a.browser or find_browser(), width, height)
    try:
        req = urllib.request.Request(f"http://127.0.0.1:{port}/json/new?about:blank", method="PUT")
        target = json.load(urllib.request.urlopen(req, timeout=10))
        page = Page(target["webSocketDebuggerUrl"])
        for domain in ("Page", "Runtime", "Log"):
            page.call(f"{domain}.enable")
        page.call("Emulation.setDeviceMetricsOverride", {
            "width": width, "height": height, "deviceScaleFactor": 2 if a.mobile else 1, "mobile": a.mobile})
        if a.mobile:
            page.call("Emulation.setTouchEmulationEnabled", {"enabled": True, "maxTouchPoints": 5})
        nav = page.call("Page.navigate", {"url": a.url})
        if nav.get("errorText"):
            page.errors.append("navigate: " + nav["errorText"])
        page.pump(30.0, until="Page.loadEventFired")
        page.pump(a.wait)
        report = {"url": page.evaluate("location.href"), "title": page.evaluate("document.title")}
        if a.selectors:
            report["selectors"] = {s: page.evaluate(f"document.querySelectorAll({json.dumps(s)}).length")
                                   for s in a.selectors}
        if a.eval:
            report["eval"] = [{"expr": e, "value": page.evaluate(e)} for e in a.eval]
        if a.shot:
            shot = page.call("Page.captureScreenshot", {"format": "png", "captureBeyondViewport": a.full})
            with open(a.shot, "wb") as f:
                f.write(base64.b64decode(shot["data"]))
            report["shot"] = a.shot
        page.pump(0.2)
        report["errors"] = page.errors
        print(json.dumps(report, indent=1, ensure_ascii=False))
        page.ws.close()
        return 1 if page.errors else 0
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()
        shutil.rmtree(profile, ignore_errors=True)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, ConnectionError, TimeoutError, OSError) as e:
        print(f"cdp: {e}", file=sys.stderr)
        sys.exit(2)
