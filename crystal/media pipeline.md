---
tags: cyber
crystal-type: entity
crystal-domain: cyber
---

how the [[crystal]] handles media files from authoring to publishing

## local authoring

drop files into `media/` at the repo root. reference them from any page:

```
![diagram](../media/architecture-diagram.png)
```

the `media/` directory is gitignored. files live only on the author's machine until uploaded to [[ipfs]]

## pre-commit upload

a git pre-commit hook runs `nu/ipfs.nu` on every commit. for each staged markdown file it:

1. extracts `../media/filename` references
2. uploads each file to [[pinata]] ipfs via api
3. rewrites the markdown in-place: `../media/filename` → `https://gateway/ipfs/QmCID`
4. re-stages the modified file

credentials live in `~/.config/cyber/env` (PINATA_JWT and PINATA_GATEWAY). if the env file is missing the hook silently skips — the ci pipeline handles it

## ci pipeline

the github actions workflow `.github/workflows/publish.yml` runs on every push to master

### cid cache

`ipfs-cache.json` persists between ci runs via `actions/cache`. maps filename to cid:

```json
{"architecture-diagram.png": "QmXyz..."}
```

rolling key pattern: each run saves a new snapshot, restores the latest on next run

### resolve step

1. scan all `pages/*.md` for `../media/` references
2. for each filename: check cache first, upload to pinata only if missing
3. test custom gateway with first cid, fall back to public gateway if non-200
4. rewrite all `../media/filename` → `https://gateway/ipfs/QmCID` via sed
5. prune stale entries from cache (files no longer referenced in any page)

### build

[[cyber-publish]] builds the site. markdown already contains ipfs urls at this point. images render as `<img src="https://gateway.pinata.cloud/ipfs/QmCID">`

### deploy

the `public/` directory is zipped and pushed to [[netlify]]

## published state

the live site at cyber.page serves all media from ipfs gateway. no media stored on netlify. the [[files]] page catalogs every file referenced across the graph with its cid and referencing pages

## flow

```
author → media/ (local)
       → pre-commit hook → pinata ipfs (optional)
       → git push → ci: cache check → upload if new → rewrite urls → build → deploy
       → cyber.page: all media served from ipfs gateway
```