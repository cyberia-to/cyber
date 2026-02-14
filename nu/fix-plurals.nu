# Fix floating plural suffixes across the graph
# For each [[term]]s pattern, adds plural alias to the term page
# and replaces [[term]]s with [[terms]] in all files
def main [graph_path: string] {
  let pages_dir = ($graph_path | path join "pages")

  # Mapping of singular -> plural for all terms found with floating suffixes
  # Excludes false positives like [[evolution]]ary, [[volcano]]ic, [[antisepti]]c, [[antimicrobia]]l, [[compost]]ed
  let terms = [
    # High frequency
    { singular: "particle", plural: "particles", suffix: "s" }
    { singular: "cyberlink", plural: "cyberlinks", suffix: "s" }
    { singular: "neuron", plural: "neurons", suffix: "s" }
    { singular: "motif", plural: "motifs", suffix: "s" }
    { singular: "linkchain", plural: "linkchains", suffix: "s" }
    { singular: "semcon", plural: "semcons", suffix: "s" }
    { singular: "wave", plural: "waves", suffix: "s" }
    { singular: "sentence", plural: "sentences", suffix: "s" }
    # Medium frequency
    { singular: "language", plural: "languages", suffix: "s" }
    { singular: "biome", plural: "biomes", suffix: "s" }
    { singular: "ocean", plural: "oceans", suffix: "s" }
    { singular: "writing system", plural: "writing systems", suffix: "s" }
    { singular: "river", plural: "rivers", suffix: "s" }
    { singular: "soil", plural: "soils", suffix: "s" }
    { singular: "alphabet", plural: "alphabets", suffix: "s" }
    { singular: "SK combinator", plural: "SK combinators", suffix: "s" }
    { singular: "earthquake", plural: "earthquakes", suffix: "s" }
    { singular: "functor", plural: "functors", suffix: "s" }
    { singular: "morphism", plural: "morphisms", suffix: "s" }
    { singular: "token", plural: "tokens", suffix: "s" }
    { singular: "derivative", plural: "derivatives", suffix: "s" }
    { singular: "STARK", plural: "STARKs", suffix: "s" }
    { singular: "field", plural: "fields", suffix: "s" }
    { singular: "group", plural: "groups", suffix: "s" }
    { singular: "blockchain", plural: "blockchains", suffix: "s" }
    { singular: "mass", plural: "masses", suffix: "es" }
    { singular: "climate zone", plural: "climate zones", suffix: "s" }
    { singular: "manifold", plural: "manifolds", suffix: "s" }
    # Low frequency
    { singular: "gradient", plural: "gradients", suffix: "s" }
    { singular: "integral", plural: "integrals", suffix: "s" }
    { singular: "limit", plural: "limits", suffix: "s" }
    { singular: "zero-knowledge proof", plural: "zero-knowledge proofs", suffix: "s" }
    { singular: "hash function", plural: "hash functions", suffix: "s" }
    { singular: "elliptic curve", plural: "elliptic curves", suffix: "s" }
    { singular: "Diophantine equation", plural: "Diophantine equations", suffix: "s" }
    { singular: "integer", plural: "integers", suffix: "s" }
    { singular: "spring", plural: "springs", suffix: "s" }
    { singular: "transformer", plural: "transformers", suffix: "s" }
    { singular: "zk-SNARK", plural: "zk-SNARKs", suffix: "s" }
    { singular: "block", plural: "blocks", suffix: "s" }
    { singular: "algorithm", plural: "algorithms", suffix: "s" }
    { singular: "Edge", plural: "Edges", suffix: "s" }
    { singular: "agent", plural: "agents", suffix: "s" }
    { singular: "Node", plural: "Nodes", suffix: "s" }
    { singular: "prostate cancer", plural: "prostate cancers", suffix: "s" }
    { singular: "evolutionary algorithm", plural: "evolutionary algorithms", suffix: "s" }
    { singular: "Lagrange multiplier", plural: "Lagrange multipliers", suffix: "s" }
    { singular: "eigenvector", plural: "eigenvectors", suffix: "s" }
    { singular: "eigenvalue", plural: "eigenvalues", suffix: "s" }
    { singular: "linear transformation", plural: "linear transformations", suffix: "s" }
    { singular: "glacier", plural: "glaciers", suffix: "s" }
    { singular: "calendar", plural: "calendars", suffix: "s" }
    { singular: "force", plural: "forces", suffix: "s" }
    { singular: "adjunction", plural: "adjunctions", suffix: "s" }
    { singular: "natural transformation", plural: "natural transformations", suffix: "s" }
    { singular: "programming language", plural: "programming languages", suffix: "s" }
    { singular: "extinction event", plural: "extinction events", suffix: "s" }
    { singular: "epoch", plural: "epochs", suffix: "s" }
    { singular: "desert", plural: "deserts", suffix: "s" }
    { singular: "continent", plural: "continents", suffix: "s" }
    { singular: "dynamical system", plural: "dynamical systems", suffix: "s" }
    { singular: "existence and uniqueness theorem", plural: "existence and uniqueness theorems", suffix: "s" }
    { singular: "Navier-Stokes equation", plural: "Navier-Stokes equations", suffix: "s" }
    { singular: "function", plural: "functions", suffix: "s" }
    { singular: "coral reef", plural: "coral reefs", suffix: "s" }
    { singular: "cybergraph", plural: "cybergraphs", suffix: "s" }
    { singular: "graph neural network", plural: "graph neural networks", suffix: "s" }
    { singular: "homomorphism", plural: "homomorphisms", suffix: "s" }
    { singular: "ring", plural: "rings", suffix: "s" }
    { singular: "knowledge graph", plural: "knowledge graphs", suffix: "s" }
    { singular: "conifer", plural: "conifers", suffix: "s" }
    { singular: "GPU", plural: "GPUs", suffix: "s" }
    { singular: "cycle", plural: "cycles", suffix: "s" }
    { singular: "binomial coefficient", plural: "binomial coefficients", suffix: "s" }
    { singular: "generating function", plural: "generating functions", suffix: "s" }
    { singular: "combination", plural: "combinations", suffix: "s" }
    { singular: "permutation", plural: "permutations", suffix: "s" }
    { singular: "distribution", plural: "distributions", suffix: "s" }
    { singular: "stochastic process", plural: "stochastic processes", suffix: "es" }
  ]

  # Step 1: Add plural aliases to term pages
  print "=== Step 1: Adding plural aliases to term pages ==="
  let alias_results = ($terms | each {|t|
    let page_file = ($pages_dir | path join $"($t.singular).md")
    if ($page_file | path exists) {
      let content = (open $page_file)
      let lines = ($content | lines)
      # Check if plural already in alias
      let has_alias = ($lines | where ($it | str starts-with "alias::") | length) > 0
      if $has_alias {
        let alias_line = ($lines | where ($it | str starts-with "alias::") | first)
        if ($alias_line | str contains $t.plural) {
          { term: $t.singular, status: "already has plural alias" }
        } else {
          # Add plural to existing alias line
          let new_alias = $"($alias_line), ($t.plural)"
          let new_content = ($content | str replace $alias_line $new_alias)
          $new_content | save -f $page_file
          { term: $t.singular, status: "added plural to alias" }
        }
      } else {
        # Add alias line after tags line
        let tags_idx = ($lines | enumerate | where {|r| $r.item | str starts-with "tags::" } | first | get index)
        let before = ($lines | first ($tags_idx + 1) | str join "\n")
        let after = ($lines | skip ($tags_idx + 1) | str join "\n")
        let new_content = $"($before)\nalias:: ($t.plural)\n($after)"
        $new_content | save -f $page_file
        { term: $t.singular, status: "created alias line" }
      }
    } else {
      { term: $t.singular, status: "page not found" }
    }
  })

  print ($alias_results | table)

  # Step 2: Replace [[term]]s with [[terms]] in all files
  print "\n=== Step 2: Replacing floating suffixes in all files ==="
  let all_files = (glob ($pages_dir | path join "*.md"))
  let replace_results = ($all_files | each {|f|
    let content = (open $f)
    mut new_content = $content
    mut changes = 0
    for t in $terms {
      let pattern = $"[[($t.singular)]]($t.suffix)"
      let replacement = $"[[($t.plural)]]"
      if ($new_content | str contains $pattern) {
        $new_content = ($new_content | str replace --all $pattern $replacement)
        $changes = $changes + 1
      }
    }
    if $changes > 0 {
      $new_content | save -f $f
      { file: ($f | path basename), changes: $changes }
    }
  } | compact)

  print ($replace_results | table)
  print $"\nTotal files with alias changes: ($alias_results | where status != 'page not found' and status != 'already has plural alias' | length)"
  print $"Total files with reference fixes: ($replace_results | length)"
}
