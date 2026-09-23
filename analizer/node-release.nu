#!/usr/bin/env nu
# Package qualified binaries and collect the exact platform set for publication.
const TARGETS = [x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin]

def checked [command: string, args: list<string>] {
    let result = (run-external $command ...$args | complete)
    if $result.exit_code != 0 { error make {msg: $"($command): ($result.stderr)"} }
    $result.stdout | str trim
}

def digest [path: string] { open --raw $path | hash sha256 }

def validate [dir: string, build: record, revision: string, version: string] {
    let inventory = (open ($dir | path join soft3-dependencies.json))
    if not $build.sources_locked or $build.version != $"cyber ($version)" {
        error make {msg: "release requires a source-locked build of the declared version"}
    }
    if $build.target not-in $TARGETS { error make {msg: "unsupported release target"} }
    if $inventory.product.revision != $revision or $inventory.product.version != $version {
        error make {msg: "inventory product differs from release source"}
    }
    if (digest ($dir | path join soft3-dependencies.json)) != $build.soft3_dependencies_sha256 {
        error make {msg: "dependency inventory differs from qualified build"}
    }
    if (digest ($dir | path join sources.lock.json)) != $inventory.source_lock_sha256 or (
        digest ($dir | path join Cargo.lock)) != $inventory.cargo_lock_sha256 or (
        $build.lock_sha256 != $inventory.cargo_lock_sha256) {
        error make {msg: "release source locks differ from inventory"}
    }
}

def package [root: string] {
    let dist = ($root | path join dist)
    let version = (open ($root | path join Cargo.toml) | get package.version)
    let revision = (checked git [-C $root rev-parse HEAD])
    let build = (open ($dist | path join build.json))
    for name in [sources.lock.json Cargo.lock rust-toolchain.toml LICENSE] {
        cp ($root | path join $name) ($dist | path join $name)
    }
    validate $dist $build $revision $version
    if (digest ($dist | path join cyber)) != $build.sha256 { error make {msg: "binary checksum mismatch"} }
    let files = [cyber build.json soft3-dependencies.json soft3-dependencies.md sources.lock.json Cargo.lock rust-toolchain.toml LICENSE]
    $files | each {|name| $"(digest ($dist | path join $name))  ($name)" }
        | str join (char nl) | $in + (char nl) | save --force ($dist | path join SHA256SUMS)
    let output = ($dist | path join release)
    mkdir $output
    let stem = $"cyber-v($version)-($build.target)"
    checked tar ([-czf ($output | path join $"($stem).tar.gz") -C $dist] | append $files | append SHA256SUMS) | ignore
    cp ($dist | path join build.json) ($output | path join $"($stem).build.json")
    for name in [soft3-dependencies.json soft3-dependencies.md sources.lock.json Cargo.lock rust-toolchain.toml] {
        cp ($dist | path join $name) ($output | path join $name)
    }
    print $"packaged ($stem)"
}

def collect [root: string, artifacts: string, tag: string] {
    let version = (open ($root | path join Cargo.toml) | get package.version)
    if $tag != $"v($version)" { error make {msg: "release tag must match Cargo.toml version"} }
    let revision = (checked git [-C $root rev-parse HEAD])
    let output = ($root | path join dist publish)
    if ($output | path exists) { error make {msg: "publication directory must be new"} }
    mkdir $output
    mut baseline = null
    for target in $TARGETS {
        let dir = ($artifacts | path join $"node-($target)")
        let stem = $"cyber-($tag)-($target)"
        let build = (open ($dir | path join $"($stem).build.json"))
        if $build.target != $target { error make {msg: "artifact target mismatch"} }
        validate $dir $build $revision $version
        for name in [sources.lock.json Cargo.lock rust-toolchain.toml] {
            if (digest ($root | path join $name)) != (digest ($dir | path join $name)) {
                error make {msg: $"artifact ($name) differs from release source"}
            }
        }
        let shared = ([soft3-dependencies.json soft3-dependencies.md sources.lock.json Cargo.lock rust-toolchain.toml]
            | each {|name| {name: $name, sha256: (digest ($dir | path join $name))} })
        if $baseline != null and $shared != $baseline { error make {msg: "platform dependency inventories differ"} }
        $baseline = $shared
        let unpacked = (checked mktemp [-d])
        checked tar [-xzf ($dir | path join $"($stem).tar.gz") -C $unpacked] | ignore
        if (digest ($unpacked | path join cyber)) != $build.sha256 or (
            open ($unpacked | path join build.json)) != $build {
            error make {msg: "archive differs from qualified build"}
        }
        for item in $shared {
            if (digest ($unpacked | path join $item.name)) != $item.sha256 {
                error make {msg: "archive inventory differs from release inventory"}
            }
        }
        rm --recursive $unpacked
        for name in ($shared.name | append $"($stem).tar.gz" | append $"($stem).build.json") {
            cp --force ($dir | path join $name) ($output | path join $name)
        }
    }
    glob ($output | path join '*') | sort | each {|path| $"(digest $path)  ($path | path basename)" }
        | str join (char nl) | $in + (char nl) | save ($output | path join SHA256SUMS)
    [
        $"# Cyber ($tag)"
        ""
        "Development prerelease of the default local Cyber node profile."
        "Native binaries: Linux x86_64, Linux ARM64, macOS Apple Silicon."
        "Each platform passed unit tests and process acceptance against its release executable."
        "Distributed consensus, peer replication and the full Joy worker lifecycle retain their own delivery gates."
        ""
        "Each archive includes the executable, checksums, build provenance, source locks and this dependency inventory."
        "Verify the downloaded archive against the attached SHA256SUMS, then unpack and run `./cyber --help`."
        ""
        (open --raw ($output | path join soft3-dependencies.md))
    ] | str join (char nl) | save ($root | path join dist release-notes.md)
    print $"collected ($TARGETS | length) qualified platforms for ($tag)"
}

def main [graph_path: string, action: string, --artifacts: string = "", --tag: string = ""] {
    let root = ($graph_path | path expand)
    match $action {
        package => { package $root }
        collect => { collect $root ($artifacts | path expand) $tag }
        _ => { error make {msg: "expected package or collect"} }
    }
}
