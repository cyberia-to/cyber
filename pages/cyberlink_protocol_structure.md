## protocol structure insights
### 1. creation model options

**option a — cyberlinks-only creation**
- particles cannot be created independently; every new particle must be the target or source of a cyberlink from an existing particle
- ensures strict graph continuity, preserving strong connectivity for the collective focus theorem
- simplifies data availability proofs since every particle is born linked to the graph's current frontier
- downside: bootstrap phase requires at least one genesis particle or controlled seed set

**option b — dual creation (particles & cyberlinks)**
- allows creation of unlinked particles ("orphans") in addition to cyberlinks
- increases flexibility — useful for staging data, private preparation, or pre-commit phases
- risk: too many unlinked particles can fragment focus and increase storage of unused nodes
- requires periodic pruning or decay to keep orphan set bounded

**trade-off summary:**
- cyberlinks-only = maximal structural discipline, better convergence guarantees
- dual creation = more expressive, but needs governance or automated decay to prevent graph dilution
### 2. hanging cyberlinks vs strict connectivity

**strict connectivity**
- new cyberlinks must attach to an existing particle (previously finalised or at least visible)
- preserves strong connectivity invariant from the first step
- ensures that every random walk can traverse newly added structure without special cases

**hanging (free-floating) cyberlinks**
- cyberlinks can connect to particles that don't yet exist or are not linked to the current graph
- enables speculative linking (future references, pre-loading network knowledge)
- risk: creates dangling edges that require later resolution; increases verification complexity
- might require a TTL or bonding mechanism to prevent abuse

**design consideration:**
- if the protocol enforces *eventual resolution* of hanging links (e.g., missing endpoint must be created within N epochs), they could be a powerful speculative tool without harming integrity
### 3. recommended synthesis
- adopt **cyberlinks-only creation** for the mainnet consensus-critical path — guarantees strong connectivity and simplifies convergence proofs under CFT
- allow **temporary orphan staging** in mempool or local state, but they are not part of consensus until linked
- permit **future-pointing hanging cyberlinks** only with a stake bond and expiry, so speculative graph shaping is possible without spam risk
### 4. interaction with economic & state models
- in a cyberlinks-only model, all focus rewards directly trace to link creation, aligning perfectly with the minting-for-focus rule【57†source】
- strict connectivity means every new unit of attention (Δπ) is measurable and attributable, strengthening the π-minting theorem【54†source】
- authenticated graph data structures can prove existence and weight of every link without handling orphan cases【59†source】
- data availability is simplified since all new objects are reachable from recent state roots【58†source】
### 5. security and performance implications
- strict connectivity limits adversarial surface for creating unreachable, malicious, or hidden data objects
- hanging edges, if allowed, must be bounded in number and lifetime to avoid denial-of-service via massive unresolved reference sets
- cyberlinks-only finalisation path reduces proof size and simplifies light client logic