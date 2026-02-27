---
alias: Shannon information theory, information theory, Claude Shannon
tags: cyber, article
crystal-type: entity
crystal-domain: biology
---
Claude Shannon, 1948. "A Mathematical Theory of Communication"

Shannon defined [[information]] as a statistical property: the less probable a message, the more information it carries. the definition is precise, quantitative, and deliberately excludes meaning

> the semantic aspects of communication are irrelevant to the engineering problem

## the formulas

entropy of a discrete source:

`H(X) = −Σ p(x) log₂ p(x)`

the average surprise per symbol. the minimum number of [[bits]] needed to encode messages from the source. maximum entropy = maximum uncertainty = all symbols equally likely

mutual information between source and received signal:

`I(X;Y) = H(X) − H(X|Y)`

how much uncertainty about X is resolved by observing Y

channel capacity:

`C = max_{p(x)} I(X;Y)`

the maximum rate at which information can be transmitted reliably over a noisy channel

## where Shannon meets [[cyber]]

Shannon's entropy applies to the data inside a [[particle]] — the raw bytes, their compressibility, their statistical structure. the [[hash]] is something else: it is the identity of the particle, a fixed-length fingerprint that enables verification, deduplication, and addressing. the hash is not the information content of the particle; it is the measurement that makes the content identifiable. a completely predictable file and a maximally random file produce hashes of the same length — but their Shannon entropy differs vastly

Shannon's channel coding theorem guarantees that [[particles]] can be transmitted reliably over noisy networks. content addressing provides automatic error detection: if the hash doesn't match, the particle is corrupted. Shannon gave the theoretical limits; content addressing gives a practical implementation

the act of hashing is where data becomes [[information]]: before hashing, the content is uncertain; after, it is identified exactly. this is reduction of uncertainty — Shannon's core concept — applied as a one-shot measurement

## where [[cyber]] goes beyond Shannon

Shannon's theory covers transmission. it answers: how do I send this message reliably? it says nothing about what the message means, how it relates to other messages, or what can be inferred from collections of messages

[[cyber]] picks up where Shannon stops

| | Shannon | [[cyber]] |
|---|---|---|
| substrate | data (bytes) | data (bytes) |
| measurement | entropy | [[hash]] |
| unit | symbol | [[particle]] |
| identity | sequence position | content address |
| naming | (none) | `~` [[name]] → [[file]] |
| structure | sequence (channel) | graph ([[cybergraph]]) |
| meaning | excluded by design | computed by the [[truth machine]] |
| cost | bandwidth, power | [[focus]] |
| output | received message | [[intelligence]] |

the chain data → [[information]] → [[file]] → [[knowledge]] → [[intelligence]] maps to:

- data: raw bytes. Shannon's entropy measures their statistical properties
- [[information]]: data identified by [[hash]] — a [[particle]]. Shannon applies here as measurement
- [[file]]: a [[particle]] given a `~` [[name]]. Shannon has no concept of naming
- [[knowledge]]: [[particles]] linked by [[neurons]] via [[cyberlinks]]. Shannon has no concept of this — linking is an assertion of meaning, which Shannon explicitly excluded
- [[intelligence]]: the observation loop between [[neurons]] and the [[truth machine]] — [[neurons]] observe [[explicit knowledge]], derive [[implicit knowledge]], and link again. Shannon has no concept of inference, relevance, or structure emerging from accumulated messages

## Shannon entropy in the [[cybergraph]]

Shannon's entropy remains relevant inside the protocol. the entropy of the [[focus]] distribution H(π) = −Σ π(v) log π(v) measures the diversity of collective attention. low entropy means the collective focuses narrowly. high entropy means attention is spread evenly. [[syntropy]] — the opposite of entropy — measures how much structure the [[truth machine]] has extracted from the graph

the [[tri-kernel]] drives the focus distribution toward a fixed point. this fixed point is where Shannon's entropy meets [[intelligence]]: the converged distribution is the protocol's answer to "what matters?"

discover all [[concepts]]
