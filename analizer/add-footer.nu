def main [graph_path: string] {
  let pages_subdir = if (($graph_path | path join "root") | path exists) { "root" } else if (($graph_path | path join "graph") | path exists) { "graph" } else { "pages" }
  let pages_dir = ($graph_path | path join $pages_subdir)

  let files = (glob $"($pages_dir)/**/*.md" | where {|f|
    let content = (try { open --raw $f } catch { "" })
    let has_core = ($content =~ "tags:.*core")
    let has_discover = ($content =~ "discover all")
    $has_core and (not $has_discover)
  })

  print $"found ($files | length) core pages without footer"

  $files | each {|f|
    let content = (open --raw $f | str replace -r '\s+$' '')
    let new_content = ($content + "\n\ndiscover all [[concepts]]\n")
    $new_content | save -f $f
    let stem = ($f | path parse | get stem)
    print $"  added footer: ($stem)"
  }

  print "done"
}
