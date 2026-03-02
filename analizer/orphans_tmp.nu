let pages = (glob "/Users/mastercyb/git/cyber/pages/**/*.md" | each {|f| ls $f | first} | flatten)
let files = $pages
let link_data = ($files | each {|f|
    let content = (open $f.name)
    let found = ($content | parse --regex '\[\[([^\]]+)\]\]' | get capture0 | each {|l| $l | str downcase})
    {file: ($f.name | path basename | str replace ".md" ""), out_links: $found}
})
let all_links = ($link_data | get out_links | flatten)
let referenced = ($all_links | uniq)
let existing_pages = ($files | get name | each {|n| $n | path basename | str replace ".md" "" | str downcase})
let orphans = ($existing_pages | where {|p| $p not-in $referenced})
print ($orphans | length)
print "---"
print ($orphans | to text)
