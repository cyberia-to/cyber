---
tags: cyber, audit, cryptography, mudra, hemera
crystal-type: report
crystal-domain: cyber
alias: cryptography audit, crypto assumptions audit, mudra audit
status: draft
date: 2026-10-08
---

# cryptographic assumptions audit

prompted by the october 2026 debate: AI-assisted mathematics may erode structured assumptions (curves, lattices) faster than expected, and hash-based constructions are the conservative refuge. question asked: how does mudra, and the stack under it, stand against that view.

## 1. the external picture, as of 2026-10-08

- the debate is about *structure*. Drake and Buterin argue AI-accelerated cryptanalysis threatens assumptions with algebraic structure first — ECDSA, then lattices — and favour hash-based signatures (WOTS, SPHINCS/SLH-DSA). Blockstream researchers make the same case for bitcoin. Ethereum has a post-quantum team targeting ~2029.
- lattices are under pressure on two fronts: an unreviewed preprint (Luo, may 2026) claiming a quantum attack on ML-KEM/Falcon/NTRU at standard parameters, and practical side-channel key recovery on masked ML-DSA in tens of traces. neither is a mathematical break of MLWE; both move sentiment.
- the least-noticed front is the one that matters most for cyber: algebraic hashes. the Ethereum Foundation's Poseidon bounty (2024–2026) is being claimed — resultant-based CICO solutions on reduced instances, S-box-skipping recovering 28 of 31 rounds on KoalaBear, the first algebraic collision attacks that beat preimage attacks, weak-matrix exploits in Poseidon2. Ethereum has publicly walked away from Poseidon for its permanent paths.

the market's conclusion — hash plus proofs, no curves, no trusted setup, no committee-chosen structure — is cyber's architecture already. the question is whether the hash cyber chose is the kind the argument has in mind.

## 2. what the stack relies on at genesis (2026-11-05)

| concern | primitive | assumption | status |
|---|---|---|---|
| identity and the old balances | secp256k1 ECDSA, ADR-036 (`mudra/src/claim.rs:123–153`), profile `cyber-secp256k1-hemera-v1` (`neuron/specs/identity.md:21–27`) | discrete log on secp256k1; SHA-256/RIPEMD-160 for the legacy address | built; labelled "asserts neither post-quantum security nor network finality" (`mudra/specs/neuron-auth.md:5–6`) |
| every particle, neuron id, link id, Merkle node, Fiat–Shamir challenge | Hemera: Poseidon2-derived permutation over Goldilocks, t=16, x⁷ full rounds, x⁻¹ partial rounds, R_F=8, R_P=16, self-generated constants (`hemera/rs/src/params.rs:6–37`) | collision and preimage resistance of a nonstandard permutation | built; "full-round security: unestablished; inverse-16 profile: experimental" (`params.rs:32–34`); "algebraic attack margin: not established" (`hemera/specs/README.md:56–58`); neither internal matrix is MDS (`specs/matrices.md:87–95`); unaudited; no algorithm agility (`hemera/README.md:34`) |
| proofs: tickets, fold, execution | zheng — SuperSpartan + sumcheck, Brakedown over Hemera Merkle, HyperNova folding | hash-only: Hemera as random oracle, code distance, Schwartz–Zippel | built; challenges in the 64-bit base field ("must not be advertised as 128-bit", `zheng/specs/execution.md:75`); the fold is trusted rather than verifier-checked (`zheng/specs/decider.md:105–111`); Brakedown spec proves distance ≥ 1 only (`lens/specs/scalar-field.md:29–32`) |
| consensus randomness: beacon, rate limit, equivocation | foculus VDF `x^(2^T) mod p` over Goldilocks (`foculus/src/vdf.rs:6–49`) | none — p−1 is public, so the exponent reduces mod p−1 and the "delay" is ~64 squarings | built placeholder, insecure as a VDF; mudra's own spec says so (`mudra/specs/delay.md:31`) |
| privacy P1–P3 | stealth (CSIDH), veil (RLWE FHE on jali), seal (ML-KEM), quorum (Shamir + VSS) | isogeny class-group action; RLWE + circular security; MLWE; information-theoretic | spec only; "neither stealth nor veil has code in mudra/src" (`cyber/launch.md`, row 11) |

nothing post-quantum protects identity or money at genesis. everything else rests on Hemera.

## 3. verdict

### 3.1 the thesis is right, and cyber already holds it

mudra's native design is signature-free: neuron = Hemera(secret), authority = a zheng proof of the preimage and the policy (`mudra/specs/identity.md:9–19`, `mudra/README.md:7–13`); ML-DSA was considered and removed in favour of proofs (`mudra/audit/signature-optimality/local-decisions.md:7`). zheng has no curve, no pairing, no trusted setup (`zheng/specs/proof-types.md:59`). this is further along the "hash plus proofs" road than bitcoin or ethereum are, and it is the correct road.

### 3.2 the risk is inverted relative to the market's worry

the market worries about ECDSA and lattices. in cyber, ECDSA is explicitly temporary, labelled, and confined to the migration path; lattices and isogenies are specs with no code. the one assumption that is permanent, unlabelled on the product surface, and under active attack by exactly the AI-assisted algebraic methods the debate fears is the hash.

Hemera is an algebraic hash — the least conservative primitive family in the post-quantum landscape — in a variant weaker than the reference it derives from: 16 partial rounds against Plonky3's 22, inverse S-box in the partial layer, non-MDS matrices, constants the designers generated themselves, and the only security claims being generic ceilings with "unestablished" written next to them. the whitepaper's own §4.1 makes the argument against this choice: in a zkVM hashes are ephemeral and parameters can change next release; in cyber hashes are identity, permanent, with a rehash cost of 10¹⁵. that argument calls for the most conservative hash available on the permanent path, and for the algebraic one only where arithmetization pays — inside proofs, where it is ephemeral.

severity: the highest in the stack. genesis freezes every particle and neuron id under this permutation, and the design rejects algorithm agility.

### 3.3 mudra, piece by piece

- secp256k1 today: honest and acceptable for phase 1 — same exposure as bitcoin, with the old keys only claiming balances that already sit under them. what is missing is the exit: a sunset for ECDSA authority after the hash-preimage identity ships, so the quantum-vulnerable surface closes by rule rather than by hope.
- native identity, Hemera(secret) + proof: the right shape, but it is only as strong as the proof system, and today the proof system is weaker than ECDSA — 64-bit challenges, a trusted fold, incomplete zero knowledge. the spec says Hemera(secret); the implementation is Hemera(pubkey) (`mudra/specs/bridge.md:11–12` against `neuron/specs/identity.md:21`). resolve before the preimage identity becomes authority.
- stealth = CSIDH: the most fragile family available. SIDH fell in 2022; CSIDH-512's quantum level is disputed and mudra refuses to certify it (`stealth.md:73–76`); the class-group structure of CSIDH-512 is known (CSI-FiSh, 2019), which also voids delay option C's "group order unknown by construction" (`delay.md:47`). the one good decision is already in the spec: spend authority is separate from the stealth secret (`stealth.md:58–60`), so a break leaks privacy, not money. replace the NIKE with a hybrid KEM (ML-KEM + X25519) and keep the separation.
- veil = FHE on jali: lattice plus circular security, the least conservative assumption pair in the stack; privacy-only, later, acceptable if labelled. the parameter table overstates: n=1024 at q≈2⁶⁴ is far below 128 bits by the HE-standard tables; `veil.md:78` already demands a re-estimate.
- seal = ML-KEM: standard, fine; ship it hybrid.
- quorum: Shamir is information-theoretic and fine; the VSS as written cannot verify a share with plain hash commitments and has no group for the "Pedersen-like" variant (`quorum.md:53–78`). rewrite, or verify shares by zheng proof.
- the foculus VDF: the one built primitive that is broken rather than merely unproven, and it is on the phase-1 path — the beacon's unbiasability is what makes settlement orderings un-front-runnable (`rewards.md` §7). the hash-only fix fits the thesis: a proof of sequential work — T iterated Hemera steps, certified by zheng IVC — is a hash-based VDF under the sequentiality assumption alone. Wesolowski over an imaginary-quadratic class group is the classical alternative and is unrelated to CSIDH.

### 3.4 ranked recommendations before 2026-11-05

1. decide the permanent hash, as a decision rather than an inheritance. two acceptable outcomes: Hemera = reference Poseidon2-Goldilocks parameters (R_P=22, x⁷ throughout, published constants) with a public cryptanalysis bounty; or a conservative hash (Keccak-f or Blake3 family) for particle and neuron identity, with the algebraic permutation kept for proof-internal Merkle and Fiat–Shamir where it is ephemeral. the current experimental inverse-16 profile is acceptable for neither.
2. replace the foculus VDF with a hash-based proof of sequential work verified by zheng, or Wesolowski on a class group of unknown order; the current placeholder cannot carry the beacon.
3. zheng to 128 bits: challenges from an extension field, a verifier-checked fold, and a distance bound for the Brakedown code that supports the claimed soundness.
4. mudra: resolve Hemera(secret) vs Hemera(pubkey); ship the preimage identity only after 3; write the ECDSA sunset rule into the identity spec.
5. stealth off CSIDH onto a hybrid KEM; delete delay option C; re-estimate jali parameters; rewrite quorum VSS.

## 4. one sentence

cyber already bet on the right side of the argument — hash and proofs, no curves — and then built the permanent layer on the one hash family the argument is currently breaking; move the permanence onto a conservative hash and the stack is ahead of the field.

## 5. second pass — the argument in full

### 5.1 why ethereum left, and for what

ethereum dropped Poseidon for its post-quantum paths — hash-based signatures and state commitments in leanVM (2027–28) — and replaced it with SHA-2 and BLAKE3 proven through binary-field SNARKs (the Binius route: reshape the proof system to fit the hash). the stated reasons: Gröbner preimage estimates on Poseidon2 fell from 2¹⁶⁴ to 2¹²⁶ and from 2¹⁷⁴ to 2¹²³ — below 128 — after the 2026 round-skipping work (~2^18.5 speedup); the whole threat concentrates on one attack line, polynomial system solving; SHA-2 has twenty-five years of cryptanalysis against Poseidon2's three; and binary-field proving reached parity (~10⁶ SHA calls proven per second on a laptop, ~100× native). their verdict on Poseidon2 was "unbroken, repairable" — and they left anyway, because when the hash is the only assumption, most mature beats most optimized.

### 5.2 efficiency — nothing grows to 64 bytes

the digest stays 32 bytes under any choice. the cost of a conservative hash appears in exactly one place: hashing *inside a proof*. what cyber proves at genesis — impulse Δφ⁺, settlement tickets, folds — is field arithmetic over ids that enter the circuit as opaque 4-limb values and are never rehashed. the hashes that do run inside proofs are Merkle paths in Brakedown and Fiat–Shamir: ephemeral, proof-time objects, where an algebraic permutation is the right tool. only content-binding proofs (storage proofs) must evaluate the identity hash in-circuit — rare, batched, and the stack already holds ethereum's answer for them: the Binius PCS over kuro F₂¹²⁸ in lens.

and the inverse-16 profile is not even the efficient choice. Hemera's own cost model (`hemera/research/permutation-decision.md`): inverse RF8/RP16 = 560 quadratic constraints and 1712 native nonlinear multiplications; reference x⁷ RF8/RP22 = 600 constraints and 600 multiplications. the inverse buys 6.67% fewer constraints at 2.85× the native cost. moving to reference parameters costs ~7% in-circuit and makes native hashing nearly three times cheaper.

### 5.3 the inverse profile — the honest case

there is no break. the claim is weaker and still decisive for a permanent primitive. point by point, from Hemera's own research:

1. the justification failed. "the original justification (huge polynomial degree implies superior security and permits sixteen partial rounds) fails" (`permutation-decision.md`). the attacker never meets degree p−2: x⁻¹ is modelled as x·y = 1, a quadratic relation — the same algebraic cost as the cheapest S-box. against polynomial-system solving, the one line ethereum named as the threat, the inverse's degree buys nothing.
2. fewer equations, same complexity. partial rounds exist to resist Gröbner; each contributes one quadratic equation. Hemera has 16, the reference 22 — a strictly smaller system of the same kind, with no compensating structure.
3. extra structure, not less. the zero convention 0→0 adds branch equations `z = xy, x(z−1) = 0, y(z−1) = 0` (`full-round-status.md`); the full-round model needs 1161 variables for a seven-byte input, and solver campaigns "do not establish attack lower bounds".
4. subspace restriction is live. `research/partial_subspace.py` exhibits, for 15 of the 16 partial rounds, an affine line of inputs on which every active S-box is the identity (input 1 is fixed by both x⁻¹ and x⁷). eprint 2026/1692 does exactly this — "algebraic cancellation of non-linear inverse S-boxes", one S-box per partial round — and reaches CICO-1 on 28 of 31 rounds of Poseidon-KoalaBear when a single initial full round shields the partial layer. Hemera's four full rounds each side shield better; its partial budget is 16 against 23.
5. the matrices are the attacked component. eprint 2026/306 improves round-skipping by exploiting the structure of the linear layers specifically; Hemera's matrices are not MDS (`specs/matrices.md:87–95`), an unanalysed deviation in exactly the part under attack.
6. the reference analysis does not transfer: custom constants and a different S-box mean none of Poseidon2's published bounds or bounty exposure apply (`permutation-decision.md`: "would not reproduce it or inherit its analysis").

so the strongest true statement about inverse-16 is "no evidence either way" — with fewer Gröbner equations than the reference, more branch structure, non-MDS matrices, zero published cryptanalysis, and a measured upside of 7% constraints bought at 2.85× native cost. for an ephemeral primitive that is acceptable. for the identity of every particle, frozen at genesis without algorithm agility, the burden of proof is on the design, and the design's own research states it is not met: "neither result has been obtained."

decision rule, two acceptable outcomes: (a) Hemera = reference Poseidon2-Goldilocks (RF8/RP22, x⁷, published constants), plus whatever round margin the 2026 repair recommends, with a public bounty — inherits three years of analysis and a repairable margin; (b) identity by a binary hash (BLAKE3 or Keccak), algebraic permutation only inside proofs, binary-field lens for content-binding proofs — the ethereum route, and the stack already has the pieces.

### 5.4 stealth, seal, veil, quorum

stealth needs a non-interactive agreement; no hash-only NIKE exists, so the choice is which structured assumption carries *privacy only* (spend authority stays separate, `stealth.md:58–60` — keep that line, it is what makes stealth a privacy risk and never a money risk).
- unify stealth and seal on one KEM. per-payment stealth = encapsulate to the recipient's published key; the ciphertext is the ephemeral. static-static agreement is lost, and the spec already says it gives no forward secrecy anyway.
- hybrid ML-KEM-768 + X25519: a break of either leaves the other. cost: ~1.1 KB ciphertext against 64 B for CSIDH — the real price, paid once per private output. add a view tag (first bytes of H(shared)) so scanning does not decapsulate every output.
- CSIDH leaves the genesis path and `delay.md` option C is deleted: the CSIDH-512 class-group structure is known (CSI-FiSh, 2019), the quantum level is disputed, and `stealth.md:73–76` already refuses to certify it. it may return as an optional profile if the literature settles.

seal: FIPS 203 ML-KEM-768, hybrid, from a verified reference implementation. drop the native Goldilocks KEM candidate — inventing a KEM is the one thing the thesis forbids. "one field" does not apply to a primitive that never enters a proof; if one day it must, the ikat lens exists.

veil: split the two needs it currently conflates. private balances (P2) need commitments, nullifiers and a zheng zero-knowledge proof — hash plus proof, no FHE — and `tok/programming-model.md:165–167` already specifies exactly that. computation on encrypted data (private inference, OMR/PIR) is FHE: lattice plus circular security, the least conservative pair in the stack; defer it, label it, and fix the parameters first — at q ≈ 2⁶⁴ the HE-standard tables need n ≥ 4096 for ~128-bit classical (n = 4096 allows log q ≈ 109; n = 1024 allows ~27), so the current "n=1024 ≈ 128 bits" is wrong by two orders of magnitude of work and "n=4096 ≈ 256" is the floor, not the ceiling.

quorum: Shamir over Goldilocks is information-theoretic and stays. the VSS is rewritten hash-only: the dealer publishes a Merkle root over all n shares, each party receives its share with its path, and a zheng proof certifies that the committed shares lie on one polynomial of degree k−1. DKG is then the sum of such sharings. threshold decryption of ML-KEM has no clean construction; threshold applies to a symmetric key (share it, reconstruct under policy) or is replaced by k-of-n proofs under the identity policy. the commit-reveal randomness beacon over quorum shares is hash-only and unbiasable under an honest k.

### 5.5 the VDF

what the beacon needs: unpredictable until propose closes, unbiasable by the last mover, verifiable by everyone. the placeholder `x^(2^T) mod p` provides none of it.

- design target — hash-based proof of sequential work: beacon = H^T(seed), seed = hash of the finalized claim set; a zheng IVC (the HyperNova fold already built for settlement) certifies the chain, so verification is O(1). the only assumption is that iterated Hemera has no shortcut — hash-only, post-quantum, and it fits "one computation": the same fold machinery, the same field. choose T with a ~10× margin over the propose window; ASIC speedup is bounded by clock ratio, the limit every VDF shares.
- launch fallback if chained IVC is not production by 2026-11-05: the same H^T, verified by recomputation. every node spends T (size it to one or two seconds) once per epoch; fully hash-only, no proof system in the loop; secure as long as the delay exceeds propose-close propagation. swap in the proven form when the fold is ready.
- classical alternative — Wesolowski over imaginary-quadratic class groups (Chia): mature, transparent, O(1) proofs, not post-quantum — acceptable for a VDF, which needs no long-term secrecy, but it adds an algebra the stack does not have.
- delete option C. the isogeny VDF rests on an unknown group order that is in fact known.

## sources

- [Vitalik Buterin on AI and ECDSA, october 2026](https://www.kucoin.com/blog/vitalik-ai-warning-ecdsa-ethereum-wallet-security) · [Justin Drake, ethereum "bunker mode"](https://www.trendingtopics.eu/ethereum-bunker-mode-ai-quantum-bitcoin/)
- [Blockstream: hash-based signatures for bitcoin](https://www.dlnews.com/articles/web3/bitcoin-researchers-eye-hash-based-signatures-for-quantum-computer-proof-upgrade/) · [hash-based multi-signatures for post-quantum ethereum (eprint 2025/055)](https://eprint.iacr.org/2025/055.pdf)
- [Luo's claimed quantum attack on ML-KEM — analysis](https://postquantum.com/security-pqc/luo-ml-kem-quantum-attack-analysis/) · [horizontal fusion attacks on ML-DSA (eprint 2026/1904)](https://eprint.iacr.org/2026/1904) · [Bernstein, understanding lattice risks](https://blog.cr.yp.to/20260630-risk.html)
- [resultant-based attacks claiming Poseidon bounties (eprint 2026/150)](https://eprint.iacr.org/2026/150) · [S-box skipping (eprint 2026/1692)](https://eprint.iacr.org/2026/1692.pdf) · [weak matrices in Poseidon2 (eprint 2026/306)](https://eprint.iacr.org/2026/306) · [why ethereum walked away from Poseidon](https://www.zkm.io/blog/why-ethereum-walked-away-from-poseidon)
- internal: `hemera/specs/README.md`, `hemera/research/permutation-decision.md`, `mudra/specs/*.md`, `mudra/audit/mudra-identity-readiness.md`, `zheng/specs/execution.md`, `zheng/specs/decider.md`, `lens/specs/scalar-field.md`, `foculus/src/vdf.rs`, `cyber/launch.md`
