# Graph statistics
# run: nu nu/stats.nu <path-to-graph>
# example: nu nu/stats.nu ~/git/cyber

def main [graph_path: string] {
    let pages = ([$graph_path "pages"] | path join)
    let graph_name = ($graph_path | path basename)
    let files = (glob $"($pages)/**/*.md" | each {|f| ls $f | first} | flatten)
    let total = ($files | length)

    print $"═══════════════════════════════════════════"
    print $"  ($graph_name | str upcase) — GRAPH STATS"
    print $"═══════════════════════════════════════════"
    print ""

    # --- General ---
    let total_size = ($files | get size | math sum)
    let avg_size = ($files | get size | each {|s| $s | into int} | math avg | math round -p 0)
    let lines_total = ($files | each {|f| open $f.name | lines | length} | math sum)

    print "── General ──"
    print $"Files:             ($total)"
    print $"Total size:        ($total_size)"
    print $"Avg size:          ($avg_size) B"
    print $"Total lines:       ($lines_total)"
    print ""

    # --- Tags (YAML frontmatter) ---
    let tag_data = ($files | each {|f|
        let content = (open $f.name)
        let ls = ($content | lines)
        let has_fm = ($ls | length) > 0 and ($ls | first) == "---"
        let tags = if $has_fm {
            let tag_line = ($ls | skip 1 | take while {|l| $l != "---"} | where {|l| $l =~ "^tags:"} | first | default "")
            if ($tag_line | is-empty) { [] } else {
                $tag_line | str replace "tags:" "" | split row "," | each {|t| $t | str trim} | where {|t| not ($t | is-empty)}
            }
        } else { [] }
        {file: ($f.name | path basename | str replace ".md" ""), tags: $tags}
    })

    let all_tags = ($tag_data | get tags | flatten)
    let no_tags = ($tag_data | where {|r| ($r.tags | length) == 0})

    print "── Tags ──"
    print $"Unique tags:       ($all_tags | uniq | length)"
    print $"Untagged files:    ($no_tags | length)"
    print ""
    print ($all_tags | uniq -c | sort-by count -r | first 30 | table)
    print ""

    if ($no_tags | length) > 0 {
        print $"── Untagged pages: ($no_tags | length) ──"
        print ($no_tags | get file | first 30 | table)
        print ""
    }

    # --- Links ---
    let link_data = ($files | each {|f|
        let content = (open $f.name)
        let found = ($content | parse --regex "\\[\\[([^\\]]+)\\]\\]" | get capture0 | each {|l| $l | str downcase})
        {
            file: ($f.name | path basename | str replace ".md" "")
            out_links: $found
            out_count: ($found | length)
        }
    })

    let all_links = ($link_data | get out_links | flatten)
    let total_links = ($all_links | length)
    let unique_targets = ($all_links | uniq | length)
    let files_with_links = ($link_data | where {|r| $r.out_count > 0} | length)
    let existing_pages = ($files | get name | each {|n| $n | path basename | str replace ".md" "" | str downcase})

    let in_counts = ($all_links | uniq -c | sort-by count -r)
    let referenced = ($all_links | uniq)
    let orphans = ($existing_pages | where {|p| $p not-in $referenced})
    let broken = ($referenced | where {|r| $r not-in $existing_pages})

    print "── Links ──"
    print $"Total links:           ($total_links)"
    print $"Unique targets:        ($unique_targets)"
    print $"Files with links:      ($files_with_links) / ($total)"
    print $"Avg links/file:        (($total_links / $total) | math round -p 1)"
    print ""

    print "── Top 15 by incoming links ──"
    print ($in_counts | first 15 | table)
    print ""

    print "── Top 10 by outgoing links ──"
    print ($link_data | sort-by out_count -r | first 10 | select file out_count | table)
    print ""

    print $"── Orphan pages: ($orphans | length) ──"
    print "  (no page links to them)"
    print ($orphans | first 30 | table)
    print ""

    print $"── Broken links: ($broken | length) ──"
    print "  (point to non-existent pages)"
    print ($broken | first 30 | table)
    print ""

    # --- Content ---
    let with_ipfs = ($files | where {|f| (open $f.name) | str contains "ipfs.io"} | length)
    let with_compounds = ($files | where {|f| (open $f.name) | str contains "chemical compound"} | length)
    let with_tables = ($files | where {|f| (open $f.name) =~ "\\|.*\\|.*\\|"} | length)

    print "── Content ──"
    print $"With IPFS links:       ($with_ipfs)"
    print $"With compounds:        ($with_compounds)"
    print $"With tables:           ($with_tables)"
}
