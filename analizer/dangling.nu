# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# Alias-aware dangling wiki-link detector
# Usage: nu analizer/dangling.nu ~/git/cyber

def main [graph_path: string] {
    # auto-detect page directory: root/ → graph/ → pages/ (same as optica)
    let pages = if ($graph_path | path join "root" | path exists) {
        $graph_path | path join "root"
    } else if ($graph_path | path join "graph" | path exists) {
        $graph_path | path join "graph"
    } else {
        $graph_path | path join "pages"
    }

    let files = (glob $"($pages)/**/*.md")
    let total_pages = ($files | length)

    print $"scanning ($total_pages) pages in ($pages)"
    print ""

    # --- build lookup set from basenames, relative paths, and aliases ---
    # every entry is lowercased for case-insensitive matching

    mut known: list<string> = []

    let page_records = ($files | each {|f|
        let rel = ($f | str replace $"($pages)/" "" | str replace ".md" "")
        let base = ($f | path parse | get stem)
        let content = (try { open --raw $f } catch { "" })

        # extract aliases from frontmatter (both alias: and alias:: forms)
        let ls = ($content | lines)
        let has_fm = ($ls | length) > 0 and ($ls | first) == "---"
        let fm_lines = if $has_fm {
            $ls | skip 1 | take while {|l| $l != "---"}
        } else {
            []
        }

        let alias_lines = ($fm_lines | where {|l| $l =~ "^alias"})
        let aliases = if ($alias_lines | length) > 0 {
            let raw = ($alias_lines | first)
            # strip "alias::" or "alias:" prefix
            let after = ($raw | str replace --regex '^alias::?\s*' '')
            $after | split row "," | each {|a| $a | str trim | str downcase} | where {|a| ($a | str length) > 0}
        } else {
            []
        }

        {rel: ($rel | str downcase), base: ($base | str downcase), aliases: $aliases}
    })

    # flatten all known names into one list
    let known = (
        ($page_records | get rel)
        | append ($page_records | get base)
        | append ($page_records | get aliases | flatten)
        | uniq
    )

    print $"known names [basenames + paths + aliases]: ($known | length)"
    print ""

    # --- extract wiki-links from all pages ---
    # regex captures the link target before any | (display text)
    let all_links = ($files | each {|f|
        let content = (try { open --raw $f } catch { "" })
        let rel = ($f | str replace $"($pages)/" "" | str replace ".md" "")
        let links = ($content | parse --regex '\[\[([^\]|]+)' | get -i capture0 | default [])
        $links | each {|link| {source: $rel, link: ($link | str trim | str downcase)}}
    } | flatten)

    let total_links = ($all_links | length)
    let unique_targets = ($all_links | get link | uniq | length)

    # --- find dangling: link target not in known set ---
    let dangling = ($all_links | where {|row|
        not ($row.link in $known)
    })

    let dangling_total = ($dangling | length)
    let dangling_unique = ($dangling | get link | uniq | length)

    # --- group by link, count occurrences, show top 50 ---
    let grouped = ($dangling
        | group-by link
        | items {|link, rows|
            let sources = ($rows | get source | uniq)
            {
                link: $link
                count: ($sources | length)
                referenced_in: ($sources | first 5 | str join ", ")
            }
        }
        | sort-by count --reverse
    )

    let top = ($grouped | first ([$grouped | length, 50] | math min))

    print "── top dangling wiki-links ──"
    print ($top | table)
    print ""

    print "── stats ──"
    print $"total pages:            ($total_pages)"
    print $"total wiki-links:       ($total_links)"
    print $"unique link targets:    ($unique_targets)"
    print $"dangling references:    ($dangling_total)"
    print $"unique dangling:        ($dangling_unique)"
    let resolved = $unique_targets - $dangling_unique
    let pct = if $unique_targets > 0 {
        ($resolved * 100) / $unique_targets | math round --precision 1
    } else { 0 }
    print $"resolution rate:        ($pct)%"
}
