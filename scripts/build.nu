#!/usr/bin/env nu
# build.nu — materialize optica subgraph config from the workspace and invoke optica
#
# Reads: workspace.toml + subgraphs/*.md declarations
# Writes: /tmp/optica-subgraphs.toml (ephemeral)
# Invokes: optica build <root_graph> --subgraphs /tmp/optica-subgraphs.toml

def main [
    --public-only          # exclude private + local-only subgraphs (for public deploys)
    --output: path         # override output directory
    --optica: path         # path to optica binary (defaults to ~/cyber/optica/target/release/optica)
    --pinned               # checkout each subgraph at its commit field (for reproducible CI builds)
] {
    let ws_root = (workspace-root)
    let ws = (open $"($ws_root)/workspace.toml")
    let root_dir = ($ws.root_dir | path expand)
    let root_graph = ($root_dir | path join $ws.graph.root_subgraph)
    let root_name = $ws.graph.root_subgraph

    let decls = (load-declarations $ws_root)
    let filtered = (filter-decls $decls $public_only $root_name)

    let subgraphs = ($filtered | enumerate | each {|it|
        let d = $it.item
        let repo_path = ($root_dir | path join ($d.repo? | default $d.name))
        if $pinned and ($d.commit? | is-not-empty) {
            ^git -C $repo_path checkout $d.commit
        }
        let derived_mount = if ($d.mount? | is-not-empty) {
            $d.mount
        } else if ($d.parent? | is-not-empty) {
            $"($d.parent)/($d.name)"
        } else {
            $d.name
        }
        let is_menu = ($d.parent? | is-empty)
        let base = {name: $d.name, path: $repo_path, mount: $derived_mount, visibility: ($d.visibility? | default "public"), menu: $is_menu}
        if $is_menu { $base | insert menu_order $it.index } else { $base }
    })

    let config_path = "/tmp/optica-subgraphs.toml"
    let toml_content = ({subgraphs: $subgraphs} | to toml)
    $toml_content | save --force $config_path

    let optica_bin = if $optica == null {
        $"($env.HOME)/cyber/optica/target/release/optica"
    } else {
        $optica
    }

    if not ($optica_bin | path exists) {
        error make {msg: $"optica binary not found at ($optica_bin). build it with: cd ($env.HOME)/cyber/optica && cargo build --release"}
    }

    print $"root graph: ($root_graph)"
    print $"subgraphs: ($subgraphs | length)"
    print $"config: ($config_path)"
    print ""

    let out = (resolve-output $root_dir $ws.graph.output $output)
    let ipfs_args = (ipfs-args $ws_root $ws)
    ^$optica_bin build $root_graph --output $out --subgraphs $config_path ...$ipfs_args
}

def ipfs-args [ws_root: string, ws] {
    let map_rel = ($ws.media?.ipfs_map? | default "ipfs-cache.json")
    let map_path = if ($map_rel | str starts-with "/") { $map_rel } else { $ws_root | path join $map_rel }
    if ($map_path | path exists) {
        let gateway = ($ws.media?.ipfs_gateway? | default "https://gateway.pinata.cloud")
        ["--ipfs-map" $map_path "--ipfs-gateway" $gateway]
    } else {
        []
    }
}

def resolve-output [root_dir: string, ws_output: string, cli_override] {
    # Relative paths resolve against root_dir (same base optica uses),
    # so `optica build` standalone and `build.nu` land in the same place.
    let raw = if $cli_override == null { $ws_output } else { $cli_override }
    if ($raw | str starts-with "/") {
        $raw
    } else {
        $root_dir | path join $raw
    }
}

def workspace-root [] {
    let cwd = (pwd)
    if ($"($cwd)/workspace.toml" | path exists) {
        $cwd
    } else {
        error make {msg: "workspace.toml not found; run from the cyber repo root (or any repo containing workspace.toml)"}
    }
}

def load-declarations [root: string] {
    let path = $"($root)/subgraphs.toml"
    if not ($path | path exists) { return [] }
    let data = (open $path)
    let key = if ($data | get -o subgraph | is-not-empty) { "subgraph" } else { "subgraphs" }
    $data | get $key | where ($it.name? | is-not-empty)
}

def filter-decls [decls, public_only: bool, root_name: string] {
    $decls
        | where ($it.archived? | default false) != true
        | where ($it.orphan? | default false) != true
        | where ($it.name? | default "") != $root_name
        | where {|d|
            if not $public_only {
                true
            } else {
                let vis = ($d.visibility? | default "public")
                let local_only = ($d."local-only"? | default false)
                $vis == "public" and not $local_only
            }
        }
}
