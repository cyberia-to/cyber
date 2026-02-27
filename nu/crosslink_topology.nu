#!/usr/bin/env nu
# Crosslink topology analysis for the semantic core
# Usage: nu nu/crosslink_topology.nu <graph_path>

def main [graph_path: string] {
    let core_concepts = [
        "graph" "link" "particle" "cyberlink" "cybergraph"
        "axon" "neuron" "avatar" "spell" "focus"
        "karma" "skill" "soul" "attention" "will"
        "token" "coin" "card" "score" "badge"
        "value" "price" "supply" "demand" "cap"
        "signal" "data" "hash" "proof" "signature"
        "information" "name" "file" "pay" "lock"
        "update" "mint" "burn" "vimputer" "time"
        "step" "state" "consensus" "finality" "tri-kernel"
        "tru" "cyberank" "knowledge" "observation" "learning"
        "inference" "training" "neural" "crystal" "cyber"
        "feedback" "equilibrium" "convergence" "syntropy" "egregore"
        "intelligence"
    ]

    # Map concept names to file paths
    let page_map = {
        graph: "bostrom/graph.md"
        link: "link.md"
        particle: "particle.md"
        cyberlink: "cyberlink.md"
        cybergraph: "cybergraph.md"
        axon: "axon.md"
        neuron: "neuron.md"
        avatar: "avatar.md"
        spell: "spell.md"
        focus: "cyber/focus.md"
        karma: "karma.md"
        skill: "skill.md"
        soul: "soul.md"
        attention: "attention.md"
        will: "will.md"
        token: "token.md"
        coin: "coin.md"
        card: "card.md"
        score: "score.md"
        badge: "badge.md"
        value: "value.md"
        price: "price.md"
        supply: "supply.md"
        demand: "demand.md"
        cap: "cap.md"
        signal: "signal.md"
        data: "data.md"
        hash: "hash.md"
        proof: "proof.md"
        signature: "signature.md"
        information: "information.md"
        name: "name.md"
        file: "file.md"
        pay: "pay.md"
        lock: "lock.md"
        update: "update.md"
        mint: "mint.md"
        burn: "burn.md"
        vimputer: "vimputer.md"
        time: "time.md"
        step: "step.md"
        state: "cyber/state.md"
        consensus: "consensus.md"
        finality: "finality.md"
        tri-kernel: "tri-kernel.md"
        tru: "tru.md"
        cyberank: "cyberank.md"
        knowledge: "knowledge.md"
        observation: "observation.md"
        learning: "learning.md"
        inference: "inference.md"
        training: "training.md"
        neural: "neural.md"
        crystal: "cyber/crystal.md"
        cyber: "cyber.md"
        feedback: "feedback.md"
        equilibrium: "equilibrium.md"
        convergence: "convergence.md"
        syntropy: "syntropy.md"
        egregore: "egregore.md"
        intelligence: "intelligence.md"
    }

    # Also build alias map for core concepts
    # These are known aliases from frontmatter that should map to core concepts
    let alias_to_core = {
        particles: "particle"
        cyberlinks: "cyberlink"
        cybergraphs: "cybergraph"
        axons: "axon"
        neurons: "neuron"
        tokens: "token"
        coins: "coin"
        cards: "card"
        scores: "score"
        badges: "badge"
        links: "link"
        names: "name"
        files: "file"
        signals: "signal"
        steps: "step"
        proofs: "proof"
        signatures: "signature"
        edges: "link"
        edge: "link"
        linking: "link"
        hashing: "hash"
        values: "value"
    }

    let results = $core_concepts | each {|concept|
        let rel_path = ($page_map | get $concept)
        let full_path = ($graph_path | path join "pages" $rel_path)
        let content = (open --raw $full_path)

        # Split off frontmatter
        let parts = ($content | split row "---\n" | skip 1)
        let body = if ($parts | length) > 1 {
            $parts | skip 1 | str join "---\n"
        } else {
            $content
        }

        let body_bytes = ($body | encode utf-8 | bytes length)
        let body_lines = ($body | lines | length)

        # Extract all [[wiki-links]] from body
        let raw_links = ($body | parse --regex '\[\[([^\]]+)\]\]' | get capture0 | uniq)

        # Lowercase for matching
        let links_lower = ($raw_links | each {|l| $l | str downcase})

        # Classify
        let core_links = ($links_lower | each {|l|
            if ($core_concepts | any {|c| $c == $l}) {
                $l
            } else if ($alias_to_core | columns | any {|a| $a == $l}) {
                ($alias_to_core | get $l)
            } else {
                null
            }
        } | compact | uniq | sort)

        let external_links = ($raw_links | where {|l|
            let ll = ($l | str downcase)
            let is_core = ($core_concepts | any {|c| $c == $ll})
            let is_alias = ($alias_to_core | columns | any {|a| $a == $ll})
            (not $is_core) and (not $is_alias)
        } | uniq | sort)

        {
            page: $concept
            core_count: ($core_links | length)
            ext_count: ($external_links | length)
            core_links: ($core_links | str join ", ")
            ext_links: ($external_links | str join ", ")
            body_lines: $body_lines
            body_bytes: $body_bytes
        }
    }

    # Sort by page name
    let sorted = ($results | sort-by page)

    print "=== CROSSLINK TOPOLOGY: SEMANTIC CORE (61 PAGES) ==="
    print ""
    print ($sorted | select page core_count ext_count body_lines body_bytes | table)

    print ""
    print "=== CORE LINKS DETAIL ==="
    print ($sorted | select page core_links | table --width 200)

    print ""
    print "=== EXTERNAL LINKS DETAIL ==="
    print ($sorted | select page ext_links | table --width 200)

    # Summary statistics
    let total_core = ($sorted | get core_count | math sum)
    let total_ext = ($sorted | get ext_count | math sum)
    let avg_core = ($total_core | into float) / ($sorted | length | into float)
    let avg_ext = ($total_ext | into float) / ($sorted | length | into float)

    print ""
    print "=== SUMMARY STATISTICS ==="
    print $"Total core links across all pages: ($total_core)"
    print $"Total external links across all pages: ($total_ext)"
    print $"Average core links per page: ($avg_core | math round --precision 2)"
    print $"Average external links per page: ($avg_ext | math round --precision 2)"

    # Pages with fewest core links
    print ""
    print "=== POTENTIAL ISLANDS (fewest core links) ==="
    print ($sorted | sort-by core_count | first 10 | select page core_count | table)

    # Pages with most core links
    print ""
    print "=== POTENTIAL HUBS (most core links) ==="
    print ($sorted | sort-by core_count --reverse | first 10 | select page core_count | table)

    # Most frequently linked-to core concepts
    print ""
    print "=== MOST LINKED-TO CORE CONCEPTS ==="
    let all_core_links = ($sorted | get core_links | each {|cl|
        $cl | split row ", " | where {|x| ($x | str trim | str length) > 0}
    } | flatten)

    let link_counts = ($all_core_links | uniq --count | sort-by count --reverse)
    print ($link_counts | first 20 | table)

    # Most common external links
    print ""
    print "=== MOST COMMON EXTERNAL LINKS ==="
    let all_ext_links = ($sorted | get ext_links | each {|el|
        $el | split row ", " | where {|x| ($x | str trim | str length) > 0}
    } | flatten)
    let ext_counts = ($all_ext_links | uniq --count | sort-by count --reverse)
    print ($ext_counts | first 20 | table)

    # Body sizes
    print ""
    print "=== BODY SIZES (sorted by bytes desc) ==="
    print ($sorted | sort-by body_bytes --reverse | select page body_bytes body_lines | table)
}
