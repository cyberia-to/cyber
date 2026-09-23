#!/usr/bin/env nu
# Exercise publication failures in a disposable fixture built from real provenance.
def checked [command: string, args: list<string>] {
    let result = (run-external $command ...$args | complete)
    if $result.exit_code != 0 { error make {msg: $result.stderr} }
    $result.stdout | str trim
}

def rejected [script: string, root: string, action: string, expected: string, extra: list<string> = []] {
    let result = (do { ^nu $script $root $action ...$extra } | complete)
    if $result.exit_code == 0 or not ($result.stderr | str contains $expected) {
        error make {msg: $"expected rejection '($expected)': ($result.stdout) ($result.stderr)"}
    }
    print $"rejected: ($expected)"
}

def main [graph_path: string] {
    let source = ($graph_path | path expand)
    let script = ($source | path join analizer/node-release.nu)
    let scratch = (checked mktemp [-d])
    let root = ($scratch | path join cyber)
    # Keep the product Git identity while changing only ignored release fixtures.
    checked git [clone --quiet --no-hardlinks --no-checkout $source $root] | ignore
    checked git [-C $root checkout --quiet --detach (checked git [-C $source rev-parse HEAD])] | ignore
    cp --recursive ($source | path join dist) ($root | path join dist)
    let dist = ($root | path join dist)
    let build = (open ($dist | path join build.json))
    let inventory = (open --raw ($dist | path join soft3-dependencies.json))
    checked nu [$script $root package] | print

    $build | update sources_locked false | to json | save --force ($dist | path join build.json)
    rejected $script $root package "source-locked build"
    $build | to json | save --force ($dist | path join build.json)

    rm ($dist | path join soft3-dependencies.json)
    rejected $script $root package "soft3-dependencies.json"
    $inventory | save ($dist | path join soft3-dependencies.json)

    $inventory | from json | update components.0.revision ("0" | fill --width 40 --character "0")
        | to json | save --force ($dist | path join soft3-dependencies.json)
    rejected $script $root package "inventory differs from qualified build"
    $inventory | save --force ($dist | path join soft3-dependencies.json)

    $inventory | from json | update source_lock_sha256 "wrong" | to json
        | save --force ($dist | path join soft3-dependencies.json)
    $build | update soft3_dependencies_sha256 (open --raw ($dist | path join soft3-dependencies.json) | hash sha256)
        | to json | save --force ($dist | path join build.json)
    rejected $script $root package "source locks differ from inventory"
    $inventory | save --force ($dist | path join soft3-dependencies.json)
    $build | to json | save --force ($dist | path join build.json)

    "wrong executable" | save --force ($dist | path join cyber)
    rejected $script $root package "binary checksum mismatch"
    cp --force ($source | path join dist cyber) ($dist | path join cyber)

    rejected $script $root collect "tag must match" [--tag v0.0.0 --artifacts $scratch]
    checked nu [$script $root package] | print

    # Synthetic platform labels exercise collection; CI separately builds each ISA.
    let artifacts = ($scratch | path join artifacts)
    mkdir $artifacts
    for target in [x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin] {
        $build | update target $target | to json | save --force ($dist | path join build.json)
        checked nu [$script $root package] | ignore
        cp --recursive ($dist | path join release) ($artifacts | path join $"node-($target)")
    }
    let version = (open ($root | path join Cargo.toml) | get package.version)
    let args = [--tag $"v($version)" --artifacts $artifacts]
    checked nu ([$script $root collect] | append $args) | print
    rm --recursive ($dist | path join publish)
    rm ($dist | path join release-notes.md)

    let arm = ($artifacts | path join node-aarch64-unknown-linux-gnu)
    "altered table" | save --append ($arm | path join soft3-dependencies.md)
    rejected $script $root collect "platform dependency inventories differ" $args
    cp --force ($dist | path join soft3-dependencies.md) ($arm | path join soft3-dependencies.md)
    rm --recursive ($dist | path join publish)

    mv $arm ($arm + ".missing")
    rejected $script $root collect ".build.json" $args
    mv ($arm + ".missing") $arm
    rm --recursive ($dist | path join publish)

    let unpacked = ($scratch | path join unpacked)
    mkdir $unpacked
    let archive = ($arm | path join $"cyber-v($version)-aarch64-unknown-linux-gnu.tar.gz")
    checked tar [-xzf $archive -C $unpacked] | ignore
    "corrupt executable" | save --force ($unpacked | path join cyber)
    checked tar [-czf $archive -C $unpacked .] | ignore
    rejected $script $root collect "archive differs from qualified build" $args
    rm --recursive $scratch
    print "release guard: nine negative checks, packaging and collection passed"
}
