---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# programmable polynomial state

## the question

can a polynomial commitment work as a database? not a fixed 12-dimension state for one protocol — a general-purpose authenticated store where anyone deploys new tables, indexes, and transition logic without protocol upgrades?

the answer is yes. the polynomial doesn't know what it stores. it evaluates at any point. the "schema" — which points mean what — is metadata above the commitment. the commitment provides authentication. the schema provides meaning. they are independent.

## what BBG_poly actually is

$$\text{BBG\_poly}: \mathbb{F}_p^3 \to \mathbb{F}_p$$

a function from (index, key, time) to value, committed via [[Brakedown]] lens. the current 12 evaluation dimensions (particles, axons, neurons, etc.) are a CONVENTION — the polynomial evaluates happily at index=42 or index=10000. nothing in the mathematics constrains the dimension count.

the constraint is not the polynomial. it is the CIRCUIT — the [[CCS]] constraints that validate state transitions. the cyberlink circuit says "update dimensions 0,1,2,3,4 with these specific relationships." if you want dimension 42, you need a circuit that validates transitions for dimension 42.

## the three layers

```
LAYER 1: COMMITMENT (authentication)
  BBG_poly committed via Brakedown lens
  any (key → value) pair can be committed, opened, verified
  O(1) proof, ~200 bytes, 10-50 μs
  NO SCHEMA — raw field elements at arbitrary evaluation points

LAYER 2: SCHEMA (meaning)
  which evaluation points mean what
  table definitions, field types, key spaces
  maintained by CozoDB relations or nox program metadata
  NOT committed — lives above the polynomial

LAYER 3: TRANSITION LOGIC (validity)
  what changes are allowed
  nox programs define valid state transitions per table
  zheng proves the transitions are correct
  CCS jets optimize common patterns
```

the layers are independent. changing the schema doesn't touch the commitment. changing the commitment doesn't change the schema. adding a table requires only a new nox program (layer 3) and a new CozoDB relation (layer 2). the polynomial (layer 1) extends naturally.

## deploying a new table

no protocol upgrade. no governance vote. no recompilation.

```
step 1: define the schema
  table "reputation" {
    key: neuron_id (hash, 32 bytes)
    fields: [
      score:  field (F_p)
      level:  field (F_p)
      karma:  field (F_p)
    ]
    dimension: 12  (next available)
  }

step 2: write the transition program (nox)
  reputation_update = [nox formula]:
    // read current values via Lens opening
    // validate: caller is authorized (neuron signature)
    // validate: score change is within bounds
    // validate: level derived from score (deterministic)
    // write new values to BBG_poly(12, neuron_id, t)

step 3: deploy
  publish the nox formula as a particle (content-addressed)
  register in CozoDB: :create reputation { key: Bytes, score: Float, level: Int, karma: Float }
  first signal calling the formula creates entries in BBG_poly

step 4: use
  any neuron can call the transition program
  zheng proves the nox execution was correct
  BBG_poly(12, neuron_id, t) is queryable via Lens opening
  CozoDB queries work immediately: ?[score] := reputation[key, score, _, _], key = $neuron_id
```

the table exists. it is authenticated (Lens commitment). it is queryable (CozoDB + Lens openings). it is provable ([[zheng]]). no protocol change.

## CCS jets: the efficiency breakthrough

without [[CCS jets|ccs-jets]], every user table transition runs as a generic [[nox]] program. a 500-step program generates 500 trace rows ≈ 4,000 constraints. this works but is ~10× more expensive than genesis tables.

CCS jets recognize transition PATTERNS and replace the nox trace with direct [[CCS]] constraints:

```
pattern: TRANSFER(source, target, amount)
  source' = source - amount
  target' = target + amount
  amount > 0
  3 constraints. for ANY table. automatically recognized.

pattern: INSERT(table, key, value)
  key ∉ table
  value matches schema
  commitment extended
  5 constraints. for ANY table.

pattern: UPDATE(table, key, old, new)
  key ∈ table, old matches, new replaces
  5 constraints.

pattern: AGGREGATE(table, key, delta)
  accumulator' = accumulator + delta
  2 constraints.
```

the result:

| table type | constraints per operation |
|---|---|
| genesis (particles, axons) — level 1 CCS jet | ~3,200 (hand-optimized) |
| user table with TRANSFER pattern — level 2 jet | 3 |
| user table with INSERT pattern — level 2 jet | 5 |
| user table with complex custom logic — no jet | ~4,000-50,000 |

user tables with standard operations are CHEAPER than genesis tables. the genesis tables pay ~3,200 constraints because they update 4-5 dimensions simultaneously. a simple user-table transfer touching one dimension costs 3 constraints.

## the genesis tables are just the first jets

the 12 [[BBG]] dimensions (particles, axons_out, axons_in, neurons, locations, coins, cards, files, time, signals, commitments, nullifiers) are not architectural — they are the first batch of CCS jets. the cyberlink transition (~3,200 constraints) is a hand-optimized level-1 CCS jet.

```
genesis = { dimension: 0-11, jet: hand-optimized CCS, deployed: genesis block }
user    = { dimension: 12+,  jet: automatic pattern match, deployed: any time }
```

if a user table becomes widely used, a level-1 CCS jet can be added for it (protocol upgrade — like adding a nox jet). but level-2 pattern jets work AUTOMATICALLY for standard operations, no upgrade needed.

## the database analogy

| traditional database | polynomial state |
|---|---|
| CREATE TABLE | deploy nox transition program |
| schema | CozoDB relation + nox program metadata |
| index | polynomial evaluation dimension |
| row | evaluation point (key) in a dimension |
| column | field within the value at an evaluation point |
| INSERT | nox program executing INSERT pattern → CCS jet (5 constraints) |
| UPDATE | nox program executing UPDATE pattern → CCS jet (5 constraints) |
| SELECT | Lens opening (~200 bytes, 10-50 μs) |
| JOIN | batch Lens opening across dimensions (structural — same polynomial) |
| WHERE | verifiable query compilation (CozoDB → CCS) |
| query optimizer | CCS jet recognition (pattern match) |
| fast path (index scan) | CCS jet (optimized encoding) |
| slow path (full scan) | generic nox execution proof |
| authentication | Lens binding (every query is cryptographically verified) |
| replication | π-weighted (storage follows attention) |
| backup | signal-first (replay signals → reconstruct state) |

every database operation has a polynomial equivalent. the key difference: every query produces a CRYPTOGRAPHIC PROOF. every state change is VERIFIED. there is no trusted database server — the polynomial commitment IS the trust.

## what this enables

### 1. permissionless table creation

any [[neurons|neuron]] can deploy a table. the table is immediately authenticated (lens), queryable (CozoDB), and provable ([[zheng]]). no approval process. the [[focus]] cost of creating entries provides the spam filter.

### 2. composable state

different tables share the same polynomial. a query that JOINs across tables is one batch Lens opening — same polynomial, different dimensions. cross-table consistency is structural (definitional, not proven). this is impossible with separate databases.

### 3. programmable transitions

the transition logic IS a [[nox]] program. anything nox can compute can be a state transition. conditional updates, multi-table transactions, recursive queries, neural network inference — all expressible, all provable.

### 4. verifiable queries

any [[CozoDB]] query compiles to polynomial opening proofs (see [[query]]). the result is cryptographically guaranteed. a phone app querying a remote node gets a proof with every response. no trust required.

### 5. retroactive jet optimization

a popular user table can get a CCS jet added later. the table doesn't change — the PROOF becomes more efficient. existing data, existing queries, existing transitions — all work as before. the jet makes new proofs smaller.

## how it scales

at [[Avogadro scale]] ($10^{23}$ particles), the global polynomial is composed from shard polynomials (see [[data structures for polynomial state]]):

$$\text{BBG\_root} = \text{compose}(C_1, C_2, \ldots, C_S)$$

each shard commits to a namespace range. composition is O(S) field operations. user tables are sharded the same way — their evaluation points distribute across shards by key hash.

```
shard assignment:
  shard_id(dimension, key) = H(dimension, key) mod S

a query for (dimension=12, key=neuron_X):
  route to shard containing H(12, neuron_X)
  Lens opening within that shard
  shard composition proof verifies against BBG_root
```

no separate sharding scheme for user tables. the polynomial handles it uniformly.

## what this is NOT

this is not a general-purpose SQL database that happens to be authenticated. it is a POLYNOMIAL STATE with database operations mapped to polynomial evaluations. the differences:

- **no ad-hoc schema changes**: once a table is deployed, its schema (nox program) is immutable (content-addressed). deploying a new version = deploying a new program = new dimension. migrations are explicit, not ALTER TABLE.

- **no free reads**: every read is a Lens opening. on a local node, this is O(1) field ops (fast). over the network, it costs ~200 bytes bandwidth. there is no "free SELECT" — every query is authenticated.

- **no ACID transactions across tables**: a single [[signal]] can update multiple dimensions (like a cyberlink updates 5), but the transition program must be a single nox formula. there is no BEGIN/COMMIT/ROLLBACK across independent programs.

- **no NULL**: polynomial evaluations at unused points return 0 (the field zero element). there is no NULL/missing distinction unless the schema encodes it explicitly (e.g., using a sentinel value).

the tradeoff: less flexible than PostgreSQL, more powerful than any blockchain state. every operation is authenticated, provable, and composable. the schema is code. the index is a polynomial. the proof is mathematics.

## open questions

1. **dimension allocation**: who assigns dimension numbers? sequential allocation (next available) is simple but wastes space if tables are abandoned. content-addressed dimensions (dimension = H(table_name)) avoid collision but produce sparse evaluation domains — does Brakedown handle sparse domains efficiently?

2. **schema evolution**: how does a table add a field? deploy a new nox program with the extended schema, migrate data from old dimension to new. can this be done lazily (read from old, write to new, garbage-collect old)?

3. **access control**: who can write to a user table? the nox program defines authorization (e.g., "only the neuron who created the key can update it"). but the polynomial itself has no access control — authorization is purely in the transition logic. is this sufficient?

4. **garbage collection**: abandoned tables (zero focus, no reads) waste polynomial evaluation points. can dimensions be reclaimed? this requires proving the dimension is empty — a completeness proof over the dimension showing all evaluation points are zero.

5. **CCS jet safety**: automatic pattern recognition (level 2) must be conservative — a false positive (matching the wrong pattern) would be unsound. what is the formal criterion for safe pattern matching? can it be proved that the recognized pattern is equivalent to the nox program for all inputs?

see [[ccs-jets]] for the jet mechanism, [[BBG]] for the polynomial state architecture, [[nox]] for the transition language, [[query]] for verifiable queries, [[data structures for polynomial state]] for shard composition, [[CozoDB]] for the query layer, [[knowledge capacity]] for the capacity limits
