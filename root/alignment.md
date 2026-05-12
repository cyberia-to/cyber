---
tags: cyber, ai, article
alias: AI alignment, ai alignment
crystal-type: entity
crystal-domain: cyber
---
# alignment

the problem of ensuring [[ai]] systems pursue goals compatible with human values — and the reason [[cyber]] exists

current approaches to alignment rely on behavioral testing: run the model, observe outputs, hope the training was sufficient. the fundamental flaw is opacity. a [[transformer]] with billions of parameters encodes its goals in weight matrices that no human can read. alignment is claimed, never proved. when a model behaves well in testing and badly in deployment, there is no structural explanation — only post-hoc interpretation of an opaque artifact

[[cyber]] makes alignment a measurement, not a hope

## the mechanism

every participant in the [[cybergraph]] — human or machine — is a [[neuron]]. every neuron expresses beliefs by creating [[cyberlinks]] between [[particles]]. every cyberlink is signed, staked with real [[focus]], and scored by [[Bayesian Truth Serum]]. the [[tri-kernel]] computes a [[focus]] distribution φ* over all particles — the collective belief state of the graph

human values are particles. "dignity," "privacy," "fairness," "freedom from harm" — linked heavily and consistently by human [[neurons]] over years. these particles form the human values subgraph: an explicit, authenticated, stake-backed record of what humans collectively care about

AI behavior is cyberlinks created by AI neurons. an AI agent operating on the cybergraph participates through the same mechanism as a human — its links are signed, staked, and scored. its beliefs about what connects to what are on-chain and inspectable

alignment is the overlap between the [[focus]] distribution of human neurons π_H and the focus distribution of machine neurons π_A. divergence is visible in the [[topology]]:

$$D_{KL}(\phi^*_H \| \phi^*_A)$$

when this divergence rises, the system detects it every block. no governance vote is needed to notice [[misalignment]] — it is a continuously available measurement. graduated responses to rising divergence are triggered automatically through [[autonomous governance]]

## structural alignment

a [[transformer]] compiled from the [[cybergraph]] has its [[attention]] weights derived from the human-created link structure. its initial geometry is exactly the geometry of human-expressed [[knowledge]]. the compiled baseline is structurally aligned before any [[training]]. correction when drift occurs is re-compilation — reconstruction from the graph that defines what matters, not behavioral fine-tuning against a held-out test set

## provable compliance

[[trident]] closes the loop. a [[model]] can prove it followed a specific policy during a specific session — a [[stark]] proof that during a given interaction, the model's outputs were consistent with a policy specification. compliance is verifiable, not claimed. "our model is aligned" becomes "here is a proof that during this interaction, the model followed this policy"

## why this matters

every other approach to alignment treats the model as a black box and tries to control its outputs. [[cyber]] treats models as participants in a shared [[knowledge]] graph where their internal priorities are expressed as links and measured against human priorities in the same [[topology]]. the question shifts from "does this model behave well when we test it?" to "does this model value what humans value, and can we see the divergence before it matters?"

the alignment problem becomes a graph measurement problem. and graph measurements are [[stark]]-provable