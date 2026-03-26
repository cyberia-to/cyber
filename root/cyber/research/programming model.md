---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-26
---
# programming model

how programs work in [[cyber]]. two objects: [[cyberlinks]] (what CAN happen) and [[neurons]] (what DOES happen). no contracts. no calls. proofs compose.

## two programming objects

every other system has one programming model. [[cyber]] has two, and they are complementary:

| object | nature | programs | what it governs |
|--------|--------|----------|-----------------|
| [[cyberlink]] (UTXO) | stateless, private, proof-composed | lock scripts + semcons (type scripts) | rules of the world |
| [[neuron]] (agent) | persistent state, autonomous | [[progs]] (nox programs in [[rune]]) | beings in the world |

semcons define what CAN happen — the physics. progs define what DOES happen — the agency. one without the other does not work: semcon without prog = rules without players. prog without semcon = players without rules.

this combination exists nowhere else:

| system | UTXO programming | agent programming |
|--------|-----------------|-------------------|
| Bitcoin | script (limited) | — |
| Ethereum | — | accounts + contracts |
| Cardano | eUTXO validators | — |
| Nervos | lock + type scripts | — |
| Ergo | boxes + registers | — |
| Neptune | lock + type (STARK) | — |
| Solana | — | programs + accounts |
| cyber | lock + semcon ([[zheng]] proof) | [[progs]] (autonomous [[neurons]]) |

### cyberlink programming (UTXO layer)

the cyberlink is the value + meaning carrier. lock scripts say WHO can create it. type scripts (semcons) say WHAT rules it follows. both are [[nox]] programs verified by [[zheng]]. the neuron that created the cyberlink is hidden (private UTXO). the aggregate effect is public (BBG polynomial).

### neuron programming (agent layer)

the [[neuron]] is the actor. it has persistent state in [[BBG]] polynomial (focus, karma, balance aggregates, delegation relationships). a [[prog]] is a nox program running in [[rune]] that gives the neuron autonomous behavior — it listens for events via [[hint]], reads state via BBG queries, and creates cyberlinks in response.

the prog has no special privileges. its cyberlinks pass through the same lock + type scripts as manual ones. the prog IS the neuron's brain. what the prog decides to do, the semcons constrain whether it is allowed.

### the bridge: BBG_poly

the two layers see each other through the [[BBG]] polynomial:
- semcons (UTXO scripts) READ neuron state: `bbg_query(neurons, ν, karma)` — is this neuron reputable enough?
- progs (neuron programs) READ UTXO aggregates: `bbg_query(particles, p, energy)` — how much attention does this particle have?

the polynomial IS the shared state between the two programming models. O(1) queries, no call stack, no reentrancy.

## the primitive

the only record type is [[cyberlink]]:

```
cyberlink(ν, p, q, τ, a, v, t)
  ν = neuron         WHO acts
  p = from particle  semantic source
  q = to particle    semantic target
  τ = token          WHICH denomination (CYB / H / VOLT / AMPERE)
  a = amount         HOW MUCH value at stake
  v = valence        DIRECTION (+1 reinforce / -1 challenge)
  t = timestamp      WHEN
```

every cyberlink moves value AND conveys meaning. always both. `a > 0` always — free links do not exist. a signal is a batch of cyberlinks. nothing else enters the system.

## UTXO model

each cyberlink is a UTXO. creating a cyberlink = spending previous UTXOs + producing new ones.

```
INPUTS:
  cyberlink(ν, ν_reserve, ν_reserve, CYB, 1000, +1, t_old)

OUTPUTS:
  cyberlink(ν, article_A, article_B, CYB, 100, +1, t_now)   ← knowledge edge
  cyberlink(ν, ν_reserve, ν_reserve, CYB, 900, +1, t_now)   ← change
```

conservation: 1000 = 100 + 900. both outputs are cyberlinks. both move value and carry meaning.

balance is not a separate type. a neuron's balance = sum of all its unspent cyberlinks. idle tokens sit in reserve cyberlinks (ν → ν_reserve).

## two scripts per cyberlink

every cyberlink is validated by two [[nox]] programs:

### lock script — WHO can act

proves the neuron has the right to spend the input UTXOs.

```
neuron_lock(witness):
  secret ← hint()
  assert hemera(secret) == ν.address
  return 0
```

extensions compose via nox pattern 2 (compose):

| lock | adds |
|------|------|
| neuron | H(secret) == address |
| timelock | neuron + assert t_now ≥ unlock_time |
| delegation | neuron(delegatee) + verify chain to delegator |
| multisig | m secrets, each verified against keyset |
| recovery | multisig(friends) + timelock(delay) |

### type script — WHAT rules apply

proves conservation (plumb) AND semantic validity (semcon). always both layers:

```
type_script = plumb(τ, a) ∧ semcon(p, q, v, rules)
```

plumb layer (always present):

```
for each denomination d in {CYB, H, VOLT, AMPERE}:
  assert Σ input_amounts(d) ≥ Σ output_amounts(d)
  fee[d] = Σ inputs(d) - Σ outputs(d)
```

semcon layer (specific to the type of link):

```
semcon_knowledge:  p and q are particles, τ ∈ {CYB, VOLT}
semcon_social:     p and q are neurons, p ≠ q, τ = AMPERE
semcon_governance: p is proposal, q is option, τ = HYDROGEN
semcon_transfer:   p is sender context, q is receiver context
semcon_geo:        q is geohash, attestation in hint, τ = VOLT
semcon_card:       NFT uniqueness proof, τ = CYB
```

no plumb without semcon. no semcon without plumb. inseparable.

## state: two layers

```
PRIVATE (UTXO):     individual cyberlinks     commitment A(x) + nullifier N(x)
PUBLIC  (poly):     aggregates + derived       BBG_poly(index, key, t)
```

every signal updates both:
1. spend input UTXOs (nullify) + create output UTXOs (commit) → private
2. update [[BBG]] polynomial at affected dimensions → public

one [[zheng]] proof covers both layers.

## reading state: polynomial queries

scripts can READ [[BBG]] polynomial state without spending anything:

```
let karma = bbg_query(neurons, ν, karma)
let energy = bbg_query(particles, p, energy)
let reserves = bbg_query(pools, pool_id, reserves)
```

O(1) polynomial evaluation. one [[lens]] opening. ~200 bytes proof. replaces Ethereum view functions, Cardano reference inputs, Solana account reads. but cheaper — polynomial evaluation vs Merkle path.

## three levels of programming

### level 1: semcons (validation rules)

a semcon defines: "if someone creates a cyberlink with THIS type, HERE is what must be true."

reactive. does not initiate. validates transitions.

```
semcon_amm_swap.tri:
  fn validate(link, bbg):
    pool = bbg_query(pools, link.p)
    new_a = pool.reserve_a + link.a
    dy = pool.reserve_b - (pool.reserve_a * pool.reserve_b) / new_a
    assert output.a == dy
    assert new_a * (pool.reserve_b - dy) ≥ pool.reserve_a * pool.reserve_b
```

a DEX is not a contract. a DEX is a semcon that validates swap cyberlinks. pool state lives in [[BBG]] polynomial. rules live in the type script.

### level 2: progs (autonomous agents)

a [[prog]] is a [[nox]] program running in [[rune]]. listens for events via [[hint]]. creates cyberlinks in response. no special privileges — a prog creates cyberlinks like any neuron.

```
prog_liquidator.tri:
  fn main():
    loop:
      event = hint::cybergraph_event("health_factor < 1")
      position = bbg_query(lending, event.id)
      cyberlink(self, position.collateral, position.debt, CYB, amount, -1, now)
```

progs are neurons with programs. their cyberlinks pass through the same lock + type scripts as manual ones.

### level 3: proof composition (the key insight)

this is what makes the model strictly more powerful than call-based systems.

```
Ethereum:
  A.swap() → calls B.transfer() → calls C.approve() → calls D.callback()
  if D reverts → C reverts → B reverts → A inconsistent?
  ORDERING MATTERS. REENTRANCY POSSIBLE.

cyber:
  signal = [swap_link, transfer_link, approval_link]
  proof(swap valid) ⊕ proof(transfer valid) ⊕ proof(approval valid) → proof(signal valid)
  each cyberlink independently validated.
  proofs fold (HyperNova): 1000 proofs → 1 proof, O(1).
  NO ORDERING. NO REENTRANCY. NO CALL STACK.
```

proofs compose MATHEMATICALLY. calls compose PROCEDURALLY. mathematical composition has no side effects, no order dependency, no failure cascade.

a signal can contain cyberlinks governed by different semcons. the zheng proof validates ALL of them in one batch. composability at the signal level, not at the call stack level.

## conviction

conviction of a cyberlink grows with time:

```
conviction(link) = a × f(t_now - t_created)
```

not a stored field — derived from amount and age. the longer a cyberlink lives (unspent), the more it influences [[cyberank]].

withdrawal: spend the cyberlink UTXO → tokens return to reserve → conviction resets to zero. incentive: hold links longer = more influence.

## bandwidth

every cyberlink costs AMPERE (bandwidth). AMPERE regenerates proportional to [[focus]] (from staked HYDROGEN).

no AMPERE = cannot create cyberlinks. rate limiting is economic, not protocol-level.

## what replaces what

| Ethereum concept | cyber equivalent |
|-----------------|-----------------|
| contract deployment | register semcon (a cyberlink in the graph) |
| contract storage | [[BBG]] polynomial dimensions |
| function call | cyberlink with appropriate semcon |
| msg.sender | ν (neuron in lock script) |
| view function | bbg_query (polynomial evaluation) |
| event emission | output cyberlinks (public aggregates in BBG) |
| ERC-20 token | denomination τ with plumb conservation |
| ERC-721 NFT | card semcon with uniqueness proof |
| proxy upgrade | new semcon version (new cyberlink) |
| gas estimation | exact focus cost (per-pattern, compile-time) |
| reentrancy guard | unnecessary (no calls, only proofs) |
| flashloan | impossible (conservation enforced per signal, no intra-signal state) |

## what you can build

| application | semcon | what validates |
|-------------|--------|---------------|
| DEX (AMM) | semcon_swap | constant product, conservation |
| lending | semcon_borrow + semcon_liquidate | collateral ratio, health factor |
| oracle | semcon_oracle | attestation signature, freshness |
| NFT marketplace | semcon_card + semcon_transfer | uniqueness, ownership transfer |
| DAO | semcon_governance | voting power ∝ HYDROGEN, time bounds |
| prediction market | semcon_market | outcome resolution, payout |
| name service | semcon_name | uniqueness, ownership |
| bridge | semcon_bridge | cross-chain attestation |

every application = a semcon (type script) + optional prog (autonomous agent). no contract. no account. no call stack.

## the tradeoff

the model forbids synchronous inter-program calls. you cannot "call the DEX from inside the lending protocol." but you can: include both a lending cyberlink AND a swap cyberlink in the same signal. the zheng proof validates both atomically. composability at the signal level.

this is stricter than Ethereum (no arbitrary calls). but safer (no reentrancy, no call stack overflow, no gas estimation failures). and more powerful where it matters: privacy by default, exact costs, proof composition, polynomial state queries.

## honest assessment

| property | confidence |
|----------|-----------|
| cyberlink as only UTXO type | high — proven by Neptune's model |
| lock + type two-script model | high — Neptune, Nervos both validate |
| semcon as type script | medium-high — novel naming, proven mechanism |
| proof composition replaces calls | high — mathematically sound |
| BBG_poly queries replace view functions | medium — polynomial state unproven at scale |
| prog as autonomous agent | high — same as any neuron, no special privileges |
| no synchronous calls | design choice — limits some patterns, eliminates reentrancy class |

discover all [[concepts]]
