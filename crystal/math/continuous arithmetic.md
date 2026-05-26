---
tags: math, cyber, nox
crystal-type: entity
crystal-domain: math
alias: continuous arithmetic, eml, elementary functions, continuous Sheffer stroke
---

# continuous arithmetic

## the primitive question

every branch of mathematics rests on a small set of operations. natural numbers need successor. Boolean logic needs NAND. linear algebra needs matrix multiplication. the question "what is the smallest generating set?" reveals deep structure.

for continuous mathematics — the world of exp, ln, sin, cos, sqrt, and their compositions — this question remained open for four centuries. elementary functions are the workhorses of science: differential equations, Fourier analysis, orbital mechanics, signal processing, neural networks. each function comes with its own rules, its own identities, its own hardware unit. a scientific calculator has 36 buttons. a CPU has separate instructions for each.

in 2026, Andrzej Odrzywołek showed that one binary operator suffices.

## the discovery

$$\text{eml}(x, y) = e^x - \ln y$$

Exp Minus Log. together with the constant 1, this single operator generates all 36 standard elementary functions: arithmetic operations (+, -, ×, /), exponentiation, logarithms, roots, trigonometric functions and their inverses, hyperbolic functions and their inverses, and the fundamental constants e, φ*, and i.

a two-button calculator — eml and 1 — computes everything a scientific calculator can.

source: "All elementary functions from a single operator" (arXiv:2603.21852, April 2026)

## historical path

1614 — [[John Napier]] invents [[logarithms]]. multiplication reduces to addition: x × y = exp(ln x + ln y). two operations become one, at the cost of table lookup.

1722 — [[Roger Cotes]] establishes the exp-log representation of trigonometric functions. cosine and sine become faces of the complex exponential.

1748 — [[Leonhard Euler]] publishes the formula e^(iφ) = cos φ + i sin φ. the entire world of trigonometry collapses into one identity. all trig functions are redundant — complex exponential is sufficient.

1835 — [[Joseph Liouville]] formalizes elementary functions as finite compositions of exp, ln, algebraic operations, and constants. the field is defined.

2026 — Odrzywołek: even exp, ln, and arithmetic are redundant. one operator generates the field.

four centuries from many operations to one. the reduction path: 36 → 7 (Wolfram) → 6 (Calc 3) → 4 (Calc 2) → 3 (Calc 0) → 2 (eml + constant 1).

## three Sheffer strokes

a [[Sheffer stroke]] is a single operator from which all operations in a domain can be derived. Henry Sheffer (1913) showed NAND suffices for Boolean algebra. the existence of Sheffer strokes in other domains was an open question.

three are now known:

| domain | operator | constant | generates |
|--------|---------|----------|-----------|
| Boolean (0/1) | NAND(x, y) = ¬(x ∧ y) | none | all logic gates |
| continuous (R/C) | eml(x, y) = exp(x) - ln(y) | 1 | all elementary functions |
| discrete (F_p) | unknown | — | open question |

the parallel is structural, not metaphorical. NAND circuits are networks of identical gates. eml circuits are trees of identical nodes. in both cases, universality arises from a single asymmetric binary operator applied to its own outputs.

no single operator is known for finite field arithmetic. the six operations (add, sub, mul, inv, eq, lt) may be irreducible, or a discrete Sheffer stroke may await discovery.

## the grammar

every eml expression is a binary tree:

$$S \to 1 \mid \text{eml}(S, S)$$

a context-free language. leaves are the constant 1 or input variables. internal nodes are all identical: apply eml. the structure is isomorphic to full binary trees, counted by [[Catalan numbers]].

this uniformity has consequences:
- enumeration is tractable (Catalan structure is well-studied)
- representation is canonical (one tree per expression, up to simplification)
- search is systematic (parameterize and optimize over tree space)
- hardware is homogeneous (one circuit element, tiled)

## key constructions

| function | eml construction | tree depth |
|----------|-----------------|-----------|
| e | eml(1, 1) | 1 |
| exp(x) | eml(x, 1) | 1 |
| ln(z) | eml(1, eml(eml(1, z), 1)) | 3 |
| 0 | exp(x) - exp(x) via eml compositions | 5-7 |
| -1 | via ln and exp compositions | 7-9 |
| φ* | via Euler identity, complex domain | deep |
| i | via ln(-1) = iφ* | deep |
| x + y | via ln(exp(x) × exp(y)) | 11-19 |
| x × y | via exp(ln(x) + ln(y)) | 17 |
| sin(x) | via Euler: (e^(ix) - e^(-ix)) / 2i | deep |

basic functions (exp, ln, e) have shallow trees. arithmetic operations (+, -, ×, /) require medium depth. trigonometric functions require deep trees — they emerge through the complex exponential route.

internal computation operates over C (complex numbers). even for real-valued functions like sin(x), the intermediate steps pass through complex arithmetic via Euler's formula. this is analogous to quantum computing using complex amplitudes to compute real probabilities.

## non-commutativity

eml(x, y) ≠ eml(y, x). exp(x) - ln(y) ≠ exp(y) - ln(x).

this asymmetry is essential. symmetric binary operators (like addition, multiplication, average) cannot generate all elementary functions. the proof: a symmetric operator f(x,y) = f(y,x) preserves symmetry under composition, but elementary functions include asymmetric operations (subtraction, division, exponentiation). asymmetry in the generator is necessary.

eml achieves asymmetry by combining exp (which grows) with ln (which shrinks). the operator simultaneously has access to both sides of the exp-log duality.

## the ternary candidate

a ternary variant exists:

$$T(x, y, z) = e^{x-y} \cdot \frac{\ln z}{\ln x}$$

the unique property: T(x, x, x) = 1 for any x. the constant 1 is self-generated — no external constant needed. the grammar becomes S → T(S, S, S) with variables as the only terminals.

| property | eml (binary) | T (ternary) |
|----------|-------------|-------------|
| inputs | 2 | 3 |
| constant required | 1 (external) | none (self-generated) |
| leaves per depth d | 2^d | 3^d |
| self-contained | no | yes |
| verified | yes (2026) | candidate (in preparation) |

T is algebraically cleaner: no external dependency, closed under composition, higher combinatorial density. if verified, T would be the preferred continuous Sheffer stroke.

verification status: "in preparation" (Odrzywołek, ref. 47). the bootstrap path — from T(x,x,x) = 1 to all elementary functions — is not yet constructively demonstrated.

## symbolic regression

classical symbolic regression searches over heterogeneous grammars: many distinct operators (+, -, ×, /, exp, ln, sin, ...), each with its own arity and rules. the search space is irregular and hard to parameterize.

eml collapses this to a uniform search space: binary trees of identical nodes. a "master formula" of depth n has 5 × 2^n - 6 parameters (linear combinations at each input). train all parameters via gradient descent (Adam optimizer), then snap to binary values (0 or 1). when the underlying law is elementary, trained weights recover the exact formula.

demonstrated results (Odrzywołek 2026):
- exact recovery of ln(x) from numerical data, depth 3
- 100% recovery rate at depths 1-2
- ~25% at depth 2, decreasing with depth
- correct basins of attraction exist even at depth 5-6

this bridges symbolic AI (exact closed-form expressions) and neural AI (gradient-trainable architectures). an eml tree is simultaneously a mathematical formula and a neural network. the same structure that represents knowledge can discover knowledge.

## analog circuits

eml defines a circuit element: two inputs (x, y), one output (exp(x) - ln(y)). networks of these elements compute any elementary function. table 3 in the paper shows the symbol alongside NAND gate, operational amplifier, and transistor.

where NAND gates tile to build digital processors, eml elements tile to build analog mathematical computers. one component, universal capability. potential applications: scientific instruments, signal processing, control systems — anywhere mathematical precision matters and digital approximation is insufficient.

## [[determinism]] boundary

eml operates over R (reals) or C (complex numbers). numeric evaluation of exp and ln produces values that depend on hardware implementation: IEEE754 rounding, FPU architecture (x87 vs SSE vs ARM), precision settings. the same eml(x, y) may produce different floating-point results on different machines.

for systems requiring [[determinism]] — same input produces same output on any machine — numeric eml is inadmissible as a primitive operation. symbolic eml (tree construction and manipulation) preserves determinism: the tree IS the result, identical on all machines.

the boundary: symbolic eml = deterministic, provable, content-addressable. numeric eml = hardware-dependent, approximate, outside the proof boundary.

systems built on [[provable memory]] and [[STARK]] proofs adopt symbolic eml as representation and defer numeric evaluation to the hardware layer — where approximation is acceptable (display, actuation, sensor thresholds) and determinism is not required.

## connection to computation

a [[noun]] — a binary tree of atoms and cells — has the same grammar as an eml expression:

$$\text{noun} = \text{atom} \mid \text{cell}(\text{noun}, \text{noun})$$
$$\text{eml expression} = 1 \mid \text{eml}(\text{eml expression}, \text{eml expression})$$

[[Catalan numbers]] count both. the data structure that stores computation (noun) is isomorphic to the data structure that stores continuous mathematics (eml tree). this is not coincidence — both are full binary trees over identical elements.

consequence: any system that processes nouns can process eml expressions without modification. the representation, storage, hashing, commitment, and proof infrastructure that works for computation also works for continuous mathematics. no new data type, no new commitment scheme, no new proof system.

eml trees as nouns: build with cell construction, navigate with tree operations, hash with content addressing, commit with polynomial commitments, prove with STARK traces. all existing infrastructure, zero extension.

## open questions

- does the ternary T generate all elementary functions? constructive bootstrap needed
- does a binary Sheffer stroke without constant exist? conjectured negative
- can eml serve as universal activation function for neural networks?
- does a Sheffer stroke exist for special functions (Bessel, Gamma, hypergeometric)?
- can eml-based symbolic regression discover physical laws from experimental data?
- optimal depth/complexity trade-offs for practical symbolic regression
- does a discrete Sheffer stroke exist for finite field arithmetic?
- analog eml circuits: practical realization and precision characteristics
