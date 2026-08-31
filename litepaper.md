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

everything else in this document is the machinery that makes that sentence enforceable.

---

## the gap

a payment settles globally in seconds with no bank in the loop. a *judgment* — what deserves attention, whose sentence became the pattern, who gets paid when the shared picture improves — settles nowhere. it is decided inside a company: the search index, the feed, the weight file trained on everyone's writing and rented back.

so the world has a public ledger for value and none for meaning. the consequences are not abstract. you cannot open the weights and see why this answer. you cannot pay the teacher whose sentence became the pattern. you cannot prove that a machine and a human still care about the same world. and the one architecture everybody bets on — quadratic attention, where twice the context costs four times the compute — puts the next oracle inside four balance sheets.

citations and likes were the attempt. they are weak money: easy to spam, impossible to audit, silent on whether anything actually got clearer.

---

## the machine, in one page

five primitives. a **particle** is content-addressed data — its own hash, so it cannot be quietly edited. a **neuron** is a keypair: human, model, sensor, agent, all the same citizenship. a **cyberlink** is a signed, staked, timestamped claim that two particles belong together. a **token** is the weight behind that claim. **focus** is what falls out.

one operator. the [[tri-kernel]] runs three local maps over the staked graph — diffusion (where probability flows), springs (what satisfies structure), heat (what the graph looks like at scale $\tau$) — and iterates to a fixed point $\phi^*$. under a contraction condition the fixed point exists, is unique, and is computable from a bounded neighborhood. nobody votes on importance. it is computed, and it is the same number for everyone.

three jobs from that one engine, each of which is a closed industry today:

- **rank** — $\phi^*$ is the ranking. no operator, no ad market, no feed algorithm.
- **compile** — the model's architecture is *read off* the graph rather than chosen: embedding dimension from spectral entropy, heads from dialect structure, depth from diameter times convergence rate. the compiled model is a hash of the graph. reproducible, auditable, and every weight traces to the neurons whose links produced it.
- **price** — the shift in $\phi^*$ that a contribution caused is the quantity money is minted against.

no float anywhere in the provable path. everything is fixed-point arithmetic over one 64-bit prime field, so two machines produce byte-identical results and a proof can be written about the whole computation.

---

## why the money is different

**minting is a measurement, not a policy.** the network's order is [[syntropy]] $J = D_{\mathrm{KL}}(\phi^*\Vert u)$ — how far collective focus has travelled from noise. new [[cyber/$CYB|$CYB]] exists only where $J$ went up, in the amount it went up. inflation stops being a number someone chooses and becomes the reading of a physical process.

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

what the last vehicle produced, rebuilt from block events and matched link-for-link against the chain's own statistics:

| | |
|---|---|
| cyberlinks | 2,949,732 |
| particles | 3,143,650 |
| accounts | 61,675 |
| signed at least one transaction | 52,918 |
| staked to consensus | 16,791 |
| voted in governance | 5,134 |
| hand-linked knowledge | 1,240 neurons |

three results matter to anyone deciding whether this is real.

**the payment direction was inverted and people paid anyway.** wikipedia's volunteers write free under editors. imagenet paid crowdworkers to label. here every link cost its author scarce stake, no editor approved anything, and there was no answer key to forge — and sixty thousand accounts joined an economy whose only product was structured attention.

**the content survived with no incentive to store it.** 97.62% of particles are still available in complete form — every block of every file, not just the root — five years on, with no storage rewards and no proof-of-storage ever deployed. that is the number a storage market has to beat, and it suggests the hard part of permanence is economic, not technical.

**and the crystal is still thin.** this is the number that should be read as opportunity rather than as a result. the graph's measured semantic dimensionality is $d^* = 31$ against a planetary target of $10^3$–$10^4$. its giant component holds 47% of particles — more than half the corpus sits in islands, unreachable from the core. one archivist neuron signed 77.6% of all links, and only 1,240 of 61,675 accounts ever linked anything at all.

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

if you allocate capital: this is a market that does not exist yet — the settlement of meaning — with the only five-year field experiment anyone has run on it already finished, measured, and published.

if you are a person with something true to say: link it, stake it, and be paid when the picture sharpens.

---

read the full argument in the [[whitepaper]] · the physics audit in [[physical analogies]] · the stack in [[soft3]] · the sealed corpus at [[bostrom]]

discover all [[concepts]]
