# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
def main [graph_path: string] {
  # auto-detect page directory: root/ → graph/ → pages/ (same as optica)
  let pages_dir = if ($graph_path | path join "root" | path exists) {
      "root"
  } else if ($graph_path | path join "graph" | path exists) {
      "graph"
  } else {
      "pages"
  }
  let groups = [
    {group: "graph", members: ["link", "particle", "cyberlink", "cybergraph", "axon"]},
    {group: "neuron", members: ["avatar", "spell", "focus", "karma", "skill", "soul", "attention", "will"]},
    {group: "token", members: ["coin", "card", "score", "badge"]},
    {group: "value", members: ["price", "supply", "demand", "cap"]},
    {group: "signal", members: ["data", "hash", "proof", "signature", "information", "name", "file"]},
    {group: "cyberlink", members: ["pay", "lock", "update", "mint", "burn"]},
    {group: "vimputer", members: ["time", "step", "state", "consensus", "finality", "tri-kernel", "tru", "cyberank"]},
    {group: "knowledge", members: ["observation", "learning", "inference", "training", "neural", "crystal"]},
    {group: "cyber", members: ["feedback", "equilibrium", "convergence", "syntropy", "egregore", "intelligence"]}
  ]

  let all_concepts = $groups | each {|g|
    let all_pages = [$g.group] | append $g.members
    $all_pages | enumerate | each {|it|
      let p = $it.item
      let role = if $it.index == 0 { "head" } else { "member" }
      let path = ($graph_path | path join $pages_dir $"($p).md")
      if ($path | path exists) {
        let c = (open --raw $path)
        let lines = ($c | lines)
        let ct = ($lines | where {|l| $l starts-with "crystal-type:"} | if ($in | is-empty) { ["crystal-type: NONE"] } else { $in } | first | str replace "crystal-type: " "" | str trim)
        let cd = ($lines | where {|l| $l starts-with "crystal-domain:"} | if ($in | is-empty) { ["crystal-domain: NONE"] } else { $in } | first | str replace "crystal-domain: " "" | str trim)
        let has_icon = ($lines | where {|l| $l starts-with "icon:"} | is-not-empty)
        let alias_line = ($lines | where {|l| $l starts-with "alias:"} | if ($in | is-empty) { ["alias: "] } else { $in } | first | str replace "alias: " "" | str trim)
        let has_core_tag = ($lines | where {|l| $l =~ "tags:.*core"} | is-not-empty)
        let has_discover = ($lines | where {|l| $l =~ "discover all"} | is-not-empty)

        # count non-empty lines after frontmatter
        let in_body = ($c | split row "---" | skip 1 | if ($in | length) >= 2 { $in | skip 1 | str join "---" } else { "" })
        let body_count = ($in_body | lines | where {|l| ($l | str trim | str length) > 0} | length)
        let has_headers = ($in_body | lines | where {|l| $l starts-with "#"} | is-not-empty)

        {page: $p, group: $g.group, role: $role, type: $ct, domain: $cd, icon: $has_icon, core_tag: $has_core_tag, aliases: $alias_line, body_lines: $body_count, headers: $has_headers, discover: $has_discover}
      } else {
        {page: $p, group: $g.group, role: $role, type: "MISSING", domain: "", icon: false, core_tag: false, aliases: "", body_lines: 0, headers: false, discover: false}
      }
    }
  } | flatten

  print "== ALL CORE CONCEPTS =="
  print ($all_concepts | select page group role type domain body_lines | table)

  print ""
  print "== BY CRYSTAL TYPE =="
  let by_type = $all_concepts | where {|r| $r.type != "MISSING"} | group-by type | items {|k, v| {type: $k, count: ($v | length), pages: ($v | get page | str join ", ")}} | sort-by count -r
  print ($by_type | table)

  print ""
  print "== MISSING PAGES =="
  print ($all_concepts | where {|r| $r.type == "MISSING"} | select page group | table)

  print ""
  print "== MISSING core TAG =="
  print ($all_concepts | where {|r| $r.type != "MISSING" and $r.core_tag == false} | select page group type | table)

  print ""
  print "== NO icon =="
  print ($all_concepts | where {|r| $r.type != "MISSING" and $r.icon == false} | select page group type | table)

  print ""
  print "== NO discover footer =="
  print ($all_concepts | where {|r| $r.type != "MISSING" and $r.discover == false} | select page group type | table)

  print ""
  print "== HAS HEADERS (## sections) =="
  print ($all_concepts | where {|r| $r.headers == true} | select page group type body_lines | table)

  print ""
  print "== THIN PAGES (body <= 3 lines) =="
  print ($all_concepts | where {|r| $r.type != "MISSING" and $r.body_lines <= 3} | select page group type body_lines | table)

  print ""
  print "== LARGE PAGES (body > 20 lines) =="
  print ($all_concepts | where {|r| $r.body_lines > 20} | select page group type body_lines | table)
}
