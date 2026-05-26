#!/usr/bin/env nu
# serve.nu — materialize optica subgraph config and run optica serve with live reload
#
# Same shape as build.nu but invokes `optica serve` instead of `build`.

def main [
    --public-only              # exclude private + local-only subgraphs
    --output: path             # override output directory
    --port: int = 8888         # server port
    --bind: string = "127.0.0.1"
    --open                     # open browser on start
    --optica: path             # path to optica binary
] {
    let ws_root = (workspace-root)
    let ws = (open $"($ws_root)/workspace.toml")
    let root_dir = ($ws.root_dir | path expand)
    let root_graph = ($root_dir | path join $ws.graph.root_subgraph)

    let root_name = $ws.graph.root_subgraph
    let decls = (load-declarations $ws_root)
    let filtered = (filter-decls $decls $public_only $root_name)
    let subgraphs = ($filtered | each {|d|
        let derived_mount = if ($d.mount? | is-not-empty) {
            $d.mount
        } else if ($d.parent? | is-not-empty) {
            $"($d.parent)/($d.name)"
        } else {
            $d.name
        }
        {name: $d.name, path: ($root_dir | path join ($d.repo? | default $d.name)), mount: $derived_mount, visibility: ($d.visibility? | default "public")}
    })

    let config_path = "/tmp/optica-subgraphs.toml"
    ({subgraphs: $subgraphs} | to toml) | save --force $config_path

    let optica_bin = if $optica == null {
        $"($env.HOME)/cyber/optica/target/release/optica"
    } else {
        $optica
    }
    if not ($optica_bin | path exists) {
        error make {msg: $"optica binary not found at ($optica_bin)"}
    }

    let out = (resolve-output $ws_root $ws.graph.output $output)

    print $"serving ($root_graph) with ($subgraphs | length) subgraphs at http://($bind):($port)"
    print $"output: ($out)"

    let open_args = if $open { ["--open"] } else { [] }
    let ipfs_args = (ipfs-args $ws_root $ws)
    ^$optica_bin serve $root_graph --output $out --subgraphs $config_path --port $port --bind $bind ...$open_args ...$ipfs_args
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

def resolve-output [ws_root: string, ws_output: string, cli_override] {
    let raw = if $cli_override == null { $ws_output } else { $cli_override }
    if ($raw | str starts-with "/") {
        $raw
    } else {
        $ws_root | path join $raw
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
