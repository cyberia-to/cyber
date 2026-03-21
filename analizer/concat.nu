def main [graph_path: string, --output (-o): string = "", --full (-f)] {
  # always include graph pages
  mut files = (glob $"($graph_path)/root/**/*.md" | sort)

  if $full {
    # blog posts
    let blog = (glob $"($graph_path)/blog/*.md" | sort)
    $files = ($files | append $blog)

    # analizer scripts
    let scripts = (glob $"($graph_path)/analizer/*.nu" | sort)
    $files = ($files | append $scripts)

    # top-level config files
    let configs = (glob $"($graph_path)/*.md" | append (glob $"($graph_path)/*.toml") | sort)
    $files = ($files | append $configs)
  }

  let count = ($files | length)

  let body = ($files | each {|f|
    let rel = ($f | str replace $"($graph_path)/" "")
    let content = (open --raw $f | str trim)
    $"--- ($rel) ---\n($content)"
  } | str join "\n\n")

  let mode = if $full { "full (graph + blog + scripts + config)" } else { "graph only" }
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
