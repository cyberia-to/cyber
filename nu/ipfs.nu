# IPFS pre-commit hook: upload local assets to Pinata, rewrite URLs in markdown
# Usage: nu nu/ipfs.nu <graph-path>
# Reads PINATA_JWT and PINATA_GATEWAY from ~/.config/cyber/env

def main [graph_path: string] {
    let env_file = ($"($env.HOME)/.config/cyber/env")
    if not ($env_file | path exists) {
        print "No ~/.config/cyber/env file found, skipping IPFS upload"
        print "Create it with: PINATA_JWT=<your-jwt> and PINATA_GATEWAY=<your-gateway>"
        return
    }

    # Parse env file
    let env_vars = (open $env_file | lines | where {|l| $l =~ '=' and not ($l | str starts-with '#')} | each {|l|
        let parts = ($l | split row '=' | collect)
        let key = ($parts | first | str trim)
        let val = ($parts | skip 1 | str join '=' | str trim)
        {key: $key, val: $val}
    } | reduce -f {} {|it, acc| $acc | insert $it.key $it.val})

    let jwt = ($env_vars | get -i PINATA_JWT | default '')
    let gateway = ($env_vars | get -i PINATA_GATEWAY | default '')

    if ($jwt | is-empty) or ($gateway | is-empty) {
        print "PINATA_JWT or PINATA_GATEWAY not set in ~/.config/cyber/env, skipping"
        return
    }

    # Find staged .md files that reference assets/
    let staged = (git diff --cached --name-only --diff-filter=ACM | lines | where {|f| $f =~ '\.md$'})
    if ($staged | is-empty) {
        return
    }

    # Collect all assets/ references from staged files
    let refs = ($staged | each {|f|
        let path = ([$graph_path $f] | path join)
        if ($path | path exists) {
            open $path | lines | each {|l|
                $l | parse -r 'assets/[^)\s"]+\.(png|jpg|jpeg|gif|webp|svg)' | get capture0?
            } | flatten
        }
    } | flatten | where {|r| $r != null} | uniq)

    if ($refs | is-empty) {
        return
    }

    let asset_paths = ($staged | each {|f|
        let path = ([$graph_path $f] | path join)
        if ($path | path exists) {
            open $path | parse -r '(assets/[^)\s"]+\.(?:png|jpg|jpeg|gif|webp|svg))' | get capture0
        }
    } | flatten | uniq)

    if ($asset_paths | is-empty) {
        return
    }

    print $"Found ($asset_paths | length) asset references in staged files"

    mut rewrites = []

    for asset in $asset_paths {
        let full_path = ([$graph_path $asset] | path join)
        if not ($full_path | path exists) {
            print $"  SKIP: ($asset) — file not found"
            continue
        }

        # Upload to Pinata
        print $"  Uploading ($asset)..."
        let resp = (http post
            --content-type multipart/form-data
            --headers [Authorization $"Bearer ($jwt)"]
            https://api.pinata.cloud/pinning/pinFileToIPFS
            {file: (open $full_path --raw)}
        )

        let cid = ($resp | get -i IpfsHash | default '')
        if ($cid | is-empty) {
            print $"  WARNING: Failed to upload ($asset)"
            continue
        }

        let ipfs_url = $"($gateway)/ipfs/($cid)"
        print $"  ($asset) → ($ipfs_url)"
        $rewrites = ($rewrites | append {from: $asset, to: $ipfs_url})
    }

    if ($rewrites | is-empty) {
        return
    }

    # Rewrite URLs in staged markdown files
    for f in $staged {
        let path = ([$graph_path $f] | path join)
        if not ($path | path exists) { continue }
        mut content = (open $path)
        for rw in $rewrites {
            $content = ($content | str replace --all $rw.from $rw.to)
        }
        $content | save -f $path
        # Re-stage the modified file
        cd $graph_path
        git add $f
    }

    print $"IPFS: ($rewrites | length) assets uploaded and rewritten"
}
