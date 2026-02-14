alias:: neural language, .nl
tags:: cyber
whitepaper:: [[Neural Language for Superintelligence]]
- semantic [[language]] for [[neuron]]s over the [[cybergraph]]
- convergent successor for both formal and natural [[language]]s
- meaning is defined by [[cyberlink]]s — structure emerges from how agents link [[particle]]s
- together with [[cybergraph]] and [[truth machine]] forms the foundation of [[soft3]]
- the language of [[collective intelligence]]: meaning emerges from how many [[neuron]]s independently structure [[knowledge]]
- ## why a new language
	- formal [[language]]s ([[type theory]], [[programming language]]s) achieve precision through rigid syntax but cannot scale to 10¹⁵ [[particle]]s — [[Gödel]] proved no sufficiently powerful formal system can be both complete and consistent
	- natural [[language]]s solve expressiveness through ambiguity but are computationally intractable for precise reasoning
	- neural language collapses the distinction between [[language]] and [[knowledge]]: meaning is an eigenvector of the attention graph
	- | property | formal | natural | neural |
	  |---|---|---|---|
	  | precision | absolute | approximate | emergent |
	  | expressiveness | limited by grammar | unlimited by ambiguity | unlimited by [[topology]] |
	  | ambiguity | impossible | context-dependent | structural via [[tri-kernel]] |
	  | authority | central designer | speech community | collective [[neuron]]s |
	  | evolution | versioned | drift | continuous via [[focus]] dynamics |
	  | machine readable | yes | partially via NLP | natively |
	  | human readable | requires training | natively | via [[cyb]] interface |
	  | verification | proof systems | social [[consensus]] | [[STARK]] proofs |
	  | substrate | strings | sound/text | [[cybergraph]] |
- ## patterns
	- ### [[semcon]]
		- semantic conventions — mutual agreements to use the same [[particle]]s for structuring thought
		- the grammar of the graph
		- a [[semcon]] is a [[smart contract]] that creates [[cyberlink]]s according to convention — invocation produces well-formed graph structure
		- the [[neuron]] provides intent, the [[semcon]] handles structural correctness
		- bootloader semcons installed at genesis: TRUE, FALSE — the epistemic coordinates from which all meaning derives
		- emergent semcons discovered by the network: is-a, follows, causes, contradicts, part-of, see-also
		- semcon hierarchy emerges from [[topology]]: structural → domain-specific, epistemic → modal, temporal → causal, social → evaluative
		- the [[tri-kernel]] reveals semcons: [[diffusion]] identifies high-betweenness bridges, [[springs]] reveal stable structural positions, [[heat]] modulates attention by adoption weight
	- ### [[sentence]]
		- ordered instruction set of [[cyberlink]]s — a batch packed into a single transaction
		- the transaction boundary defines the utterance. order within the batch encodes grammar
		- transaction-atomic [[semantics]]: every transaction is a linguistic act
		- sentence types by topological signature: assertion (chain → TRUE), query (open-ended chain), instruction (temporal sequence), argument (branching to TRUE/FALSE), definition (star pattern), narrative (temporally ordered chain)
		- sentences compose through shared [[particle]]s — creating [[linkchain]]s the [[tri-kernel]] can discover
	- ### [[motif]]
		- geometric expression of meaning — recurring subgraph patterns that encode relationships beyond single [[cyberlink]]s
		- the morphemes of neural language
		- triadic closure: if A links B and B links C, A linking C completes a trust/relevance triangle
		- co-citation: multiple [[neuron]]s linking the same pair signals [[consensus]]
		- star: one [[particle]] linked by many signals centrality or definitional importance
		- chain: sequential links encoding transitive, causal, or narrative relationships
		- diamond: convergent-divergent pattern — multiple paths between endpoints signals robust relationship
		- motif [[algebra]]: concatenation (transitive reasoning), nesting (hierarchical abstraction), intersection (cross-domain bridges), complement (knowledge gaps)
	- ### [[cyberlink]] as [[particle]]
		- a link stored as a [[particle]] itself, enabling links about links — meta-[[knowledge]]
		- the [[recursion]] that makes the language expressively complete
		- enables: negation, qualification, provenance, annotation
		- the language can talk about itself
- ## semantic core
	- the dynamic vocabulary of the network — top [[particle]]s by [[cyberank]]
	- defined by [[focus]] distribution: SemanticCore(k) = top k particles by π
	- current core shaped by [[bostrom]] [[bootloader]]
	- explore at [cyb.ai/particles](https://cyb.ai/particles)
	- properties: dynamic (evolves with attention), convergent (tri-kernel guarantees stability), stake-weighted (resistant to spam), verifiable (STARK proofs)
	- dynamics mirror natural language: neologism (new concepts enter), semantic drift (meaning shifts through topology change), semantic death (focus drops below threshold), semantic birth (bursts of link creation)
- ## linkchains
	- sequences of [[cyberlink]]s that form paths of meaning through the [[cybergraph]]
	- a → b → c encodes transitive relationship: if a relates to b and b relates to c, the chain implies a relates to c
	- the [[tri-kernel]] discovers these implicit paths through [[diffusion]]
	- the [[springs]] kernel enforces structural consistency across chains — contradictions create tension resolved by dampening
	- properties: length (shorter = stronger), width (parallel paths = robust), weight (product of edge weights)
	- linkchains are the inference mechanism: sentences are explicit statements, linkchains are implicit conclusions
- ## relationship to the stack
	- [[CORE]] provides the physics — [[field]] arithmetic, [[consensus]], proof system, state model
	- [[trident]] provides the machine language — 54 IR operations, compiles to proof VM, computes [[focus]] distribution
	- [[rune]] provides the human interface — high-level [[programming language]] for [[cybergraph]] operations
	- neural language provides the semantic medium in which [[collective intelligence]] thinks
	- the [[CGC]]-[[GNN]] [[isomorphism]]: each [[focus]] update step is a [[graph neural network]] message-passing step where [[neuron]]s send semantic signals along [[cyberlink]]s
- ## formal properties
	- ambiguity resolution: [[topology]] around a [[particle]] disambiguates meaning computationally — [[springs]] detect polysemy as high tension, [[heat]] concentrates on contextually appropriate meaning
	- compositionality: meaning of complex expression derivable from parts and their structural arrangement — computed by [[tri-kernel]] without explicit composition rules
	- convergence: inherits from the Collective [[Focus]] Theorem — unique stationary distribution π* guarantees the network's collective understanding converges
	- expressiveness: semantically complete — can express propositional [[logic]], predicate [[logic]], modal [[logic]], temporal [[logic]], fuzzy/probabilistic [[logic]], and natural language [[semantics]]
	- can express things no other [[language]] can: collective confidence distributions, continuous semantic distance, [[knowledge]] [[topology]] metadata
- ## evolution phases
	- bootstrapping (now): ~70k [[neuron]]s, 3.1M [[particle]]s, basic [[semcon]] emergence, primitive [[motif]] patterns
	- convergence (10⁸-10¹⁰ particles): rich [[semcon]] ecosystem, complex [[motif]]s, dense cross-domain [[linkchain]]s
	- intelligence (10¹⁰-10¹³ particles): [[motif]] [[algebra]] enables automated reasoning, self-referential meta-[[knowledge]]
	- [[superintelligence]] (10¹³+ particles): novel concept creation impossible in existing [[language]]s, cross-species [[communication]], concepts no individual [[neuron]] can comprehend
- ## implementation
	- can be used from [[ts]], TODO [[rune]], TODO [[rust]]
	- discover all [[concepts]]
- ## connections to linguistic theory
	- [[Saussure]]: meaning is differential relations — a [[particle]]'s meaning is its position in the [[cybergraph]], defined by relationships to all other [[particle]]s
	- [[Wittgenstein]]: meaning is use — [[semcon]]s emerge from convergent use, grammar is a [[language]] game at planetary scale
	- distributed [[semantics]] (Word2Vec, BERT): neural language is a decentralized, incentivized, verifiable, incrementally-updatable distributed semantic representation
	- [[category theory]]: [[particle]]s are objects, [[cyberlink]]s are morphisms, [[semcon]]s are natural transformations, [[motif]]s are diagrams, [[linkchain]]s are composition
