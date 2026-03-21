def main [graph_path: string, --output (-o): string = ""] {
  let files = (glob $"($graph_path)/root/**/*.md" | sort)
  let count = ($files | length)

  let body = ($files | each {|f|
    let rel = ($f | str replace $"($graph_path)/" "")
    let content = (open --raw $f | str trim)
    $"--- ($rel) ---\n($content)"
  } | str join "\n\n")

  let header = $"# Knowledge Graph: ($graph_path | path basename)\n# Pages: ($count)\n# Generated: (date now | format date '%Y-%m-%d')\n"
  let result = $"($header)\n($body)\n"

  if $output == "" {
    print $result
  } else {
    $result | save -f $output
    let size = (open --raw $output | str length)
    print $"Saved ($count) pages to ($output) — ($size / 1024 | math round -p 0) KB"
  }
}
