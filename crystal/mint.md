---
alias: issuance, create token
tags: cyber, core
crystal-type: process
crystal-domain: cyber
crystal-size: enzyme
---
how [[tokens]] enter existence. [[coins]] through [[consensus]] rewards, [[cards]] through provenance binding, [[scores]] through earned reputation

[[coins]] mint at the protocol level. each [[step]], the [[tru]] distributes newly created [[coins]] to [[validators]] and their delegators according to the inflation schedule defined by [[consensus]] parameters. this issuance funds network security.

[[cards]] mint through a dedicated [[signal]] from a [[neuron]]. the minting [[neuron]] defines the card's supply, metadata, and provenance binding — an immutable link to the originating [[particle]] or external reference that establishes authenticity.

[[scores]] mint algorithmically. the protocol evaluates a [[neuron]]'s contribution history — [[cyberlinks]] created, [[cyberank]] earned, governance participation — and issues [[scores]] that reflect cumulative reputation within the [[cybergraph]].

every mint event increases total supply of the minted [[token]] type. the protocol enforces supply caps where defined: [[coins]] follow a predictable inflation curve, [[cards]] respect the maximum supply set at creation, [[scores]] accumulate within bounded tiers.

mint requires authority. [[coins]] mint only through [[consensus]] execution. [[cards]] mint only by the issuing [[neuron]] with a valid [[signature]]. [[scores]] mint only by the protocol's reputation module. unauthorized mint attempts fail at validation.

the [[gas]] cost of minting a [[card]] scales with its metadata size and initial supply count. this prevents spam issuance and ensures every minted [[token]] carries an economic commitment from its creator.

discover all [[concepts]]
