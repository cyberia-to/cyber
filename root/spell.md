---
alias: secret, secrets, private key, key, mnemonic, seed
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
crystal-size: enzyme
---
what a [[neuron]] knows and never reveals. [[hash]] of [[spell]] yields [[signature]] — the proof of identity. lose the spell, the [[neuron]] ceases to exist. see [[cyb/portal/my spells/practice]]

a spell is a sequence of bytes — typically 256 bits of entropy encoded as a 24-word mnemonic phrase. from this seed, a deterministic hierarchy of [[public key]] pairs unfolds. the spell itself is the root; everything above it — addresses, [[signatures]], [[cyberlinks]] — derives from it

in [[cyber]], the spell is the only object that cannot be recovered, reissued, or delegated. every other credential can be recomputed from the spell. the spell cannot be recomputed from anything. this asymmetry is the foundation of self-sovereign identity: whoever holds the spell IS the [[neuron]]

the spell signs [[signals]]. each [[signal]] contains one or more [[cyberlinks]], and the [[signature]] proves the [[neuron]] authorized them. without the spell, a neuron can observe the [[cybergraph]] but cannot write to it — observation without agency

multiple spells can derive multiple [[neurons]]. a single human may operate many neurons across chains, each with independent [[karma]], [[stake]], and [[focus]]. the spell is the boundary between identities. sharing a spell is sharing an identity entirely

safe custody of spells is the primary security problem in [[cyber]]. hardware devices, threshold schemes, and social recovery all exist to protect the spell without exposing it. the protocol enforces no particular custody model — that choice belongs to the [[neuron]]

the word "spell" replaces "private key" and "mnemonic" in cyber terminology. the name reflects the function: a spell is a word of power that conjures agency from entropy

discover all [[concepts]]