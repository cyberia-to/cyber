# Cyber candidate qualification — 2026-09-24

Verdict: RED. All macOS/Linux ARM64/x64 platforms and shared stack gates ran in
[Actions run 35975320980](https://github.com/cyberia-to/cyber/actions/runs/35975320980).
[GitHub draft candidate](https://github.com/cyberia-to/cyber/releases)
retains their inventories, logs and checksums. No executable was produced.

Product source: `083871d20d30adfd1e31615b1615940ed4b9141f`. Shared qualifier:
`c5cc2077df510610abb7b5bd69675af6048ed724`. The source revision maps agree with the Soft3 and Cyb
candidates recorded on this date. The versions and complete source set are
retained in `candidate.json` and `sources.json`.

The `soft3-dependency` gate passes: the manifest pin, source and qualifier agree.
The native release gate fails on `source revision mismatch: bbg`; development
pins still select integration branches while the train closes over default
branches. Other failures include unavailable Nu, component lockfile readiness,
cybergraph's missing `local-storage` feature and unfinished conformance. Exact
commands and results are in the archives and `release-validation.json`.

The existing Node CI passed on all of its platforms for PR #111
([run 35974768436](https://github.com/cyberia-to/cyber/actions/runs/35974768436),
PR head `b5a7002e2cfd8fd0fcce207c06bbd33d7a5581b6`). This validates the development
build and its existing source lock. The stricter common candidate result remains
RED and supplies no production binary.

Commands:

```sh
gh workflow run release-train.yml --repo cyberia-to/cyber --ref master -f candidate=candidate-20260924.1 -f cut=false
gh run download 35975320980 --repo cyberia-to/cyber --name assembled-candidate --dir /tmp/cyber-final-assembled
python3 release/train.py draft --output /tmp/cyber-final-assembled
```

Draft creation used the qualifier revision above from a clean origin checkout.
`SHA256SUMS` covers the generated release assets; this README and remote
verification receipts are additional audit context. Assets were downloaded back
from GitHub and verified. No version tag or public promotion was performed.
The candidate freeze remains active; these receipts and the launch-log rows
stay on `release/2026-09-24` pending owner verdict.
