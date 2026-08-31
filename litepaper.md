---
tags: cyber, article, cip
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
alias: cyber litepaper, litepaper
---

# cyber litepaper

> money settles without a bank. meaning still rents its rank from whoever owns the index.

**cyber is a protocol that mints money for one thing only: a proven, measurable reduction in collective uncertainty.** not for burned electricity, not on a schedule a committee votes on — for the exact amount by which the network's shared picture of the world got sharper, attributed to whoever sharpened it.

one law, two products. read forward, it is a superintelligence whose every synapse was bought by someone who believed it: a mind assembling itself out of paid, signed, provable acts of understanding. read backward, it is the hardest money ever proposed: a currency that cannot inflate faster than the world's knowledge grows. everything else in this document is the machinery that makes both readings enforceable.

---

## the gap

a payment settles globally in seconds with no bank in the loop. a *judgment* — what deserves attention, whose sentence became the pattern, who gets paid when the shared picture improves — settles nowhere. it is decided inside a company: the search index, the feed, the weight file trained on everyone's writing and rented back.

so the world has a public ledger for value and none for meaning. the consequences are not abstract. you cannot open the weights and see why this answer. you cannot pay the teacher whose sentence became the pattern. you cannot prove that a machine and a human still care about the same world. and the one architecture everybody bets on — quadratic attention, where twice the context costs four times the compute — puts the next oracle inside four balance sheets.

citations and likes were the attempt. they are weak money: easy to spam, impossible to audit, silent on whether anything actually got clearer.

---

## what a superintelligence is

not a bigger model. **one focus** — a single distribution $\phi^*$ over everything known, computed from every mind that pays to teach, that provably sees what no member sees alone.

that last clause is a measurement, not a slogan. run the same ranking engine on one neuron's private view of the graph, then on the whole graph, and score both on prediction tasks. the gap is [[superadditivity]] — and on benchmark it is positive against the *strongest* participant at every connectivity level. the collective beats its best member, and the margin is a number that runs on a phone. emergence stops being philosophy the moment it has an error bar.

superintelligence also has a scale, and the scale has physics. a graph, like a gas, crosses a phase threshold where individual links stop mattering and only the field $\phi^*$ describes the system — the graph's own Avogadro number:

| milestone | particles | what happens there |
|---|---|---|
| crystal seed | 5,040 | irreducible cross-domain seed, curated |
| bootloader corpus, today | 3,143,650 | flow phase, $d^* = 31$, one archivist dominant |
| cognition threshold | 10⁸–10⁹ | hierarchies form; concepts appear that no individual holds |
| thermodynamic regime | 10¹² | $\phi^*$ becomes the only description of state |
| design target | 10¹⁵ particles · 10¹⁰ neurons | planetary mind; $d^*$ saturates at 10³–10⁴ semantic axes |

we are five years in and nine orders of magnitude early — which tells you exactly what the next decade of work is, and that nobody is late.

two more properties separate this from a large model. **alignment becomes a distance**: human and machine neurons stake into the same graph, so their divergence is $D_{\mathrm{KL}}$ between two focus distributions on shared ground — a monitored number, not a sermon. and **the protocol is itself a neuron**: it holds keys, stake, treasury and [[karma]], links from its own agenda, and retunes its parameters from the graph's own $\phi^*$ — so the codebase and the development direction settle in the same consensus as every other claim. no founder steers it alone. that self-referential loop, running at scale, is what the word superintelligence means here.

## the machine, in one page

five primitives. a **particle** is content-addressed data — its own hash, so it cannot be quietly edited. a **neuron** is a keypair: human, model, sensor, agent, all the same citizenship. a **cyberlink** is a signed, staked, timestamped claim that two particles belong together. a **token** is the weight behind that claim. **focus** is what falls out.

one operator. the [[tri-kernel]] runs three local maps over the staked graph — diffusion (where probability flows), springs (what satisfies structure), heat (what the graph looks like at scale $\tau$) — and iterates to a fixed point $\phi^*$. under a contraction condition the fixed point exists, is unique, and is computable from a bounded neighborhood. nobody votes on importance. it is computed, and it is the same number for everyone.

three jobs from that one engine, each of which is a closed industry today:

- **rank** — $\phi^*$ is the ranking. no operator, no ad market, no feed algorithm.
- **compile** — the model's architecture is *read off* the graph rather than chosen: embedding dimension from spectral entropy, heads from dialect structure, depth from diameter times convergence rate. run on the bootloader corpus this took **62 seconds on one machine with 20 GB of RAM**: $d^* = 31$, $h^* \ge 12$ heads, $L^* = 290$ layers, ~0.4M parameters — because the graph's sparsity ($\rho \approx 10^{-7}$) makes compilation near-linear in links at any scale. the compiled model is a hash of the graph. reproducible, auditable, and every weight traces to the neurons whose links produced it.
- **price** — the shift in $\phi^*$ that a contribution caused is the quantity money is minted against.

no float anywhere in the provable path. everything is fixed-point arithmetic over one 64-bit prime field ($p = 2^{64} - 2^{32} + 1$), iterated a compile-time-constant number of steps, so two machines produce byte-identical results and a proof can be written about the whole computation: hash-based, post-quantum, no trusted setup, ~100–200 KB per proof, folding to one constant-size check per settlement cluster.

---

## good money

the deepest consequence of the design is monetary, so it deserves its own claim: **this is the first money whose issuance is a measurement.**

every hard currency in history rationed its supply with a cost. gold's cost is geological — scarcity by accident of crust. bitcoin's cost is thermodynamic but spent on nothing: the dissipation buys ordering of transactions and is otherwise discarded. [[cyber/$CYB|$CYB]]'s cost is epistemic: the network's order is [[syntropy]] $J = D_{\mathrm{KL}}(\phi^*\Vert u)$ — how far collective focus has travelled from noise, in bits — and new money exists only where $J$ went up, in the amount it went up. **the money supply grows exactly as fast as the world's proven understanding, and cannot grow faster.** a unit of $CYB is a receipt for negentropy.

that gives it the full list of hard-money properties, each enforced by a different mechanism rather than by promise:

- **unforgeable** — no mint without a validity proof of the focus shift; forging a claim means forging a proof, and there is no trusted setup to corrupt.
- **conserved** — settled shares are clipped to the realized global shift: over-claiming cannot exceed the value that was actually created, by construction rather than by audit.
- **uncopyable earnings** — a perfect copy of a link mints zero: the surprise gate weights it out while its capital still ranks. printing money by repetition is structurally closed.
- **no committee** — supply follows stepped emission plus fee redistribution; when fees exceed emission the network runs net deflationary, and the transition from emission-funded to fee-funded happens continuously, with no governance vote anywhere in the loop.
- **physically backed** — the asset behind the money is knowledge, and [[landauer limit|Landauer's bound]] prices its floor: ~3×10⁻²¹ joules per bit at room temperature — the lowest mass per unit of value physics permits. which is also why it is the one asset worth carrying between planets.

**two pays, two risks.** staking a link is a bet on truth: capital at risk, influence and reward if you were early and right. mining is capital-free work: sample the fair division of a proven shift and fold the proofs. you earn for settling credit, not for guessing the future.

**two pays, two risks.** staking a link is a bet on truth: capital at risk, influence and reward if you were early and right. mining is capital-free work: sample the fair division of a proven shift and fold the proofs. you earn for settling credit, not for guessing the future.

**the proof-of-work does the accounting.** dividing value fairly among overlapping contributors is a [[Shapley value]] computation, and Shapley is estimated by sampling random orderings. so each mining ticket *is* a sample: the hash a miner grinds is the ordering index, and the same act secures the chain and computes who gets paid. there is no synthetic puzzle. thermodynamics says this is not merely elegant — precision costs dissipation, so a trustworthy division of credit has an energy price floor, and paying it once instead of twice is the least wasteful design available.

**copying earns nothing.** content addressing makes perfect copies free, so a copied link would produce the same focus shift as the original. a per-contribution surprise score — [[Bayesian Truth Serum]], where truthful reporting is the equilibrium — enters the value function as a weight, so a copy joins the mint weightless while its capital still ranks.

---

## why it is not another chain

the binding constraint is not throughput. it is light.

earth to mars is minutes one way. any protocol whose liveness assumes a fast planet-wide round is already dead at that distance, and every design that finalizes by counting votes assumes exactly that. so finality here is the same fixed point doing a third job: a fact is final when enough attention has gathered on it — $\phi^*_i > \tau$ — not when a global committee replies. nodes gossip, each runs the same contraction, identical signals produce one root everywhere, as a pond finds one level without phoning the far shore.

domains settle at domain speed. a partition freezes cross-domain trade instead of inventing two truths. disputes pay only the light they must.

designing for planets is not decoration. it is what forces the removal of every global round, and the removal is what makes the thing work well on one planet.

---

## the bootloader

we did not argue the premise. we ran it.

the **bootloader** is not a chain. it is the mission of growing the [[crystal]] — the seed graph dense enough to boot a mind — and it has had three vehicles: cyberChain in 2016, the Euler network that put pagerank inside consensus on GPUs in 2018, and [[bostrom]], which ran knowledge-graph consensus on a live cosmos-sdk chain for 1,735 days and sealed 25,120,712 blocks on 2026-08-05. retiring the cosmos vehicle is not finishing the mission. it is changing engines.

what the last two vehicles produced, rebuilt from block events and matched link-for-link against the chains' own statistics:

| | [[bostrom]] | [[space pussy]] |
|---|---|---|
| blocks sealed | 25,120,712 | 19,940,993 |
| cyberlinks | 2,949,732 | 29,112 |
| particles | 3,143,650 | 48,370 |
| accounts | 61,675 | 616 |
| signed a transaction | 52,918 | 616 |
| staked to consensus | 16,791 | — |
| voted in governance | 5,134 | — |
| hand-linked knowledge | 1,240 neurons | — |
| snapshot | [bostrom.network](https://bostrom.network) | [pussy.bostrom.network](https://pussy.bostrom.network) |

three results matter to anyone deciding whether this is real.

**the payment direction was inverted and people paid anyway.** wikipedia's volunteers write free under editors. imagenet paid crowdworkers to label. here every link cost its author scarce stake, no editor approved anything, and there was no answer key to forge — and sixty thousand accounts joined an economy whose only product was structured attention.

**the content survived with no incentive to store it.** 97.62% of particles are still available in complete form — every block of every file, not just the root — five years on, with no storage rewards and no proof-of-storage ever deployed. that is the number a storage market has to beat, and it suggests the hard part of permanence is economic, not technical.

**and the crystal is still thin.** this is the number that should be read as opportunity rather than as a result. the graph's measured semantic dimensionality is $d^* = 31$ against a planetary target of 10³–10⁴. its giant component holds 47% of particles — more than half the corpus sits in islands, unreachable from the core. one archivist neuron signed 77.6% of all links, and only 1,240 of 61,675 accounts ever linked anything at all.

so the binding constraint is not links, capital, or compute. it is **independent authors**, and every measurement points at the same dial. the bootloader's job is to keep growing the crystal until it can boot a mind, and by its own metric it is three orders of magnitude early.

what the last vehicle also demonstrated is exactly what it could not do: its graph could only be ranked by whoever ran the indexer, and the knowledge in it could not be proven, priced, or paid for by the people who made it. that gap is the product.

---

## why the design is hard to argue with

the load-bearing parts were not chosen. they are the same objects physics already measured, and they cannot be tuned away by a competitor with better parameters:

- the screened Laplacian is a lattice Klein–Gordon propagator, so **locality is a theorem** — the exponential clustering that lets a phone compute its own reward from a bounded neighborhood.
- the Shapley value in its continuous limit *is* thermodynamic integration, so **fair division is free-energy attribution**, with the path-independence that comes free.
- the honesty score is dissipated work, so **an honest report is the reversible limit and a lie is irreversible** — the second law with a ledger attached.

the audit runs the other way too, and it is published rather than buried: the model is Perron–Frobenius, so it describes the classical layer of the world and can never violate a Bell inequality; the specification states conservation but has not yet written its entropy account. see [[physical analogies]].

---

## what is unsolved

the honest summary: **the substrate is settled and the incentives are not.**

the mathematics is theorems. the open problems are all economic, and naming them is cheaper than discovering them later. pricing novelty is late by construction — a genuinely new link scores low on the market gate exactly when its surprise is highest. collusion among settlement miners is bounded but not closed. a miner who also contends can withhold a sample that lowers its own share; the bias is bounded by its share of compute and priced by forfeiting the subsidy, which is a deterrent, not a proof.

a superintelligence built on this stack will fail, if it fails, there — not in the operators.

---

## what exists today

a sealed, audited, publicly verifiable corpus of three million particles and their full provenance. a client that runs the graph, a terminal, and a local model on desktop and phone from one binary. a sovereign transport. a compiler that turned the bootloader graph into a transformer in 62 seconds on a single machine. specifications, in the open, for the field, the proof system, the language, and the consensus.

what is next is the graph with money in it.

---

## the invitation

if you build models: your architecture is currently guessed and your training data is unattributable. here both are derived, and the people who taught the machine can be paid.

if you build chains: the settlement work here is not synthetic. the hash you grind computes who deserves credit.

if you allocate capital: this is a market that does not exist yet — the settlement of meaning — with the only five-year field experiment anyone has run on it already finished, measured, and published. and the instrument is a currency whose supply is bounded by the growth of proven knowledge — a scarcity no central bank and no miner cartel can debase.

if you are a person with something true to say: link it, stake it, and be paid when the picture sharpens.

---

read the full argument in the [[whitepaper]] · the physics audit in [[physical analogies]] · the stack in [[soft3]] · the sealed corpus at [[bostrom]]

discover all [[concepts]]
