# trikernel.nu — compute focus distribution via diffusion and write to frontmatter
#
# Runs a simplified tri-kernel: power iteration (diffusion component)
# over the wiki-link graph, then writes results into each page's YAML frontmatter.
#
# Fields written:
#   focus:    — normalized focus score (probability, sums to 1 across all pages)
#   gravity:  — raw inbound link count
#
# Usage:
#   nu analizer/trikernel.nu ~/git/cyber                    # graph only
#   nu analizer/trikernel.nu ~/git/cyber --subgraphs        # include subgraph repos
#   nu analizer/trikernel.nu ~/git/cyber --dry-run          # compute and print, don't write
#   nu analizer/trikernel.nu ~/git/cyber --iterations 30    # more iterations (default 20)

def main [
  graph_path: string,
  --subgraphs (-s),
  --dry-run (-d),
  --iterations (-i): int = 20,
  --alpha (-a): float = 0.85,  # damping factor (teleport = 1-alpha)
] {
  print "Scanning pages..."

  # --- collect files (same logic as context.nu) ---
  mut all_files = (glob $"($graph_path)/root/**/*.md" | sort)

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
        $all_files = ($all_files | append ($md | sort))
      }
    }
  }

  let git_root = ($graph_path | path dirname)
  print $"Total files: ($all_files | length)"

  # --- build page index and alias map ---
  mut page_index = {}
  mut alias_map = {}
  mut page_files = {}  # name → file path

  for f in $all_files {
    let rel = if ($f | str starts-with $graph_path) {
      $f | str replace $"($graph_path)/" ""
    } else {
      $f | str replace $"($git_root)/" ""
    }

    let page_name = if ($rel | str starts-with "root/") {
      $rel | str replace "root/" "" | str replace ".md" ""
    } else {
      $rel | str replace ".md" ""
    }
    let name_lower = ($page_name | str downcase)

    $page_index = ($page_index | merge {$name_lower: true})
    $page_files = ($page_files | merge {$name_lower: $f})

    # extract aliases
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

  # freeze for closure capture
  let alias_map = $alias_map
  let page_index = $page_index
  let page_files = $page_files
  let all_names = ($page_index | columns)
  let n = ($all_names | length)

  print $"Pages: ($n), aliases: ($alias_map | columns | length)"

  # --- build link graph ---
  print "Building link graph..."

  # outbound: page_name → [resolved target names]
  # inbound: page_name → [source names]
  mut outbound = {}
  mut inbound = {}

  # initialize
  for name in $all_names {
    $outbound = ($outbound | merge {$name: []})
    $inbound = ($inbound | merge {$name: []})
  }

  for name in $all_names {
    let f = ($page_files | get $name)
    let raw = (open --raw $f)
    let links = ($raw | parse --regex '\[\[([^\]]+)\]\]' | get capture0 | uniq)
    let resolved = ($links | each {|l|
      let lower = ($l | str downcase)
      let target = if ($alias_map | get -o $lower) != null { $alias_map | get $lower } else { $lower }
      $target
    } | where {|t| ($page_index | get -o $t) != null})  # only count links to existing pages

    $outbound = ($outbound | merge {$name: $resolved})
    for target in $resolved {
      let current = ($inbound | get $target)
      $inbound = ($inbound | merge {$target: ($current | append $name)})
    }
  }

  let outbound = $outbound
  let inbound = $inbound

  # --- compute gravity (raw inbound count) ---
  let gravity = ($all_names | each {|name|
    {name: $name, gravity: ($inbound | get $name | length)}
  })

  # --- power iteration: diffusion (PageRank) ---
  print $"Running diffusion: ($iterations) iterations, alpha=($alpha)..."

  let teleport = (1.0 - $alpha) / ($n | into float)

  # initialize uniform
  mut phi = {}
  let init_val = 1.0 / ($n | into float)
  for name in $all_names {
    $phi = ($phi | merge {$name: $init_val})
  }

  # find dangling nodes (no outbound links to existing pages)
  mut dangling_list = []
  for name in $all_names {
    if ($outbound | get $name | length) == 0 {
      $dangling_list = ($dangling_list | append $name)
    }
  }
  let dangling = $dangling_list
  let dangling_count = ($dangling | length)
  print $"Dangling nodes: ($dangling_count)"

  for iter in 1..($iterations + 1) {
    # dangling mass: sum of phi for nodes with no outbound
    mut dangling_mass = 0.0
    for name in $dangling {
      $dangling_mass = $dangling_mass + ($phi | get $name)
    }
    let dangling_share = $alpha * $dangling_mass / ($n | into float)

    mut phi_new = {}
    for name in $all_names {
      # contribution from inbound links
      let sources = ($inbound | get $name)
      mut link_sum = 0.0
      for src in $sources {
        let src_phi = ($phi | get $src)
        let src_out = ($outbound | get $src | length)
        if $src_out > 0 {
          $link_sum = $link_sum + $src_phi / ($src_out | into float)
        }
      }

      let val = $alpha * $link_sum + $teleport + $dangling_share
      $phi_new = ($phi_new | merge {$name: $val})
    }

    # normalize
    mut total = 0.0
    for name in $all_names {
      $total = $total + ($phi_new | get $name)
    }
    if $total > 0 {
      for name in $all_names {
        let v = ($phi_new | get $name)
        $phi_new = ($phi_new | merge {$name: ($v / $total)})
      }
    }

    # convergence check (L1 norm of delta)
    mut delta = 0.0
    for name in $all_names {
      let old = ($phi | get $name)
      let new_val = ($phi_new | get $name)
      $delta = $delta + (($old - $new_val) | math abs)
    }

    if ($iter mod 5) == 0 or $iter == 1 {
      let delta_fmt = ($delta | math round -p 8)
      print $"  iteration ($iter): delta = ($delta_fmt)"
    }

    $phi = $phi_new

    if $delta < 1e-8 {
      print $"  converged at iteration ($iter)"
      break
    }
  }

  let phi = $phi

  # --- build results table ---
  let results = ($all_names | each {|name|
    let focus_val = ($phi | get $name)
    let grav = ($inbound | get $name | length)
    {
      name: $name
      file: ($page_files | get $name)
      focus: $focus_val
      gravity: $grav
    }
  } | sort-by focus -r)

  if $dry_run {
    print "\n=== TOP 50 by focus ==="
    let display = ($results | first 50 | each {|r| {
      name: $r.name
      focus: ($r.focus * 1000000 | math round -p 1)  # show as micro-units
      gravity: $r.gravity
    }})
    print ($display | table)

    let total_focus = ($results | get focus | math sum | math round -p 6)
    print $"\nTotal focus: ($total_focus) [should be 1.0]"
    let high_focus = ($results | where {|r| $r.focus > 0.0001} | length)
    print $"Pages with focus > 1e-4: ($high_focus)"
    return
  }

  # --- write to frontmatter ---
  print "Writing focus and gravity to frontmatter..."

  mut written = 0
  for row in $results {
    let f = $row.file
    let raw = (open --raw $f)

    # skip files without frontmatter
    if not ($raw | str starts-with "---") {
      continue
    }

    let lines = ($raw | lines)
    let fm_matches = ($lines | skip 1 | enumerate | where {|x| $x.item == "---"})
    if ($fm_matches | length) == 0 {
      continue
    }
    let fm_end = ($fm_matches | first | get index)
    let fm_lines = ($lines | skip 1 | first $fm_end)
    let body_lines = ($lines | skip ($fm_end + 2))

    # format focus as scientific notation for readability
    let focus_str = ($row.focus | into string)

    # build new frontmatter: remove old focus/gravity lines, add new ones
    let clean_fm = ($fm_lines | where {|l|
      not ($l | str starts-with "focus:") and not ($l | str starts-with "gravity:")
    })

    let new_fm = ($clean_fm | append $"focus: ($focus_str)" | append $"gravity: ($row.gravity)")

    # reassemble
    let new_content = (["---"] | append $new_fm | append ["---"] | append $body_lines | str join "\n")

    $new_content | save -f $f
    $written = $written + 1
  }

  print $"Written focus + gravity to ($written) pages"
  print "\nTop 10 by focus:"
  print ($results | first 10 | each {|r| {
    name: $r.name
    focus: ($r.focus * 1000000 | math round -p 1)
    gravity: $r.gravity
  }} | table)
}
