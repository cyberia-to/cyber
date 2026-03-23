# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# Crystal ontology classifier for graph pages
# Usage: nu nu/classify.nu ~/git/cyber
# Outputs JSON classification to $graph_path/data/crystal_classification.json

def main [graph_path: string] {
    # auto-detect page directory: root/ → graph/ → pages/ (same as optica)
    let pages_dir = if ($graph_path | path join "root" | path exists) {
        $graph_path | path join "root"
    } else if ($graph_path | path join "graph" | path exists) {
        $graph_path | path join "graph"
    } else {
        $graph_path | path join "pages"
    }

    # Type tag mappings: tag -> crystal type
    let entity_tags = [
        "species" "genus" "family" "class" "animals" "birds" "fungi" "plant"
        "compound" "compound-"
        "person"
        "building" "building type" "camp" "district" "pond"
        "ticker"
        "module"
        "page"
        "aip"
        "hero" "worker"
        "service" "product"
        "segment"
    ]

    let process_tags = [
        "operation" "critical"
        "kitchen/menu" "recipe" "breakfast" "mains"
    ]

    let property_tags = ["property"]
    let measure_tags = ["measure"]
    let pattern_tags = ["pattern"]
    let relation_tags = ["relation"]

    # Domain tag mappings: tag -> crystal domain
    # Order matters: first match wins (more specific first)
    let domain_rules = [
        { tags: ["property"] domain: "physics" }
        { tags: ["measure" "physics"] domain: "physics" }
        { tags: ["measure"] domain: "physics" }
        { tags: ["pattern"] domain: "mathematics" }
        { tags: ["relation"] domain: "mathematics" }
        { tags: ["species" "genus" "family" "class" "animals" "birds" "fungi" "shroom" "plant" "biology"] domain: "biology" }
        { tags: ["compound" "compound-"] domain: "chemistry" }
        { tags: ["physics" "force" "wave" "field"] domain: "physics" }
        { tags: ["mathematics" "algebra" "geometry" "topology" "statistics" "information theory" "game theory"] domain: "mathematics" }
        { tags: ["computer science" "cryptography" "machine learning" "cryptographic proofs"] domain: "computer science" }
        { tags: ["superhuman" "muscle" "disease" "longevity"] domain: "body" }
        { tags: ["cybernomics" "ticker" "token" "value" "delegation"] domain: "economics" }
        { tags: ["governance" "sovereignty" "law"] domain: "governance" }
        { tags: ["geography" "biome" "continent" "earth"] domain: "geography" }
        { tags: ["time" "history" "epoch" "year"] domain: "history" }
        { tags: ["culture" "language" "philosophy" "music"] domain: "culture" }
        { tags: ["color" "emotion" "spectrum"] domain: "culture" }
        { tags: ["food" "kitchen/menu" "recipe" "breakfast" "mains"] domain: "agriculture" }
        { tags: ["tech" "technology" "material"] domain: "materials" }
        { tags: ["energy"] domain: "energy" }
        { tags: ["person"] domain: "meta" }
        { tags: ["annotation" "research" "term" "uhash"] domain: "meta" }
        { tags: ["cyberia" "cv.land" "building" "building type" "camp" "district" "operation" "critical" "block" "team" "worker" "pond" "wc" "segment" "rack"] domain: "cyberia" }
        { tags: ["cyber" "cyb" "cip" "module" "aip" "prism" "bostrom" "page" "aos" "hero" "state" "param" "service" "product"] domain: "cyber" }
    ]

    let results = (glob ($pages_dir | path join "*.md") | each {|f|
        let content = (open $f --raw)
        let size = ($content | str length)
        let name = ($f | path basename | str replace ".md" "")
        let lines = ($content | lines)
        let first_line = ($lines | first)
        let tags = if ($first_line | str starts-with "tags::") {
            $first_line | str replace "tags:: " "" | str replace "tags::" "" | split row ", " | each {|t| $t | str trim} | where {|t| ($t | str length) > 0}
        } else {
            []
        }

        # Size class
        let size_class = if $size <= 256 { "atom" } else if $size <= 512 { "enzyme" } else if $size <= 1024 { "bridge" } else if $size <= 4096 { "article" } else { "deep" }

        # Determine type
        let has_entity = ($tags | any {|t| $t in $entity_tags})
        let has_process = ($tags | any {|t| $t in $process_tags})
        let has_property = ($tags | any {|t| $t in $property_tags})
        let has_measure = ($tags | any {|t| $t in $measure_tags})
        let has_pattern = ($tags | any {|t| $t in $pattern_tags})
        let has_relation = ($tags | any {|t| $t in $relation_tags})
        let has_article = ("article" in $tags)
        let is_ticker = ($name | str starts-with "$")

        let crystal_type = if $has_property { "Q" } else if $has_measure { "M" } else if $has_pattern { "S" } else if $has_relation { "R" } else if $has_process { "P" } else if $has_entity { "E" } else if $is_ticker { "E" } else { "unknown" }

        # Determine domain
        let crystal_domain = if ($tags | is-empty) and (not $is_ticker) {
            "unknown"
        } else if $is_ticker {
            "economics"
        } else {
            let matched = ($domain_rules | where {|rule|
                $rule.tags | any {|rt| $rt in $tags}
            })
            if ($matched | is-empty) { "unknown" } else { $matched | first | get domain }
        }

        let confidence = if $crystal_type == "unknown" or $crystal_domain == "unknown" { "low" } else { "high" }

        {
            name: $name
            size: $size
            size_class: $size_class
            tags: $tags
            crystal_type: $crystal_type
            crystal_domain: $crystal_domain
            confidence: $confidence
        }
    })

    let data_dir = ($graph_path | path join "data")
    mkdir $data_dir
    $results | to json | save -f ($data_dir | path join "crystal_classification.json")

    let total = ($results | length)
    let high = ($results | where confidence == "high" | length)
    let low = ($results | where confidence == "low" | length)

    print $"Total pages: ($total)"
    print $"High confidence: ($high)"
    print $"Low confidence: ($low)"
    print ""
    print "=== Type distribution ==="
    print ($results | where confidence == "high" | group-by crystal_type | items {|k, v| {type: $k, count: ($v | length)}} | sort-by count -r | table)
    print ""
    print "=== Domain distribution ==="
    print ($results | where confidence == "high" | group-by crystal_domain | items {|k, v| {domain: $k, count: ($v | length)}} | sort-by count -r | table)
}
