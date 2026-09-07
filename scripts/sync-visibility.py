#!/usr/bin/env python3
"""sync-visibility — GitHub is the source of truth for subgraph visibility.

Reads cyber/subgraphs.toml, queries github for every declared repo
(org cyberia-to; the optional `repo =` field overrides the name), and
rewrites `visibility` and `archived` to match reality. Line-based edit:
comments and formatting survive. Declarations marked `local-only = true`
are skipped. If anything changed, commits and pushes.
"""
import re, subprocess, sys, json, datetime

TOML = "/Users/master/cyber/cyber/subgraphs.toml"
ORG = "cyberia-to"

def gh(repo):
    r = subprocess.run(["gh","repo","view",f"{ORG}/{repo}","--json","visibility,isArchived"],
                       capture_output=True, text=True, timeout=30)
    if r.returncode != 0: return None
    return json.loads(r.stdout)

src = open(TOML).read()
blocks = src.split("[[subgraph]]")
head, blocks = blocks[0], blocks[1:]
changed = []
warn = []
out = [head]
for b in blocks:
    def field(k):
        m = re.search(rf'^{k}\s*=\s*"?([^"\n]+)"?\s*$', b, re.M)
        return m.group(1).strip() if m else None
    name = field("name")
    if not name:
        out.append(b); continue
    if field("local-only") == "true":
        out.append(b); continue
    repo = field("repo") or name
    info = gh(repo)
    if info is None:
        warn.append(f"{name}: no repo {ORG}/{repo} on github (local-only?)")
        out.append(b); continue
    want_vis = info["visibility"].lower()          # public / private
    want_arch = info["isArchived"]
    nb = b
    cur_vis = field("visibility")
    if cur_vis != want_vis:
        if cur_vis is not None:
            nb = re.sub(r'^visibility\s*=.*$', f'visibility = "{want_vis}"', nb, count=1, flags=re.M)
        else:
            nb = nb.replace(f'name = "{name}"', f'name = "{name}"\nvisibility = "{want_vis}"', 1)
        changed.append(f"{name}: visibility {cur_vis} -> {want_vis}")
    cur_arch = field("archived") == "true"
    if cur_arch != want_arch:
        if want_arch:
            nb = nb.rstrip("\n") + "\narchived = true\n\n" if not re.search(r'^archived\s*=', nb, re.M) \
                 else re.sub(r'^archived\s*=.*$', 'archived = true', nb, count=1, flags=re.M)
        else:
            nb = re.sub(r'^archived\s*=.*\n?', '', nb, flags=re.M)
        changed.append(f"{name}: archived {cur_arch} -> {want_arch}")
    out.append(nb)

new = "[[subgraph]]".join(out)
stamp = datetime.datetime.now().isoformat(timespec="seconds")
for w in warn: print("warn:", w)
if new != src:
    open(TOML, "w").write(new)
    print(f"[{stamp}] synced:", "; ".join(changed))
    subprocess.run(["git","-C","/Users/master/cyber/cyber","add","subgraphs.toml"], check=True)
    subprocess.run(["git","-C","/Users/master/cyber/cyber","commit","-q","-m",
        "subgraphs: visibility synced from github\n\n" + "\n".join(changed)], check=True)
    subprocess.run(["git","-C","/Users/master/cyber/cyber","push","-q"], check=False)
else:
    print(f"[{stamp}] in sync, no drift")
