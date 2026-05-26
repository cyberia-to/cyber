---
tags: cyber
crystal-type: entity
crystal-domain: cyber
alias: deterministic resolution
---
resolution modes of [[name]] in the [[cybergraph]]

a [[cyberlink]] is a dynamic pointer: from [[particle]] resolves to a ranked set of to [[particles]]. standard resolution is probabilistic — the [[convergence vm]] returns candidates sorted by [[cyberank]]. a name is a [[cyberlink]] that resolves deterministically: given from, return exactly one to — the latest [[particle]] linked by the owning [[neuron]]

the same mechanism underlies every naming system: [[file]] systems map paths to inodes, [[DNS]] maps domains to IP addresses, ENS maps .eth to wallets. all are dynamic pointers where a fixed label resolves to a mutable target. in the [[cybergraph]] this is native — a [[cyberlink]] already is a dynamic pointer, the only question is the resolution mode

| mode | returns | use |
|------|---------|-----|
| probabilistic | ranked set of [[particles]] by [[cyberank]] | search, discovery, inference |
| deterministic | single [[particle]], last linked by owner | naming, addressing, file system |

## the ~ prefix

the `~` prefix signals deterministic resolution

```
probabilistic:  cyber             → ranked particles
deterministic:  ~mastercyb/blog   → single latest particle
```

`~` is borrowed from Unix home directories — the [[neuron]] is the home, the path after it is a [[linkchain]] of names owned by that [[neuron]]. this turns the [[cybergraph]] into a dynamic file system where every [[neuron]] maintains a namespace rooted at `~`

## mechanics

a name is a [[cyberlink]] where:

1. from [[particle]] is the name label (content-addressed string, e.g. hash of "blog")
2. to [[particle]] is the current value (any [[particle]] — a page, an image, a program)
3. resolution picks the to of the latest [[cyberlink]] from this [[neuron]] for this from

updating a name means creating a new [[cyberlink]] with the same from and a different to. the old value remains in history. the latest wins

## as semcon

name is a [[semcon]] — a structural convention where [[neurons]] agree that certain [[cyberlinks]] are dynamic pointers meant for deterministic resolution rather than probabilistic search. the `~` prefix is the syntactic marker of this convention

## examples

```
~mastercyb/avatar     → QmCurrentAvatarCID
~mastercyb/blog       → QmLatestBlogPostCID
~mastercyb/config     → QmCurrentConfigCID
~jooy/public-key      → QmJooyPubKeyCID
```

any [[neuron]] can resolve any other [[neuron]]'s names — the namespace is public, the write access is private (only the owning [[neuron]] can update)

## relation to .moon names

[[.moon names]] are the [[bostrom]] [[bootloader]] implementation of this concept — purchased identities that map human-readable labels to [[neurons]]. names generalize this: every [[neuron]] gets an unlimited namespace for free, addressing any [[particle]] in the [[cybergraph]]

probabilistic resolution is search. deterministic resolution is addressing. both emerge from the same primitive — the [[cyberlink]] — distinguished only by a [[semcon]] prefix. the [[cybergraph]] unifies search engines and file systems into a single structure

discover all [[concepts]]