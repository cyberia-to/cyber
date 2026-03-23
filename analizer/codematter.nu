# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
# codematter.nu — add comment-frontmatter to code files, integrating them into the cybergraph
#
# Adds YAML frontmatter as comments to source files (.rs, .nu, .toml, .py, .sh, .yml)
# so tri-kernel can index them alongside .md pages.
#
# Format by language:
#   .rs:           // ---\n// tags: ...\n// ---
#   .nu, .py, .sh: # ---\n# tags: ...\n# ---
#   .toml, .yml:   # ---\n# tags: ...\n# ---
#
# JSON files are skipped (no comment syntax).
#
# Usage:
#   nu analizer/codematter.nu ~/git/cyber                    # cyber repo only
#   nu analizer/codematter.nu ~/git/cyber --subgraphs        # include all subgraph repos
#   nu analizer/codematter.nu ~/git/cyber --dry-run           # show what would be added
#   nu analizer/codematter.nu ~/git/cyber --subgraphs --stats # count only, no changes

# repo → crystal-domain mapping
const DOMAIN_MAP = {
  hemera: "comp"
  zheng: "comp"
  nebu: "comp"
  nox: "comp"
  bbg: "cyber"
  cybernode: "cyber"
  mudra: "comp"
  trident: "comp"
  optica: "comp"
  context: "cyber"
  cw-cyber: "cyber"
  cyber: "cyber"
}

# extension → comment prefix
const COMMENT_MAP = {
  rs: "//"
  nu: "#"
  py: "#"
  sh: "#"
  toml: "#"
  yml: "#"
  yaml: "#"
}

# extensions to process
const CODE_EXTENSIONS = ["rs" "nu" "py" "sh" "toml" "yml" "yaml"]

# directories to skip
const SKIP_DIRS = ["target" "node_modules" "build" ".git" ".claude" "public"]

def has-codematter [content: string, ext: string] {
  let prefix = ($COMMENT_MAP | get -o $ext | default "#")
  let marker = $"($prefix) ---"
  $content | str starts-with $marker
}

def make-codematter [repo_name: string, file_path: string, ext: string] {
  let prefix = ($COMMENT_MAP | get -o $ext | default "#")
  let domain = ($DOMAIN_MAP | get -o $repo_name | default "comp")
  let lang_tag = (match $ext {
    "rs" => "rust"
    "nu" => "nushell"
    "py" => "python"
    "sh" => "shell"
    "toml" => "config"
    "yml" => "config"
    "yaml" => "config"
    _ => $ext
  })

  # extract filename without extension as page identity
  let fname = ($file_path | path basename | str replace $".($ext)$" "")

  [
    $"($prefix) ---"
    $"($prefix) tags: ($repo_name), ($lang_tag)"
    $"($prefix) crystal-type: source"
    $"($prefix) crystal-domain: ($domain)"
    $"($prefix) ---"
  ] | str join "\n"
}

def collect-code-files [dir: string] {
  let all = (glob $"($dir)/**/*" | where {|f|
    let p = ($f | str replace $dir "")
    # skip excluded directories
    not ($SKIP_DIRS | any {|skip| $p =~ $"/($skip)/"})
  } | where {|f|
    # only files with known extensions
    let ext = ($f | path parse | get extension | default "")
    $ext in $CODE_EXTENSIONS
  } | where {|f|
    # skip lock files and generated files
    let name = ($f | path basename)
    not ($name =~ "lock" or $name =~ "generated" or $name == "Cargo.lock")
  })
  $all
}

def process-file [file: string, repo_name: string, dry_run: bool] {
  let ext = ($file | path parse | get extension | default "")
  let content = (open --raw $file)

  if (has-codematter $content $ext) {
    return { status: "skip", file: $file }
  }

  let header = (make-codematter $repo_name $file $ext)
  let new_content = $"($header)\n($content)"

  if $dry_run {
    return { status: "would-add", file: $file }
  }

  $new_content | save -f $file
  { status: "added", file: $file }
}

def main [
  graph_path: string,
  --subgraphs (-s),
  --dry-run (-d),
  --stats,              # count only
] {
  mut repos = [{ name: "cyber", path: $graph_path }]

  if $subgraphs {
    let git_root = ($graph_path | path dirname)
    let sub_repos = [hemera zheng nebu nox bbg cybernode mudra trident optica context cw-cyber]
    for repo in $sub_repos {
      let repo_path = $"($git_root)/($repo)"
      if ($repo_path | path exists) {
        $repos = ($repos | append { name: $repo, path: $repo_path })
      }
    }
  }

  mut total_files = 0
  mut total_added = 0
  mut total_skip = 0
  mut by_ext = {}

  for repo in $repos {
    let files = (collect-code-files $repo.path)

    if $stats {
      let ext_counts = ($files | each {|f|
        $f | path parse | get extension | default "?"
      } | uniq -c | sort-by count -r)
      print $"($repo.name): ($files | length) code files"
      if ($ext_counts | length) > 0 {
        print ($ext_counts | table)
      }
      $total_files = $total_files + ($files | length)
      continue
    }

    let results = ($files | each {|f| process-file $f $repo.name $dry_run})
    let added = ($results | where status == "added" or status == "would-add" | length)
    let skipped = ($results | where status == "skip" | length)

    if $added > 0 or $skipped > 0 {
      let verb = if $dry_run { "would add" } else { "added" }
      print $"($repo.name): ($verb) ($added), skipped ($skipped) \(already have frontmatter\)"
    }

    $total_files = $total_files + ($files | length)
    $total_added = $total_added + $added
    $total_skip = $total_skip + $skipped
  }

  print $"\nTotal: ($total_files) files, ($total_added) to process, ($total_skip) already done"
}
