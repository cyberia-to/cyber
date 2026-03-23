# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# Assign stake values to graph pages based on importance heuristics
# Total stake ≈ Goldilocks field p = 2^64 - 2^32 + 1
# Usage: nu analizer/stake.nu ~/git/cyber/graph

def get_fm_value [lines: list<string>, key: string] {
    let found = ($lines | where {|l| $l | str starts-with $"($key):"} | first | default "")
    if ($found | is-empty) {
        ""
    } else {
        $found | str replace $"($key):" "" | str trim | str trim --char '"'
    }
}

def compute_weight [raw: string, lines: list<string>] {
    let ct = (get_fm_value $lines "crystal-type")
    let cd = (get_fm_value $lines "crystal-domain")
    let cs = (get_fm_value $lines "crystal-size")
    let tags_str = (get_fm_value $lines "tags")
    let alias_str = (get_fm_value $lines "alias")

    let type_w = if $ct == "entity" { 5.0 } else if $ct == "process" { 4.0 } else if $ct == "measure" { 3.0 } else if $ct == "pattern" { 3.0 } else if $ct == "bridge" { 2.0 } else { 1.0 }

    let domain_w = if $cd == "cyber" { 3.0 } else if $cd == "economics" { 2.0 } else { 1.0 }

    let size_w = if $cs == "article" { 3.0 } else if $cs == "bridge" { 2.0 } else { 1.0 }

    let tag_count = if ($tags_str | is-empty) { 0 } else { $tags_str | split row "," | length }
    let tc = if $tag_count > 10 { 10 } else { $tag_count }
    let tag_w = 1.0 + ($tc | into float) * 0.1

    let alias_w = if ($alias_str | is-empty) { 1.0 } else { 1.5 }

    let word_count = ($raw | split row --regex '\s+' | where {|w| ($w | str length) > 0} | length)
    let wc = if $word_count > 1000 { 1000 } else { $word_count }
    let content_w = 1.0 + ($wc | into float) / 1000.0

    $type_w * $domain_w * $size_w * $tag_w * $alias_w * $content_w
}

def main [graph_path: string] {
    let p = 18446744069414584321.0
    let files = (glob $"($graph_path)/**/*.md")
    let total = ($files | length)
    print $"Found ($total) markdown files"

    # Phase 1: compute weights
    let weighted = ($files | each {|f|
        let raw = (open --raw $f)
        let lines = ($raw | lines)
        let weight = (compute_weight $raw $lines)
        {path: ($f | path expand | into string), weight: $weight}
    })

    let total_w = ($weighted | get weight | math sum)
    let scale = $p / $total_w
    print $"Total raw weight: ($total_w | math round --precision 2)"

    # Phase 2: write stakes to frontmatter
    $weighted | each {|w|
        let stake = ($w.weight * $scale | math round | into int)
        let content = (open --raw $w.path)
        let lines = ($content | lines)
        let first_line = ($lines | first | default "")

        let new_content = if $first_line == "---" {
            # Has frontmatter — find closing ---
            let end_results = ($lines | enumerate | where {|r| $r.index > 0 and $r.item == "---"})
            if ($end_results | is-empty) {
                $content
            } else {
                let close_idx = ($end_results | first | get index)
                let fm = ($lines | skip 1 | first ($close_idx - 1) | where {|l| not ($l | str starts-with "stake:")})
                let body = ($lines | skip ($close_idx + 1))
                let result = (["---"] | append $fm | append [$"stake: ($stake)"] | append ["---"] | append $body | str join "\n")
                $result + "\n"
            }
        } else {
            # No frontmatter
            let result = (["---" $"stake: ($stake)" "---"] | append $lines | str join "\n")
            $result + "\n"
        }

        $new_content | save --force $w.path
    } | ignore

    # Report
    let report = ($weighted | each {|w|
        let stake = ($w.weight * $scale | math round | into int)
        {page: ($w.path | path basename), stake: $stake, weight: ($w.weight | math round --precision 2)}
    } | sort-by stake -r)

    print $"\n── Top 30 by stake ──"
    print ($report | first 30 | table)
    print $"\n── Bottom 10 by stake ──"
    print ($report | last 10 | table)
    print $"\nUpdated ($total) pages"
}
