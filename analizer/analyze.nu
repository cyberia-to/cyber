# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# Аналитика Logseq графа
# запуск: nu nu/analyze.nu <путь-к-графу>
# пример: nu nu/analyze.nu ~/git/cloud-forest

def main [graph_path: string] {
    # auto-detect page directory: root/ → graph/ → pages/ (same as optica)
    let pages = if ([$graph_path "root"] | path join | path exists) {
        [$graph_path "root"] | path join
    } else if ([$graph_path "graph"] | path join | path exists) {
        [$graph_path "graph"] | path join
    } else {
        [$graph_path "pages"] | path join
    }
    let graph_name = ($graph_path | path basename)
    let files = (glob $"($pages)/*.md" | each {|f| ls $f | first} | flatten)
    let total = ($files | length)
    let total_size = ($files | get size | math sum)

    print $"═══════════════════════════════════════"
    print $"  ($graph_name | str upcase) — АНАЛИТИКА"
    print $"═══════════════════════════════════════"
    print ""
    print $"Всего файлов:  ($total)"
    print $"Общий размер:  ($total_size)"
    print ""

    # Топ-10 самых больших
    print "── Топ-10 самых больших файлов ──"
    print ($files
        | sort-by size -r
        | first 10
        | each {|r| {name: ($r.name | path basename), size: $r.size}}
        | table)

    print ""

    # Теги
    let tags = ($files | each {|f|
        let content = (open $f.name)
        let tag_line = ($content | lines | where {|l| $l starts-with "tags::"} | first | default "")
        if ($tag_line | is-empty) { [] } else {
            $tag_line | str replace "tags::" "" | split row "," | each {|t| $t | str trim}
        }
    } | flatten)

    print "── Частотность тегов ──"
    print ($tags | uniq -c | sort-by count -r | table)

    print ""

    # Категории
    let data = ($files | each {|f|
        let content = (open $f.name)
        let tag_line = ($content | lines | where {|l| $l starts-with "tags::"} | first | default "")
        let tags = if ($tag_line | is-empty) { [] } else {
            $tag_line | str replace "tags::" "" | split row "," | each {|t| $t | str trim}
        }
        let category = if ("birds" in $tags) {
            "birds"
        } else if (("fungi" in $tags) or ("shroom" in $tags)) {
            "fungi"
        } else if (("species" in $tags) or ("genus" in $tags) or ("plant" in $tags)) {
            "plants"
        } else {
            "other"
        }
        {file: ($f.name | path basename), size: ($f.size | into int), category: $category}
    })

    print "── Распределение по категориям ──"
    let categories = ($data | get category | uniq)
    print ($categories | each {|cat|
        let items = ($data | where {|r| $r.category == $cat})
        let cnt = ($items | length)
        let total = ($items | get size | math sum)
        {
            category: $cat
            count: $cnt
            total_kb: (($total / 1024) | math round -p 1)
            avg_bytes: (($total / $cnt) | math round -p 0)
        }
    } | sort-by count -r | table)

    print ""

    # Ссылки
    let links = ($files | each {|f|
        let content = (open $f.name)
        let found = ($content | parse --regex "\\[\\[([^\\]]+)\\]\\]" | get capture0)
        {
            file: ($f.name | path basename | str replace ".md" "")
            link_count: ($found | length)
        }
    })

    let total_links = ($links | get link_count | math sum)
    let files_with_links = ($links | where {|r| $r.link_count > 0} | length)

    print "── Граф связей ──"
    print $"Всего ссылок [[...]]:  ($total_links)"
    print $"Файлов со ссылками:   ($files_with_links) из ($total)"
    print ""

    print "── Топ-15 самых упоминаемых страниц ──"
    let all_links = ($files | each {|f|
        let content = (open $f.name)
        $content | parse --regex "\\[\\[([^\\]]+)\\]\\]" | get capture0
    } | flatten | each {|l| $l | str downcase})

    print ($all_links | uniq -c | sort-by count -r | first 15 | table)

    print ""

    # IPFS
    let ipfs = ($files | each {|f|
        let content = (open $f.name)
        let ipfs_links = ($content | parse --regex "ipfs\\.io/ipfs/([A-Za-z0-9]+)" | get capture0)
        {file: ($f.name | path basename | str replace ".md" ""), ipfs_count: ($ipfs_links | length)}
    } | where {|r| $r.ipfs_count > 0})

    let total_ipfs = ($ipfs | get ipfs_count | math sum)
    print "── IPFS контент ──"
    print $"Файлов с IPFS:   ($ipfs | length)"
    print $"Всего ссылок:    ($total_ipfs)"
    print ""
    print ($ipfs | sort-by ipfs_count -r | first 10 | table)
}
