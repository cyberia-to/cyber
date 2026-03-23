# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
def main [graph_path: string, --output (-o): string = "", --full (-f), --subgraphs (-s)] {
  mut files = (glob $"($graph_path)/root/**/*.md" | sort)

  if $full {
    let blog = (glob $"($graph_path)/blog/*.md" | sort)
    $files = ($files | append $blog)
    let scripts = (glob $"($graph_path)/analizer/*.nu" | sort)
    $files = ($files | append $scripts)
    let configs = (glob $"($graph_path)/*.md" | append (glob $"($graph_path)/*.toml") | sort)
    $files = ($files | append $configs)
  }

  if $subgraphs {
    let git_root = ($graph_path | path dirname)
    let repos = [hemera zheng nebu nox bbg cybernode mudra trident optica]
    for repo in $repos {
      let repo_path = $"($git_root)/($repo)"
      if ($repo_path | path exists) {
        # try root/ first, then graph/, then pages/, fallback to all .md
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
        }
        $files = ($files | append ($md | sort))
      }
    }
  }

  let count = ($files | length)

  let body = ($files | each {|f|
    # try to make path relative to git root for subgraphs
    let rel = if ($f | str starts-with $graph_path) {
      $f | str replace $"($graph_path)/" ""
    } else {
      let git_root = ($graph_path | path dirname)
      $f | str replace $"($git_root)/" ""
    }
    let content = (open --raw $f | str trim)
    $"--- ($rel) ---\n($content)"
  } | str join "\n\n")

  mut flags = []
  if $full { $flags = ($flags | append "full") }
  if $subgraphs { $flags = ($flags | append "subgraphs") }
  let mode = if ($flags | length) == 0 { "graph only" } else { $flags | str join " + " }
  let header = $"# Knowledge Graph: ($graph_path | path basename)\n# Mode: ($mode)\n# Files: ($count)\n# Generated: (date now | format date '%Y-%m-%d')\n"
  let result = $"($header)\n($body)\n"

  if $output == "" {
    print $result
  } else {
    $result | save -f $output
    let size = (open --raw $output | str length)
    print $"Saved ($count) files to ($output) — ($size / 1024 | math round -p 0) KB"
  }
}
