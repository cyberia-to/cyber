# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# Apply Crystal ontology classification to graph pages
# Reads classification from /tmp/crystal_classification.json
# Adds crystal-type:: and crystal-domain:: properties after tags:: line
# Usage: nu nu/apply-crystal.nu ~/git/cyber

def main [graph_path: string] {
    # auto-detect page directory: root/ → graph/ → pages/ (same as optica)
    let pages_dir = if ($graph_path | path join "root" | path exists) {
        $graph_path | path join "root"
    } else if ($graph_path | path join "graph" | path exists) {
        $graph_path | path join "graph"
    } else {
        $graph_path | path join "pages"
    }
    let classification = (open /tmp/crystal_classification.json)

    let type_names = {
        E: "entity"
        P: "process"
        Q: "property"
        R: "relation"
        M: "measure"
        S: "pattern"
    }

    mut modified = 0
    mut skipped = 0

    for entry in $classification {
        let file = ($pages_dir | path join $"($entry.name).md")
        if not ($file | path exists) {
            print $"SKIP: ($entry.name) - file not found"
            $skipped = $skipped + 1
            continue
        }

        let content = (open $file --raw)
        let lines = ($content | lines)
        let crystal_type = ($type_names | get $entry.crystal_type)
        let crystal_domain = $entry.crystal_domain

        # Check if already has crystal-type
        let has_crystal_type = ($lines | any {|l| $l | str starts-with "crystal-type::"})
        let has_crystal_domain = ($lines | any {|l| $l | str starts-with "crystal-domain::"})

        if $has_crystal_type and $has_crystal_domain {
            $skipped = $skipped + 1
            continue
        }

        # Find insertion point: after tags:: line, or after alias:: if present, or at line 0
        let tags_matches = ($lines | enumerate | where {|r| $r.item | str starts-with "tags::"})
        let tags_idx = if ($tags_matches | is-empty) { -1 } else { $tags_matches | get index | first }

        let new_lines = if $tags_idx >= 0 {
            # Check if next lines are alias:: or icon:: - skip past those
            mut insert_at = ($tags_idx + 1)
            let remaining = ($lines | skip ($tags_idx + 1))
            for line in $remaining {
                if ($line | str starts-with "alias::") or ($line | str starts-with "icon::") {
                    $insert_at = $insert_at + 1
                } else {
                    break
                }
            }

            # Build new content
            let before = ($lines | first $insert_at)
            let after = ($lines | skip $insert_at)

            mut new_props = []
            if not $has_crystal_type {
                $new_props = ($new_props | append $"crystal-type:: ($crystal_type)")
            }
            if not $has_crystal_domain {
                $new_props = ($new_props | append $"crystal-domain:: ($crystal_domain)")
            }

            $before | append $new_props | append $after
        } else {
            # No tags:: line - prepend crystal properties at top
            let props = [$"crystal-type:: ($crystal_type)" $"crystal-domain:: ($crystal_domain)"]
            $props | append $lines
        }

        $new_lines | str join "\n" | save -f $file
        $modified = $modified + 1
    }

    print $"Modified: ($modified) pages"
    print $"Skipped: ($skipped) pages"
}
