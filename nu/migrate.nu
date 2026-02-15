# migrate.nu — Convert Logseq graph to pure markdown
# Usage: nu nu/migrate.nu ~/git/cyber
#
# What it does:
# 1. Converts property:: value → YAML frontmatter
# 2. Normalizes outliner bullets → standard markdown
# 3. Strips Logseq block-level metadata from body
# 4. Renames ___-encoded files to directory namespaces
# 5. Decodes percent-encoded filenames

def main [graph_path: string] {
    let graph = ($graph_path | path expand)
    let pages_dir = ($graph | path join "pages")
    let journals_dir = ($graph | path join "journals")

    print $"Migrating graph at ($graph)"
    print ""

    # Step 1+2+3: Convert pages
    let page_files = (glob ($pages_dir | path join "*.md"))
    let page_count = ($page_files | length)
    print $"Found ($page_count) pages"

    mut converted = 0
    mut skipped = 0
    for file in $page_files {
        let result = (convert_file $file)
        if $result == "converted" {
            $converted = $converted + 1
        } else {
            $skipped = $skipped + 1
        }
    }
    print $"  Converted: ($converted), Already migrated: ($skipped)"

    # Convert journals
    if ($journals_dir | path exists) {
        let journal_files = (glob ($journals_dir | path join "*.md"))
        let journal_count = ($journal_files | length)
        print $"\nFound ($journal_count) journals"

        mut j_converted = 0
        mut j_skipped = 0
        for file in $journal_files {
            let result = (convert_file $file)
            if $result == "converted" {
                $j_converted = $j_converted + 1
            } else {
                $j_skipped = $j_skipped + 1
            }
        }
        print $"  Converted: ($j_converted), Already migrated: ($j_skipped)"
    }

    # Step 4: Move ___-encoded files to directories
    print "\nMoving ___-encoded files to directories..."
    let ns_files = (glob ($pages_dir | path join "*___*"))
    let ns_count = ($ns_files | length)
    print $"  Found ($ns_count) namespaced files"

    for file in $ns_files {
        move_namespaced_file $file $pages_dir
    }

    # Step 5: Decode percent-encoded filenames
    print "\nDecoding percent-encoded filenames..."
    let pct_files = (glob ($pages_dir | path join "*%*"))
    let pct_count = ($pct_files | length)
    print $"  Found ($pct_count) percent-encoded files"

    for file in $pct_files {
        decode_filename $file
    }

    print "\nDone!"
}

# Convert a single file: properties → frontmatter, outliner → prose
def convert_file [file: string]: nothing -> string {
    let content = (open --raw $file)

    # Skip if already migrated (starts with ---)
    if ($content | str starts-with "---\n") or ($content | str starts-with "---\r\n") {
        return "skipped"
    }

    let lines = ($content | lines)

    # Extract properties from top of file
    let extract = (extract_properties $lines)
    let props = $extract.properties
    let body_lines = $extract.body

    # Build YAML frontmatter
    let frontmatter = (build_frontmatter $props)

    # Normalize body (outliner → prose)
    let body = (normalize_body $body_lines)

    # Write result
    let result = if ($frontmatter | is-empty) {
        $body
    } else {
        $"---\n($frontmatter)---\n($body)"
    }

    $result | save --force --raw $file
    return "converted"
}

# Extract property:: value lines from top of file
# Returns: { properties: list<{key, value}>, body: list<string> }
def extract_properties [lines: list<string>]: nothing -> record {
    mut props = []
    mut body = []
    mut in_props = true
    mut found_any = false

    for line in $lines {
        if $in_props {
            # Strip leading "- " if present (Logseq wraps properties in bullets)
            let check = if ($line | str starts-with "- ") {
                $line | str substring 2..
            } else {
                $line
            }
            let trimmed = ($check | str trim)

            if ($trimmed | is-empty) {
                if $found_any {
                    $in_props = false
                }
                # Don't add empty line between props and body
            } else if ($trimmed =~ '^[a-zA-Z_-]+::\s*') {
                # It's a property line
                let parts = ($trimmed | split row "::" | each { str trim })
                let key = ($parts | first)
                let value = if ($parts | length) > 1 {
                    $parts | skip 1 | str join ":: "
                } else {
                    ""
                }
                $props = ($props | append {key: $key, value: $value})
                $found_any = true
            } else {
                # Non-property line — end of properties
                $in_props = false
                $body = ($body | append $line)
            }
        } else {
            $body = ($body | append $line)
        }
    }

    {properties: $props, body: $body}
}

# Build YAML frontmatter string from property list
def build_frontmatter [props: list<record<key: string, value: string>>]: nothing -> string {
    if ($props | is-empty) {
        return ""
    }

    mut lines = []
    for prop in $props {
        let key = $prop.key
        mut value = $prop.value

        # Strip [[wikilinks]] from tag/alias values
        $value = ($value | str replace --all "[[" "" | str replace --all "]]" "")

        # Determine if value needs quoting
        let needs_quote = (value_needs_yaml_quoting $value)

        let yaml_line = if ($value | is-empty) {
            $"($key):"
        } else if $needs_quote {
            # Use double quotes, escape internal quotes
            let escaped = ($value | str replace --all '"' '\"')
            $"($key): \"($escaped)\""
        } else {
            $"($key): ($value)"
        }

        $lines = ($lines | append $yaml_line)
    }

    ($lines | str join "\n") + "\n"
}

# Check if a YAML value needs quoting
def value_needs_yaml_quoting [value: string]: nothing -> bool {
    if ($value | is-empty) { return false }

    # Values that look like YAML special types
    let lower = ($value | str downcase)
    if $lower in ["true", "false", "yes", "no", "on", "off", "null", "~"] {
        return true
    }

    # Values that start with special YAML chars
    let first = ($value | str substring 0..1)
    if $first in ["{", "}", "[", "]", "&", "*", "?", "|", "-", "<", ">", "=", "!", "%", "@", "`", "'", '"'] {
        return true
    }

    # Values containing : followed by space, or # preceded by space
    if ($value =~ ': ') or ($value =~ ' #') {
        return true
    }

    # Values that are pure numbers (would be parsed as int/float)
    if ($value =~ '^\d+$') or ($value =~ '^\d+\.\d+$') {
        return true
    }

    # Values starting with 0x, 0o, 0b (YAML numeric literals)
    if ($value =~ '^0[xXoObB]') {
        return true
    }

    false
}

# Normalize outliner body content to standard markdown
def normalize_body [lines: list<string>]: nothing -> string {
    # Check if this is already prose (not outliner format)
    let non_empty = ($lines | where { |l| ($l | str trim) != "" } | first 10)
    let bullet_count = ($non_empty | where { |l| ($l =~ '^\s*- ') } | length)
    let total = ($non_empty | length)

    let is_outliner = if $total == 0 {
        false
    } else {
        ($bullet_count / $total) > 0.5
    }

    if not $is_outliner {
        # Already prose — just strip block-level metadata
        let cleaned = (strip_block_metadata $lines)
        return ($cleaned | str join "\n")
    }

    # Outliner normalization
    mut output = []
    mut i = 0
    mut in_code_block = false
    let line_count = ($lines | length)

    while $i < $line_count {
        let line = ($lines | get $i)

        # Track code blocks — preserve verbatim
        if ($line =~ '^\s*-?\s*```') or (not $in_code_block and ($line =~ '```')) {
            if $in_code_block {
                $in_code_block = false
                $output = ($output | append $line)
            } else {
                $in_code_block = true
                # Strip bullet prefix from code fence
                let stripped = ($line | str replace --regex '^\s*- ' '')
                $output = ($output | append $stripped)
            }
            $i = $i + 1
            continue
        }

        if $in_code_block {
            $output = ($output | append $line)
            $i = $i + 1
            continue
        }

        # Skip block-level metadata lines
        if (is_block_metadata $line) {
            $i = $i + 1
            continue
        }

        # Skip empty bullets "- " or "-"
        if ($line | str trim) in ["- ", "-"] {
            $i = $i + 1
            continue
        }

        # Empty line
        if ($line | str trim | is-empty) {
            $output = ($output | append "")
            $i = $i + 1
            continue
        }

        # Top-level bullet: starts with "- " (no leading whitespace)
        if ($line =~ '^- ') {
            let content = ($line | str substring 2..)

            # Heading: - ## Title → ## Title
            if ($content =~ '^#{1,6}\s') {
                $output = ($output | append "")
                $output = ($output | append $content)
                $output = ($output | append "")
                $i = $i + 1
                continue
            }

            # Check if next line is a sub-bullet (indented bullet)
            let has_children = if ($i + 1) < $line_count {
                let next = ($lines | get ($i + 1))
                ($next =~ '^\s+- ') or ($next =~ '^\t- ')
            } else {
                false
            }

            if $has_children {
                # Parent with children: parent becomes paragraph, children become list
                $output = ($output | append "")
                $output = ($output | append $content)
                $output = ($output | append "")
                $i = $i + 1

                # Collect children
                while $i < $line_count {
                    let child = ($lines | get $i)
                    if ($child =~ '^\s+- ') or ($child =~ '^\t+- ') {
                        # Reduce indent by one level (strip first tab or 2 spaces)
                        let stripped = if ($child | str starts-with "\t") {
                            $child | str substring 1..
                        } else if ($child | str starts-with "  ") {
                            $child | str substring 2..
                        } else {
                            $child
                        }
                        # Skip metadata in children too
                        if not (is_block_metadata $stripped) {
                            $output = ($output | append $stripped)
                        }
                    } else if ($child | str trim | is-empty) {
                        $output = ($output | append "")
                        $i = $i + 1
                        continue
                    } else if ($child =~ '^\s+[^-]') {
                        # Continuation line (indented but not a bullet)
                        # Reduce indent
                        let stripped = if ($child | str starts-with "\t") {
                            $child | str substring 1..
                        } else if ($child | str starts-with "  ") {
                            $child | str substring 2..
                        } else {
                            $child
                        }
                        if not (is_block_metadata $stripped) {
                            $output = ($output | append $stripped)
                        }
                    } else {
                        break
                    }
                    $i = $i + 1
                }
            } else {
                # Single top-level bullet → paragraph
                $output = ($output | append "")
                $output = ($output | append $content)
                $i = $i + 1
            }
        } else if ($line =~ '^\s+- ') or ($line =~ '^\t+- ') {
            # Orphan sub-bullet (no parent) — reduce indent and keep as list
            let stripped = if ($line | str starts-with "\t") {
                $line | str substring 1..
            } else if ($line | str starts-with "  ") {
                $line | str substring 2..
            } else {
                $line
            }
            if not (is_block_metadata $stripped) {
                $output = ($output | append $stripped)
            }
            $i = $i + 1
        } else {
            # Non-bullet line (continuation, table row, etc.)
            if not (is_block_metadata $line) {
                $output = ($output | append $line)
            }
            $i = $i + 1
        }
    }

    # Collapse multiple blank lines
    let result = ($output | str join "\n")
    $result | str replace --all --regex '\n{3,}' "\n\n" | str trim
}

# Check if a line is Logseq block-level metadata that should be stripped
def is_block_metadata [line: string]: nothing -> bool {
    let trimmed = ($line | str trim)
    if ($trimmed =~ '^collapsed::\s') { return true }
    if ($trimmed =~ '^id::\s[0-9a-f]') { return true }
    if ($trimmed =~ '^query-properties::') { return true }
    if ($trimmed =~ '^query-table::') { return true }
    if ($trimmed =~ '^query-sort-by::') { return true }
    if ($trimmed =~ '^query-sort-desc::') { return true }
    false
}

# Strip block-level metadata from lines (for prose pages)
def strip_block_metadata [lines: list<string>]: nothing -> list<string> {
    $lines | where { |l| not (is_block_metadata $l) }
}

# Move a ___-encoded filename to directory structure
def move_namespaced_file [file: string, pages_dir: string] {
    let basename = ($file | path basename)
    let name = ($basename | str replace '.md' '')

    # Split on ___ to get path segments
    let segments = ($name | split row "___")

    if ($segments | length) < 2 {
        return
    }

    # Build target path
    let dir_parts = ($segments | drop 1 | length)
    let parent_segments = ($segments | drop nth (($segments | length) - 1))
    let leaf = ($segments | last)

    let target_dir = ($parent_segments | each { |s| $s | str trim } | path join)
    let target_dir_full = ($pages_dir | path join $target_dir)
    let target_file = ($target_dir_full | path join $"($leaf | str trim).md")

    # Create directory
    mkdir $target_dir_full

    # Move file
    mv $file $target_file
    print $"  ($basename) → ($target_dir)/($leaf | str trim).md"
}

# Decode percent-encoded filename
def decode_filename [file: string] {
    let dir = ($file | path dirname)
    let basename = ($file | path basename)

    # Simple percent decoding for common cases
    mut decoded = $basename
    $decoded = ($decoded | str replace --all "%2E" ".")
    $decoded = ($decoded | str replace --all "%2e" ".")
    $decoded = ($decoded | str replace --all "%3A" ":")
    $decoded = ($decoded | str replace --all "%3a" ":")
    $decoded = ($decoded | str replace --all "%3F" "?")
    $decoded = ($decoded | str replace --all "%3f" "?")
    $decoded = ($decoded | str replace --all "%23" "#")
    $decoded = ($decoded | str replace --all "%2F" "/")
    $decoded = ($decoded | str replace --all "%2f" "/")

    if $decoded != $basename {
        let target = ($dir | path join $decoded)
        mv $file $target
        print $"  ($basename) → ($decoded)"
    }
}
