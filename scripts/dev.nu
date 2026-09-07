#!/usr/bin/env nu
# rebuild optica + restart protocol-only serve.
# full graph: cd ~/cyber/cyberia-blog && nu scripts/dev.nu

def main [
    --port: int = 8888
] {
    let optica_dir = $"($env.HOME)/cyber/optica"
    let workspace = $"($env.HOME)/cyber/cyber"

    print "▸ build"
    do {
        cd $optica_dir
        ^cargo build --release
    }

    print "▸ stop old serve"
    ^bash -c 'pgrep -f "optica serve" | xargs kill 2>/dev/null; pgrep -f "scripts/serve.nu" | xargs kill 2>/dev/null; true'
    sleep 1sec

    print $"▸ start protocol serve on ($port)"
    cd $workspace
    nu scripts/serve.nu --port $port
}
