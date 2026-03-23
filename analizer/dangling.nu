# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# Find dangling wiki-links that have a namespaced match
# A link [[foo]] is dangling if graph/foo.md does NOT exist
# but graph/*/foo.md or graph/*/*/foo.md DOES exist somewhere
#
# run: nu analizer/dangling.nu ~/git/cyber/graph
# output: table of dangling links, their namespaced match, and referencing files

def main [graph_path: string] {
    # 1. Build page index: page_name → file_path
    let files = (glob $"($graph_path)/**/*.md")
    let page_index = ($files | each {|f|
        let rel = ($f | str replace $"($graph_path)/" "")
        let name = ($rel | str replace ".md" "")
        {name: $name, path: $rel}
    })
    let page_names = ($page_index | get name)

    print $"pages indexed: ($page_names | length)"
    print ""

    # 2. Extract all wiki-links from all files
    let all_links = ($files | each {|f|
        let content = (open --raw $f)
        let rel = ($f | str replace $"($graph_path)/" "")
        let links = ($content | parse --regex '\[\[([^\[\]]+)\]\]' | get capture0)
        $links | each {|link| {source: $rel, link: $link}}
    } | flatten)

    # 3. Find dangling: link target not in page_names
    let dangling = ($all_links | where {|row|
        not ($row.link in $page_names)
    })

    # 4. For each dangling link, check if a namespaced version exists
    #    e.g. [[oracle]] is dangling, but cyb/oracle exists
    let with_match = ($dangling | each {|row|
        let bare = ($row.link | path basename)
        let matches = ($page_names | where {|name|
            ($name | str ends-with $"/($bare)") and $name != $row.link
        })
        if ($matches | length) > 0 {
            {
                link: $row.link
                source: $row.source
                match: ($matches | str join ", ")
            }
        }
    } | compact)

    if ($with_match | length) == 0 {
        print "no dangling links with namespaced matches found"
        return
    }

    # 5. Group by dangling link for readable output
    let grouped = ($with_match
        | group-by link
        | items {|link, rows|
            let sources = ($rows | get source | uniq | str join ", ")
            let match = ($rows | first | get match)
            {dangling: $link, should_be: $match, referenced_in: $sources, count: ($rows | get source | uniq | length)}
        }
        | sort-by count --reverse
    )

    print $"found ($grouped | length) dangling links with namespaced matches:"
    print ""
    print ($grouped | table)
}
