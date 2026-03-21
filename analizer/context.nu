# context.nu — smart context packer for LLM consumption
# Scores pages by graph gravity (inbound links) and density,
# then greedily packs the most valuable content into a token budget.
#
# Usage:
#   nu analizer/context.nu ~/git/cyber                           # graph only, default 900K tokens
#   nu analizer/context.nu ~/git/cyber --subgraphs               # include subgraph repos
#   nu analizer/context.nu ~/git/cyber --subgraphs --budget 500  # 500K token budget
#   nu analizer/context.nu ~/git/cyber --subgraphs --stats       # print score table, don't write

def main [
  graph_path: string,
  --output (-o): string = "",
  --subgraphs (-s),
  --budget (-b): int = 900,       # token budget in thousands
  --stats,                        # print ranking table only
] {
  let token_budget = $budget * 1000
  # ~3.5 chars per token for mixed markdown+math content
  let char_budget = ($token_budget * 3.5 | into int)

  print $"Scanning pages..."

  # --- collect all markdown files ---
  mut all_files = (glob $"($graph_path)/root/**/*.md" | sort)

  # blog and scripts
  let blog = (glob $"($graph_path)/blog/*.md" | sort)
  let scripts = (glob $"($graph_path)/analizer/*.nu" | sort)
  let configs = (glob $"($graph_path)/*.md" | append (glob $"($graph_path)/*.toml") | sort)
  $all_files = ($all_files | append $blog | append $scripts | append $configs)

  # subgraph repos (sibling directories)
  mut subgraph_files = []
  if $subgraphs {
    let git_root = ($graph_path | path dirname)
    let repos = [hemera zheng nebu nox bbg cybernode mudra trident optica]
    for repo in $repos {
      let repo_path = $"($git_root)/($repo)"
      if ($repo_path | path exists) {
        let md = if ($"($repo_path)/root" | path exists) {
          glob $"($repo_path)/root/**/*.md"
        } else if ($"($repo_path)/graph" | path exists) {
          glob $"($repo_path)/graph/**/*.md"
        } else if ($"($repo_path)/pages" | path exists) {
          glob $"($repo_path)/pages/**/*.md"
        } else {
          glob $"($repo_path)/**/*.md"
            | where {|f| not ($f | str contains "/.git/")}
            | where {|f| not ($f | str contains "/build/")}
            | where {|f| not ($f | str contains "/target/")}
            | where {|f| not ($f | str contains "/node_modules/")}
        }
        $subgraph_files = ($subgraph_files | append $md)
      }
    }
    $all_files = ($all_files | append ($subgraph_files | sort))
  }

  print $"Total files: ($all_files | length)"

  # --- build page name → file path index and alias map ---
  let git_root = ($graph_path | path dirname)

  mut page_index = {} # lowercase_name → file_path
  mut alias_map = {}  # lowercase_alias → lowercase_canonical_name

  for f in $all_files {
    let rel = if ($f | str starts-with $graph_path) {
      $f | str replace $"($graph_path)/" ""
    } else {
      $f | str replace $"($git_root)/" ""
    }

    # derive page name from path: root/cyber/focus.md → cyber/focus
    let page_name = if ($rel | str starts-with "root/") {
      $rel | str replace "root/" "" | str replace ".md" ""
    } else {
      $rel | str replace ".md" "" | str replace ".nu" "" | str replace ".toml" ""
    }

    let name_lower = ($page_name | str downcase)
    $page_index = ($page_index | merge {$name_lower: $f})

    # extract aliases from frontmatter
    let raw = (open --raw $f)
    if ($raw | str starts-with "---") {
      let lines = ($raw | lines)
      let fm_matches = ($lines | skip 1 | enumerate | where {|x| $x.item == "---"})
      let fm_end = if ($fm_matches | length) > 0 { $fm_matches | first | get index } else { 999 }
      if $fm_end < 999 {
        let fm_lines = ($lines | skip 1 | first $fm_end)
        let alias_lines = ($fm_lines | where {|l| $l | str starts-with "alias:"})
        if ($alias_lines | length) > 0 {
          let alias_line = ($alias_lines | first)
          let aliases = ($alias_line | str replace "alias:" "" | split row "," | each {|a| $a | str trim | str downcase} | where {|a| ($a | str length) > 0})
          for a in $aliases {
            $alias_map = ($alias_map | merge {$a: $name_lower})
          }
        }
      }
    }
  }

  print $"Page index: ($page_index | columns | length) pages, ($alias_map | columns | length) aliases"

  # --- extract links and compute scores ---
  print $"Computing gravity and density..."

  mut inbound_counts = {}  # page_name → count of inbound links
  mut page_data = []       # {file, name, size, outbound, links_raw}

  for f in $all_files {
    let raw = (open --raw $f)
    let size = ($raw | str length)

    let rel = if ($f | str starts-with $graph_path) {
      $f | str replace $"($graph_path)/" ""
    } else {
      $f | str replace $"($git_root)/" ""
    }

    let page_name = if ($rel | str starts-with "root/") {
      $rel | str replace "root/" "" | str replace ".md" ""
    } else {
      $rel | str replace ".md" "" | str replace ".nu" "" | str replace ".toml" ""
    }

    # extract wiki-links
    let links = ($raw | parse --regex '\[\[([^\]]+)\]\]' | get capture0 | uniq)
    let links_lower = ($links | each {|l| $l | str downcase})

    # resolve each link and count inbound
    for link in $links_lower {
      # resolve alias → canonical
      let target = if ($alias_map | get -o $link) != null {
        $alias_map | get $link
      } else {
        $link
      }

      # increment inbound count
      let current = ($inbound_counts | get -o $target | default 0)
      $inbound_counts = ($inbound_counts | merge {$target: ($current + 1)})
    }

    $page_data = ($page_data | append {
      file: $f
      name: ($page_name | str downcase)
      rel: $rel
      size: $size
      outbound: ($links_lower | length)
    })
  }

  # --- score each page ---
  # gravity: inbound link count (how many pages reference this one)
  # density: outbound links per KB (how connected is this page relative to its size)
  # substance: raw content size (longer = more knowledge, but with diminishing returns)
  #
  # score = gravity² × (1 + density) × log2(substance)
  # gravity squared because hub pages are disproportionately valuable
  # log substance to favor content but not linearly (a 10KB page isn't 10× better than 1KB)

  mut scored = []
  for page in $page_data {
    let gravity = ($inbound_counts | get -o $page.name | default 0)
    let density = if $page.size > 0 { ($page.outbound / ($page.size / 1024.0)) } else { 0.0 }
    let substance = if $page.size > 100 { ($page.size | math log 2) } else { 1.0 }

    # bonus for pages with high stake (frontmatter field)
    let raw = (open --raw $page.file)
    let has_stake = ($raw | str contains "stake:")

    let gravity_sq = ($gravity * $gravity | into float)
    let score = $gravity_sq * (1.0 + $density) * $substance * (if $has_stake { 1.5 } else { 1.0 })

    $scored = ($scored | append {
      file: $page.file
      rel: $page.rel
      name: $page.name
      size: $page.size
      gravity: $gravity
      outbound: $page.outbound
      density: ($density | math round -p 2)
      score: ($score | math round -p 1)
    })
  }

  # sort by score descending
  let ranked = ($scored | sort-by score -r)

  if $stats {
    print "\n=== TOP 50 pages by score ==="
    let display = ($ranked | first 50 | each {|r| {
      rel: $r.rel
      gravity: $r.gravity
      out: $r.outbound
      score: $r.score
      kb: ($r.size / 1024 | math round -p 1)
    }})
    print ($display | table)

    let zero_gravity = ($ranked | where gravity == 0 | length)
    let total = ($ranked | length)
    print $"\nTotal: ($total) pages, ($zero_gravity) with zero inbound links"

    # gravity distribution
    let g_dist = ($ranked | where gravity > 0 | get gravity | describe)
    print $"Gravity distribution: ($g_dist)"
    return
  }

  # --- greedy knapsack: pack by score until budget ---
  let budget_msg = $"Packing into ($budget)K token budget [($char_budget) chars]..."
  print $budget_msg

  mut packed = []
  mut total_chars = 0
  mut packed_count = 0

  # always include top-level config first
  let config_files = ($ranked | where {|r| ($r.rel | str starts-with "CLAUDE") or ($r.rel | str starts-with "README") or ($r.rel | str ends-with ".toml")})
  for cf in $config_files {
    let content = (open --raw $cf.file | str trim)
    let entry = $"--- ($cf.rel) ---\n($content)\n"
    $total_chars = $total_chars + ($entry | str length)
    $packed = ($packed | append $entry)
    $packed_count = $packed_count + 1
  }

  # pack by score
  for page in $ranked {
    if $total_chars >= $char_budget { break }

    # skip configs already packed
    if ($page.rel | str starts-with "CLAUDE") or ($page.rel | str starts-with "README") or ($page.rel | str ends-with ".toml") {
      continue
    }

    let content = (open --raw $page.file | str trim)
    let entry = $"--- ($page.rel) ---\n($content)\n"
    let entry_size = ($entry | str length)

    if ($total_chars + $entry_size) > $char_budget {
      # skip this page if it would exceed budget — try smaller ones
      continue
    }

    $packed = ($packed | append $entry)
    $total_chars = $total_chars + $entry_size
    $packed_count = $packed_count + 1
  }

  let total_pages = ($ranked | length)
  let coverage_pct = ($packed_count * 100 / $total_pages | math round -p 1)
  let est_tokens = ($total_chars / 3.5 | math round -p 0 | into int)

  let gen_date = (date now | format date '%Y-%m-%d')
  let graph_name = ($graph_path | path basename)
  let header = [
    $"# Knowledge Graph Context: ($graph_name)"
    $"# Packed: ($packed_count) / ($total_pages) pages [($coverage_pct)%]"
    $"# Estimated tokens: ($est_tokens) / ($token_budget) budget"
    "# Method: gravity^2 * (1 + density) * log2(substance) — greedy knapsack"
    $"# Generated: ($gen_date)"
    ""
  ] | str join "\n"

  let result = ([$header] | append $packed | str join "\n")

  if $output == "" {
    let out_path = $"/tmp/cyber-context-($budget)k.md"
    $result | save -f $out_path
    let size_kb = ($total_chars / 1024 | math round -p 0)
    print $"Saved ($packed_count)/($total_pages) pages to ($out_path) — ($size_kb) KB, ~($est_tokens) tokens [($coverage_pct)% coverage]"
  } else {
    $result | save -f $output
    let size_kb = ($total_chars / 1024 | math round -p 0)
    print $"Saved ($packed_count)/($total_pages) pages to ($output) — ($size_kb) KB, ~($est_tokens) tokens [($coverage_pct)% coverage]"
  }
}
