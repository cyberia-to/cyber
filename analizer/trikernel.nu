# trikernel.nu — compute tri-kernel focus distribution and write to frontmatter
#
# Three operators over the wiki-link graph:
#   D (diffusion)  — PageRank: where does probability flow?
#   S (springs)    — screened Laplacian: mean neighbor focus (structural equilibrium)
#   H (heat)       — 2-hop smoothed focus (multi-scale context at resolution τ)
#
# Composite: focus = norm(λd·D + λs·S + λh·H),  λd + λs + λh = 1
#
# Fields written to frontmatter:
#   diffusion: — D component (PageRank probability)
#   springs:   — S component (neighbor equilibrium)
#   heat:      — H component (2-hop smoothed)
#   focus:     — composite tri-kernel score (normalized, sums to 1)
#   gravity:   — raw inbound link count
#   density:   — outbound wiki-links per KB of content
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
  --alpha (-a): float = 0.85,     # diffusion damping (teleport = 1-alpha)
  --lambda-d: float = 0.5,        # diffusion weight
  --lambda-s: float = 0.3,        # springs weight
  --lambda-h: float = 0.2,        # heat weight
  --tau: float = 1.0,             # heat kernel bandwidth
] {
  print "Scanning pages..."

  # --- collect files ---
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

  # --- build page index, alias map, sizes ---
  mut page_index = {}
  mut alias_map = {}
  mut page_files = {}
  mut page_sizes = {}

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
    let raw = (open --raw $f)
    let size = ($raw | str length)

    $page_index = ($page_index | merge {$name_lower: true})
    $page_files = ($page_files | merge {$name_lower: $f})
    $page_sizes = ($page_sizes | merge {$name_lower: $size})

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

  let alias_map = $alias_map
  let page_index = $page_index
  let page_files = $page_files
  let page_sizes = $page_sizes
  let all_names = ($page_index | columns)
  let n = ($all_names | length)
  let nf = ($n | into float)

  print $"Pages: ($n), aliases: ($alias_map | columns | length)"

  # --- build link graph ---
  print "Building link graph..."

  mut outbound = {}
  mut inbound = {}
  mut out_count = {}

  for name in $all_names {
    $outbound = ($outbound | merge {$name: []})
    $inbound = ($inbound | merge {$name: []})
    $out_count = ($out_count | merge {$name: 0})
  }

  for name in $all_names {
    let f = ($page_files | get $name)
    let raw = (open --raw $f)
    let links = ($raw | parse --regex '\[\[([^\]]+)\]\]' | get capture0 | uniq)
    let resolved = ($links | each {|l|
      let lower = ($l | str downcase)
      if ($alias_map | get -o $lower) != null { $alias_map | get $lower } else { $lower }
    } | where {|t| ($page_index | get -o $t) != null})

    $outbound = ($outbound | merge {$name: $resolved})
    $out_count = ($out_count | merge {$name: ($resolved | length)})
    for target in $resolved {
      let current = ($inbound | get $target)
      $inbound = ($inbound | merge {$target: ($current | append $name)})
    }
  }

  let outbound = $outbound
  let inbound = $inbound
  let out_count = $out_count

  # =================================================================
  # OPERATOR 1: DIFFUSION (PageRank)
  # =================================================================
  print $"D: diffusion — ($iterations) iterations, alpha=($alpha)..."

  let teleport = (1.0 - $alpha) / $nf

  mut phi_d = {}
  let init_val = 1.0 / $nf
  for name in $all_names {
    $phi_d = ($phi_d | merge {$name: $init_val})
  }

  mut dangling_list = []
  for name in $all_names {
    if ($out_count | get $name) == 0 {
      $dangling_list = ($dangling_list | append $name)
    }
  }
  let dangling = $dangling_list
  print $"  dangling nodes: ($dangling | length)"

  for iter in 1..($iterations + 1) {
    mut dangling_mass = 0.0
    for name in $dangling {
      $dangling_mass = $dangling_mass + ($phi_d | get $name)
    }
    let dangling_share = $alpha * $dangling_mass / $nf

    mut phi_new = {}
    for name in $all_names {
      let sources = ($inbound | get $name)
      mut link_sum = 0.0
      for src in $sources {
        let src_phi = ($phi_d | get $src)
        let src_out = ($out_count | get $src)
        if $src_out > 0 {
          $link_sum = $link_sum + $src_phi / ($src_out | into float)
        }
      }
      let val = $alpha * $link_sum + $teleport + $dangling_share
      $phi_new = ($phi_new | merge {$name: $val})
    }

    # normalize
    mut total = 0.0
    for name in $all_names { $total = $total + ($phi_new | get $name) }
    if $total > 0 {
      for name in $all_names {
        let v = ($phi_new | get $name)
        $phi_new = ($phi_new | merge {$name: ($v / $total)})
      }
    }

    mut delta = 0.0
    for name in $all_names {
      $delta = $delta + ((($phi_d | get $name) - ($phi_new | get $name)) | math abs)
    }

    if ($iter mod 5) == 0 or $iter == 1 {
      print $"  iteration ($iter): delta = ($delta | math round -p 8)"
    }
    $phi_d = $phi_new
    if $delta < 1e-8 { print $"  converged at iteration ($iter)"; break }
  }

  let phi_d = $phi_d

  # =================================================================
  # OPERATOR 2: SPRINGS (screened Laplacian equilibrium)
  # For each page: average diffusion focus of all direct neighbors
  # (both inbound and outbound). Approximates (L + μI)⁻¹ x₀
  # =================================================================
  print "S: springs — neighbor equilibrium..."

  mut phi_s = {}
  for name in $all_names {
    let in_neighbors = ($inbound | get $name)
    let out_neighbors = ($outbound | get $name)

    # collect unique neighbors
    mut neighbor_sum = 0.0
    mut neighbor_count = 0
    for nb in $in_neighbors {
      $neighbor_sum = $neighbor_sum + ($phi_d | get $nb)
      $neighbor_count = $neighbor_count + 1
    }
    for nb in $out_neighbors {
      $neighbor_sum = $neighbor_sum + ($phi_d | get $nb)
      $neighbor_count = $neighbor_count + 1
    }

    # screened: blend neighbor average with own diffusion (screening μ)
    let mu = 0.1
    let nb_avg = if $neighbor_count > 0 { $neighbor_sum / ($neighbor_count | into float) } else { 1.0 / $nf }
    let val = (1.0 - $mu) * $nb_avg + $mu * ($phi_d | get $name)
    $phi_s = ($phi_s | merge {$name: $val})
  }

  # normalize springs
  mut s_total = 0.0
  for name in $all_names { $s_total = $s_total + ($phi_s | get $name) }
  if $s_total > 0 {
    for name in $all_names {
      let v = ($phi_s | get $name)
      $phi_s = ($phi_s | merge {$name: ($v / $s_total)})
    }
  }
  let phi_s = $phi_s

  # =================================================================
  # OPERATOR 3: HEAT (multi-scale smoothing, 2-hop neighborhood)
  # Heat kernel e^{-τL} approximated as weighted 2-hop average.
  # hop-1 neighbors get weight e^{-τ}, hop-2 get weight e^{-2τ}
  # =================================================================
  print $"H: heat — 2-hop smoothing, tau=($tau)..."

  let w1 = ((-1.0 * $tau) | math exp)  # weight for 1-hop
  let w2 = ((-2.0 * $tau) | math exp)  # weight for 2-hop

  mut phi_h = {}
  for name in $all_names {
    let in_neighbors = ($inbound | get $name)
    let out_neighbors = ($outbound | get $name)

    # 1-hop: direct neighbors
    mut sum_1hop = 0.0
    mut count_1hop = 0
    mut hop1_set = []
    for nb in $in_neighbors {
      $sum_1hop = $sum_1hop + ($phi_d | get $nb)
      $count_1hop = $count_1hop + 1
      $hop1_set = ($hop1_set | append $nb)
    }
    for nb in $out_neighbors {
      $sum_1hop = $sum_1hop + ($phi_d | get $nb)
      $count_1hop = $count_1hop + 1
      $hop1_set = ($hop1_set | append $nb)
    }

    # 2-hop: neighbors of neighbors (sample — full would be too slow)
    mut sum_2hop = 0.0
    mut count_2hop = 0
    for nb in $hop1_set {
      let nb_in = ($inbound | get $nb)
      for nb2 in $nb_in {
        $sum_2hop = $sum_2hop + ($phi_d | get $nb2)
        $count_2hop = $count_2hop + 1
      }
    }

    let self_val = ($phi_d | get $name)
    let hop1_avg = if $count_1hop > 0 { $sum_1hop / ($count_1hop | into float) } else { 0.0 }
    let hop2_avg = if $count_2hop > 0 { $sum_2hop / ($count_2hop | into float) } else { 0.0 }

    # heat kernel: self + weighted 1-hop + weighted 2-hop
    let val = $self_val + $w1 * $hop1_avg + $w2 * $hop2_avg
    $phi_h = ($phi_h | merge {$name: $val})
  }

  # normalize heat
  mut h_total = 0.0
  for name in $all_names { $h_total = $h_total + ($phi_h | get $name) }
  if $h_total > 0 {
    for name in $all_names {
      let v = ($phi_h | get $name)
      $phi_h = ($phi_h | merge {$name: ($v / $h_total)})
    }
  }
  let phi_h = $phi_h

  # =================================================================
  # COMPOSITE: focus = norm(λd·D + λs·S + λh·H)
  # =================================================================
  print $"Composing: lambda_d=($lambda_d) lambda_s=($lambda_s) lambda_h=($lambda_h)"

  mut phi_focus = {}
  for name in $all_names {
    let d = ($phi_d | get $name)
    let s = ($phi_s | get $name)
    let h = ($phi_h | get $name)
    let val = $lambda_d * $d + $lambda_s * $s + $lambda_h * $h
    $phi_focus = ($phi_focus | merge {$name: $val})
  }

  # normalize composite
  mut f_total = 0.0
  for name in $all_names { $f_total = $f_total + ($phi_focus | get $name) }
  if $f_total > 0 {
    for name in $all_names {
      let v = ($phi_focus | get $name)
      $phi_focus = ($phi_focus | merge {$name: ($v / $f_total)})
    }
  }
  let phi_focus = $phi_focus

  # =================================================================
  # BUILD RESULTS
  # =================================================================
  let results = ($all_names | each {|name|
    let grav = ($inbound | get $name | length)
    let size_bytes = ($page_sizes | get $name)
    let out = ($out_count | get $name)
    let dens = if $size_bytes > 0 { ($out | into float) / ($size_bytes / 1024.0) } else { 0.0 }
    {
      name: $name
      file: ($page_files | get $name)
      diffusion: ($phi_d | get $name)
      springs: ($phi_s | get $name)
      heat: ($phi_h | get $name)
      focus: ($phi_focus | get $name)
      gravity: $grav
      density: $dens
    }
  } | sort-by focus -r)

  if $dry_run {
    print "\n=== TOP 50 by focus ==="
    let display = ($results | first 50 | each {|r| {
      name: $r.name
      D: ($r.diffusion * 1e6 | math round -p 0)
      S: ($r.springs * 1e6 | math round -p 0)
      H: ($r.heat * 1e6 | math round -p 0)
      focus: ($r.focus * 1e6 | math round -p 0)
      gravity: $r.gravity
      density: ($r.density | math round -p 1)
    }})
    print ($display | table)

    let total = ($results | get focus | math sum | math round -p 6)
    print $"\nTotal focus: ($total) [should be 1.0]"
    return
  }

  # =================================================================
  # WRITE TO FRONTMATTER
  # =================================================================
  print "Writing tri-kernel results to frontmatter..."

  let fields_to_clean = ["focus:" "diffusion:" "springs:" "heat:" "gravity:" "density:"]

  mut written = 0
  for row in $results {
    let f = $row.file
    let raw = (open --raw $f)
    if not ($raw | str starts-with "---") { continue }

    let lines = ($raw | lines)
    let fm_matches = ($lines | skip 1 | enumerate | where {|x| $x.item == "---"})
    if ($fm_matches | length) == 0 { continue }
    let fm_end = ($fm_matches | first | get index)
    let fm_lines = ($lines | skip 1 | first $fm_end)
    let body_lines = ($lines | skip ($fm_end + 2))

    # remove old tri-kernel fields
    let tk_prefixes = ["focus:" "diffusion:" "springs:" "heat:" "gravity:" "density:"]
    let clean_fm = ($fm_lines | where {|l| not ($tk_prefixes | any {|p| $l | str starts-with $p})})

    let density_str = ($row.density | math round -p 2 | into string)

    let new_fm = ($clean_fm
      | append $"diffusion: ($row.diffusion | into string)"
      | append $"springs: ($row.springs | into string)"
      | append $"heat: ($row.heat | into string)"
      | append $"focus: ($row.focus | into string)"
      | append $"gravity: ($row.gravity)"
      | append $"density: ($density_str)"
    )

    let new_content = (["---"] | append $new_fm | append ["---"] | append $body_lines | str join "\n")
    $new_content | save -f $f
    $written = $written + 1
  }

  print $"Written 6 fields to ($written) pages"
  print "\nTop 10 by focus:"
  print ($results | first 10 | each {|r| {
    name: $r.name
    D: ($r.diffusion * 1e6 | math round -p 0)
    S: ($r.springs * 1e6 | math round -p 0)
    H: ($r.heat * 1e6 | math round -p 0)
    focus: ($r.focus * 1e6 | math round -p 0)
    gravity: $r.gravity
    density: ($r.density | math round -p 1)
  }} | table)
}
