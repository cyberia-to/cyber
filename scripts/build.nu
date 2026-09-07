#!/usr/bin/env nu
# Protocol-only build. The full graph lives in ~/cyber/cyberia-blog.
#
#   optica build . --output build
#   # or, the whole project graph:
#   cd ~/cyber/cyberia-blog && nu scripts/build.nu

def main [
    --output: path
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
    let out = if $output == null { $"($root)/build" } else { $output }
    print $"protocol-only build → ($out)"
    print "full graph: cd ~/cyber/cyberia-blog && nu scripts/build.nu"
    ^$optica_bin build $root --output $out
    if ($"($root)/_redirects" | path exists) {
        cp $"($root)/_redirects" $"($out)/_redirects"
    }
}
