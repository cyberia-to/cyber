# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
def main [graph_path: string] {
    # auto-detect page directory: root/ → graph/ → pages/ (same as optica)
    let p = if ($graph_path | path join "root" | path exists) {
        $graph_path | path join "root"
    } else if ($graph_path | path join "graph" | path exists) {
        $graph_path | path join "graph"
    } else {
        $graph_path | path join "pages"
    }
    let files = (glob ($p | path join "*.md") | each {|f|
        let name = ($f | path basename | str replace ".md" "" | str replace "%3A" ":" | str replace "%2F" "/" | str replace "___" "/")
        let nl = ($name | str downcase)
        let tags = (try { open $f | lines | where ($it | str starts-with "tags::") | first | str downcase } catch { "" })
        {name: $name, nl: $nl, tags: $tags}
    })
    print $"total pages: ($files | length)"
    print ""

    let core_names = ["particle" "cyberlink" "neuron" "focus" "tri-kernel" "cyberank" "karma" "soft3" "vimputer" "cybergraph" "bootloader" "cybernomics" "prism" "aips" "bandwidth" "cip" "search" "oracle" "token" "cyb" "cyber" "bostrom" "cyberia" "coin" "uniq" "score" "badge" "record" "relevance machine" "consensus" "collective learning"]
    let itself = ($files | where {|pg| ($pg.nl | str starts-with "cyber/") or ($pg.nl | str starts-with "bostrom/") or ($pg.nl | str starts-with "cyb/") or ($pg.nl in $core_names)} | length)

    let math_names = ["graph theory" "linear algebra" "probability" "calculus" "information theory" "category theory" "game theory" "number theory" "topology" "logic" "entropy" "set theory" "isomorphism" "observation probability" "negentropy vs entropy" "logical clock"]
    let math = ($files | where {|pg| $pg.nl in $math_names} | length)

    let phys_names = ["mechanics" "thermodynamics" "electromagnetism" "quantum mechanics" "quantum" "relativity" "cosmology" "physics" "force" "momentum"]
    let physics_c = ($files | where {|pg| $pg.nl in $phys_names} | length)

    let chem_names = ["organic chemistry" "biochemistry" "periodic table" "chemistry" "chemical bond" "molecule"]
    let chem = ($files | where {|pg| ($pg.tags | str contains "compound") or ($pg.nl in $chem_names)} | length)

    let bio_names = ["biology" "taxonomy" "evolution" "ecology" "genetics" "neuroscience" "microbiology" "photosynthesis" "biome engineering" "biome" "forest" "mycelium" "gut microbiome" "sensor network" "dna repair mechanisms"]
    let bio = ($files | where {|pg| ($pg.tags | str contains "species") or ($pg.tags | str contains "genus") or ($pg.nl in $bio_names)} | length)

    let cs_names = ["computation" "cryptography" "distributed systems" "networking" "machine learning" "programming" "ipfs" "neural network" "future of computation" "nature of distributed computation" "distributed neural network" "distributed constraint optimization" "soft3 and machine learning" "cryptography and web3" "reality of foundation models"]
    let cs = ($files | where {|pg| $pg.nl in $cs_names} | length)

    let state_names = ["governance" "dao" "network state" "startup societies" "cyberia" "cyber state" "jurisdiction" "cyber nation" "cyberia vision" "start societies and network states" "network state with superintelligence" "add your network state" "add your startup society" "coordination"]
    let states = ($files | where {|pg| $pg.nl in $state_names} | length)

    let econ_names = ["microeconomics" "macroeconomics" "token economics" "game theory" "market" "staking" "liquidity" "bonding" "adaptive hybrid economics" "energy market" "new design"]
    let econ = ($files | where {|pg| ($pg.tags | str contains "cybernomics") or ($pg.nl in $econ_names)} | length)

    let people = ($files | where {|pg| $pg.tags | str contains "person"} | length)

    let mat_names = ["silicon" "lithium" "copper" "biochar" "cellulose" "lignin" "chitin" "graphene" "timber" "steel" "concrete" "roman concrete" "produce concrete" "bostrom/lithium" "dilithium" "lithium-ion battery"]
    let materials = ($files | where {|pg| $pg.nl in $mat_names} | length)

    let tok_names = ["cyb" "hydrogen" "boot" "volt" "ampere" "btc" "eth" "atom"]
    let tokens = ($files | where {|pg| ($pg.tags | str contains "ticker") or ($pg.nl in $tok_names)} | length)

    let en_names = ["energy" "energy autonomy" "energy market" "close energy loop" "solar" "wind" "wind turbine" "geothermal" "nuclear" "battery" "lithium-ion battery" "water battery" "photovoltaic" "biomass" "buy energy" "informational energy" "solar energy"]
    let energy_c = ($files | where {|pg| $pg.nl in $en_names} | length)

    let geo_names = ["geography" "climate" "climate zones" "ocean" "carbon cycle" "water cycle" "cyber valley" "cyber valley/districts" "cyber valley estate" "cyber valley/infrastructure" "cyber valley story" "cyber valley. 2025 reflection"]
    let geo = ($files | where {|pg| $pg.nl in $geo_names} | length)

    let body_names = ["anatomy" "health" "immune" "metabolism" "nutrition" "longevity" "immortality" "superhuman" "longevity and health" "gut microbiome"]
    let body = ($files | where {|pg| ($pg.tags | str contains "muscle") or ($pg.nl in $body_names)} | length)

    let cult_names = ["language" "writing" "alphabet" "culture" "permaculture" "neural language"]
    let culture = ($files | where {|pg| $pg.nl in $cult_names} | length)

    print ([
        {domain: "itself (protocol)" current: $itself}
        {domain: "mathematics" current: $math}
        {domain: "physics" current: $physics_c}
        {domain: "chemistry (compounds)" current: $chem}
        {domain: "biology (species+genus)" current: $bio}
        {domain: "computer science" current: $cs}
        {domain: "states & governance" current: $states}
        {domain: "economics" current: $econ}
        {domain: "people" current: $people}
        {domain: "materials & elements" current: $materials}
        {domain: "tokens & currencies" current: $tokens}
        {domain: "energy" current: $energy_c}
        {domain: "geography" current: $geo}
        {domain: "the body" current: $body}
        {domain: "culture & language" current: $culture}
    ] | table --expand)

    let total = $itself + $math + $physics_c + $chem + $bio + $cs + $states + $econ + $people + $materials + $tokens + $energy_c + $geo + $body + $culture
    print ""
    print $"sum across domains: ($total) \(pages may overlap\)"
}
