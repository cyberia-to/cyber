#!/usr/bin/env nu
# dev.nu — one-step rebuild + restart for local dev.
#
# Run this after ANY change under ~/cyber/optica (Rust source,
# templates/*.html, static/*.css|js|woff2). Optica's binary embeds
# templates and static assets via include_str!, so editing the source
# files alone has no effect on the running serve.
#
# Sequence: cargo build (release) → kill any running optica serve →
# launch a fresh serve in the background. Returns once the new serve
# has started; readiness is then verified by the caller.

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
    # Use bash to kill matching processes — nu's `ps` schema and pipe
    # syntax vary across versions; bash + pgrep + xargs is portable.
    ^bash -c 'pgrep -f "optica serve" | xargs kill 2>/dev/null; pgrep -f "scripts/serve.nu" | xargs kill 2>/dev/null; true'
    sleep 1sec

    print $"▸ start serve on ($port)"
    cd $workspace
    nu scripts/serve.nu --port $port
}
