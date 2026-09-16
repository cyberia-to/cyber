# Validate filesystem Markdown links in an explicit source set.
def main [graph_path: string, --files: string] {
  let root = ($graph_path | path expand)
  let files = if $files == null {
    glob $"($root)/**/*.md" --exclude ["**/.git/**" "**/target/**" "**/build/**" "**/node_modules/**"]
  } else { $files | from json }
  mut issues = []
  mut checked = 0
  for file in $files {
    let file = (if ($file | str starts-with "/") { $file } else { $root | path join $file } | path expand)
    let links = (open --raw $file | parse --regex '\[[^\]]*\]\(([^\)\n]+)\)' | get capture0)
    for raw in $links {
      let target = ($raw | str trim | str trim --char '<' | str trim --char '>' | split row '#' | first)
      if $target == "" or $target =~ '^[A-Za-z][A-Za-z0-9+.-]*:' { continue }
      # Markdown links with a quoted title are uncommon in this graph; report
      # rather than silently interpreting an ambiguous local target.
      let decoded = (try { $target | url decode } catch { $target })
      let path = ($file | path dirname | path join $decoded | path expand)
      $checked = $checked + 1
      if not ($path | path exists) { $issues = ($issues | append {file: $file, target: $target}) }
    }
  }
  print $"Checked ($checked) local links in ($files | length) files"
  if ($issues | is-not-empty) {
    print ($issues | to json)
    error make {msg: $"($issues | length) broken local links"}
  }
}
