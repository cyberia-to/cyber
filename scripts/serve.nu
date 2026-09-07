#!/usr/bin/env nu
# Protocol-only serve. The full graph lives in ~/cyber/cyberia-blog.
#
#   nu scripts/serve.nu
#   # whole project graph:
#   cd ~/cyber/cyberia-blog && nu scripts/serve.nu

def main [
    --port: int = 8888
    --bind: string = "127.0.0.1"
    --open
    --optica: path
] {
    let root = (pwd)
    if not ($"($root)/publish.toml" | path exists) {
        error make {msg: "run from the cyber repo root"}
    }
    let optica_bin = if $optica == null {
        $"($env.HOME)/cyber/optica/target/release/optica"
    } else {
        $optica
    }
    if not ($optica_bin | path exists) {
        error make {msg: $"optica binary not found at ($optica_bin)"}
    }
    let open_args = if $open { ["--open"] } else { [] }
    print $"serving protocol at http://($bind):($port)"
    print "full graph: cd ~/cyber/cyberia-blog && nu scripts/serve.nu"
    ^$optica_bin serve $root --output $"($root)/build" --port $port --bind $bind ...$open_args
}
