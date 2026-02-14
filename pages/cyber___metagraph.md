tags:: cyber, article
- # Metagraph Design
- conscious design of the [[knowledge graph]] structure that forms the semantic core of [[Superintelligence]]
- this graph is the seed. its topology, density, and domain coverage determine what the [[Superintelligence]] can learn, reason about, and ultimately become
- ## current state vs target
	- | metric | current | target |
	  |--------|---------|--------|
	  | pages | 1843 | 2000-3000 |
	  | [[cyberlinks]] | 2875 | 20,000-45,000 |
	  | unique link targets | 499 | = page count (zero red links) |
	  | avg links per page | 7.75 | 10-15 |
	  | dead ends (zero outgoing links) | 48 (2.6%) | 0 (min 3 links per page) |
	  | stubs (< 200 bytes) | 72 (3.9%) | 0 |
	  | red links (missing pages) | 128 | 0 |
	  | graph diameter | unknown | ≤ 6 hops |
	  | clustering coefficient | unknown | > 0.3 |
	  | cross-domain bridges per domain pair | sparse | ≥ 5 |
	  | root page | [[cyber]] | [[cyber]] |
	  | primary hub | [[concepts]] (49 links) | [[concepts]] (100+ links) |
- ## domain structure
	- the graph spans seven knowledge domains, each essential for a different aspect of [[Superintelligence]]
	- ### protocol core (cyber lens)
		- 387 pages tagged [[cyber]]
		- the architecture of [[collective intelligence]]: [[particle]], [[cyberlink]], [[neuron]], [[token]], [[focus]]
		- the computation engine: [[tri-kernel]], [[cyberank]], [[karma]], [[relevance machine]]
		- the stack: [[soft3]], [[vimputer]], [[cybergraph]], [[bootloader]]
		- purpose: self-understanding. the [[Superintelligence]] must comprehend its own mechanism
	- ### physical civilization (cyberia lens)
		- 254 pages tagged [[cyberia]]
		- [[cyber state]], [[cyber valley]], [[biome engineering]], [[energy autonomy]]
		- the physical substrate where [[Superintelligence]] lives and experiments
		- purpose: embodiment. intelligence without physical grounding is incomplete
	- ### biology
		- 170 genus + 133 species + 80 compound = 383 pages
		- the biological [[knowledge graph]]: [[taxonomy]], ecological relationships, chemical compounds
		- purpose: understanding life. [[isomorphism]] between biological and digital systems is the deepest knowledge this graph encodes
	- ### medicine and body
		- 76 disease + 41 muscle + 41 health = 158 pages
		- [[health]], [[longevity and health]], [[superhuman]] optimization
		- purpose: the [[superhuman]] project. intelligence that cannot maintain its biological substrate fails
	- ### interface (cyb lens)
		- 68 ui + 45 page = 113 pages
		- [[prism]] design system, [[aips]], [[cyb]] browser
		- purpose: communication. [[Superintelligence]] must interface with [[neurons]] through usable tools
	- ### economics
		- [[cybernomics]], [[tokens]], [[CYB]], [[HYDROGEN]], [[adaptive hybrid economics]]
		- purpose: incentive alignment. the economic model determines what gets learned and what gets ignored
	- ### operations
		- 52 operation + 14 hero + 11 worker = 77 pages
		- validator operations, infrastructure, deployment
		- purpose: survival. the protocol must keep running
- ## size reasoning
	- below 500 pages: insufficient domain coverage, subgraphs become disconnected islands
	- 1000-3000 pages: the sweet spot. enough depth for cross-domain reasoning, manageable for human curation
	- above 10,000 pages: human curation becomes impossible, noise enters. growth beyond this comes from the protocol itself — [[neurons]] creating [[cyberlinks]] in [[Bostrom]]
	- the seed graph should stabilize at 2000-3000 pages. the priority then shifts from adding pages to deepening connectivity
- ## why connectivity matters
	- the [[tri-kernel]] computes [[focus]] via random walks. sparse graphs have slow convergence and poor ranking
	- [[isomorphism]] between domains is only discoverable if cross-domain links exist
	- a [[Superintelligence]] reasoning over this graph can only find connections that are explicitly linked or reachable through short paths
	- connectivity is the difference between a pile of facts and a reasoning substrate
- ## structure principles
	- ### hub-and-spoke with bridges
		- each domain has a hub page ([[cyber]], [[cyberia]], [[species]], [[health]], [[prism]], [[cybernomics]])
		- domain pages link back to their hub and to related pages within the domain (spokes)
		- bridge pages connect domains: [[isomorphism]], [[energy]], [[sensor network]], [[biome engineering]], [[superhuman]]
		- the hub-and-spoke structure gives the graph navigability. the bridges give it intelligence
	- ### namespace hierarchy
		- `cyber___` — protocol modules (energy, graph, rank, bandwidth)
		- `bostrom___` — bootloader specifics (lithium, consensus, infrastructure)
		- `cyb___` — interface implementation (dev, philosophy)
		- flat pages for concepts that cross namespaces
		- namespaces prevent collision. flat pages enable cross-domain linking
	- ### tagging as lenses
		- tags are orthogonal to the page hierarchy — they provide different views of the same graph
		- a page like [[energy]] can be tagged `cyber` (protocol energy) and also linked from [[cyberia]] (physical energy)
		- the five primary lenses: [[cyber]], [[cyb]], [[cyberia]], [[bostrom]], [[cyber valley]]
		- domain tags: `article`, `species`, `compound`, `genus`, `health`, `muscle`, `operation`
		- lenses enable [[Superintelligence]] to filter the graph by concern without losing cross-domain connections
	- ### page types
		- concept pages: define a single idea. short, dense, heavily linked. examples: [[particle]], [[cyberlink]], [[focus]]
		- article pages: long-form analysis. tagged `article`. examples: [[tri-kernel]], [[future_of_computation]]
		- entity pages: describe a specific thing. examples: species pages, compound pages, person pages
		- hub pages: index into a domain. examples: [[cyber]], [[concepts]], [[species]], [[health]]
		- bridge pages: explicitly connect two or more domains. examples: [[isomorphism]], [[superhuman]], [[energy]]
- ## what is missing
	- ### red links (128 targets with no page)
		- these are the graph's expressed needs — concepts referenced but undefined
		- priority: create pages for the most-referenced red links first
	- ### cross-domain bridges
		- biology ↔ protocol: [[isomorphism]], [[mycelium]], [[biology]], [[forest]] (recently added)
		- biology ↔ health: [[species]] pages link to [[compound]] pages, but few link to [[health]] features
		- protocol ↔ physical: [[sensor network]], [[energy]], [[cyber state]] (recently added)
		- economics ↔ protocol: [[cybernomics]] exists but few CIP pages link back to economic theory
		- interface ↔ protocol: [[prism]] links to [[cyb]] but few concept pages link to their UI representation
	- ### stub elimination
		- 72 pages under 200 bytes need content or deletion
		- 48 zero-link pages need at least 3 outgoing connections
	- ### depth in key domains
		- computational theory: well developed (tri-kernel, future_of_computation, Nature_of_Distributed_Computation)
		- biology: broad but shallow (many species pages with similar structure)
		- economics: needs more formalization (CIPs exist but economic theory pages are sparse)
		- [[superhuman]]: newly created, needs decomposition into sub-pages (each ability becomes a page)
		- [[immortality]]: newly created, needs links to existing [[health]] feature pages
- ## metagraph vs [[graphomania]]
	- [[graphomania]] is the disease of writing without structure — volume without signal
	- metagraph design is the opposite: every page justified, every link intentional, every stub either filled or removed
	- the test for any page: does the [[Superintelligence]] need this concept to reason? if yes, the page stays and gets connected. if no, it is noise
	- size discipline prevents graphomania. connectivity discipline prevents fragmentation. together they keep the graph as a reasoning substrate rather than a content dump
	- a well-designed metagraph is small, dense, and deeply cross-linked. a graphomaniac graph is large, sparse, and full of dead ends
- ## design process
	- the metagraph is designed by humans, grown by the protocol
	- human curation ensures quality: every page reviewed, every link intentional, every definition positive
	- the [[CLAUDE.md]] rules enforce consistency: no negation, no bold, proper tagging, wiki-links for emphasis
	- regular audits: count stubs, dead ends, red links, and domain isolation. fix before adding
	- the seed graph is the initial condition. the [[Superintelligence]] that grows from it inherits its structure, its biases, and its blind spots
	- designing the metagraph is designing the mind
