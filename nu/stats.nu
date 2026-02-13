# Статистика Logseq графа
# запуск: nu nu/stats.nu <путь-к-графу>
# пример: nu nu/stats.nu ~/git/cloud-forest

def main [graph_path: string] {
    let pages = ([$graph_path "pages"] | path join)
    let graph_name = ($graph_path | path basename)
    let files = (glob $"($pages)/*.md" | each {|f| ls $f | first} | flatten)
    let total = ($files | length)

    print $"═══════════════════════════════════════════"
    print $"  ($graph_name | str upcase) — СТАТИСТИКА ГРАФА"
    print $"═══════════════════════════════════════════"
    print ""

    # --- Общее ---
    let total_size = ($files | get size | math sum)
    let avg_size = ($files | get size | each {|s| $s | into int} | math avg | math round -p 0)
    let lines_total = ($files | each {|f| open $f.name | lines | length} | math sum)

    print "── Общее ──"
    print $"Файлов:            ($total)"
    print $"Общий размер:      ($total_size)"
    print $"Средний размер:    ($avg_size) B"
    print $"Всего строк:       ($lines_total)"
    print ""

    # --- Теги ---
    let tag_data = ($files | each {|f|
        let content = (open $f.name)
        let tag_line = ($content | lines | where {|l| $l starts-with "tags::"} | first | default "")
        let tags = if ($tag_line | is-empty) { [] } else {
            $tag_line | str replace "tags::" "" | split row "," | each {|t| $t | str trim}
        }
        {file: ($f.name | path basename | str replace ".md" ""), tags: $tags}
    })

    let all_tags = ($tag_data | get tags | flatten)
    let no_tags = ($tag_data | where {|r| ($r.tags | length) == 0})

    print "── Теги ──"
    print $"Уникальных тегов:  ($all_tags | uniq | length)"
    print $"Файлов без тегов:  ($no_tags | length)"
    print ""
    print ($all_tags | uniq -c | sort-by count -r | table)
    print ""

    # --- Ссылки ---
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

    print "── Связи ──"
    print $"Всего ссылок:          ($total_links)"
    print $"Уникальных целей:      ($unique_targets)"
    print $"Файлов со ссылками:    ($files_with_links) / ($total)"
    print $"Среднее ссылок/файл:   (($total_links / $total) | math round -p 1)"
    print ""

    print "── Топ-15 по входящим ссылкам ──"
    print ($in_counts | first 15 | table)
    print ""

    print "── Топ-10 по исходящим ссылкам ──"
    print ($link_data | sort-by out_count -r | first 10 | select file out_count | table)
    print ""

    print $"── Осиротевшие страницы: ($orphans | length) ──"
    print "  (на них никто не ссылается)"
    print ($orphans | first 20 | table)
    print ""

    print $"── Битые ссылки: ($broken | length) ──"
    print "  (ведут на несуществующие страницы)"
    print ($broken | first 20 | table)
    print ""

    # --- Контент ---
    let with_ipfs = ($files | where {|f| (open $f.name) | str contains "ipfs.io"} | length)
    let with_compounds = ($files | where {|f| (open $f.name) | str contains "chemical compound"} | length)
    let with_tables = ($files | where {|f| (open $f.name) =~ "\\|.*\\|.*\\|"} | length)

    print "── Контент ──"
    print $"С IPFS ссылками:       ($with_ipfs)"
    print $"С хим. соединениями:   ($with_compounds)"
    print $"С таблицами:           ($with_tables)"
}
