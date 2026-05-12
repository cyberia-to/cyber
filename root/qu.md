---
tags: cyber, language
alias: Qu, quantum language, quantum circuit language
crystal-type: entity
crystal-domain: cyber
---
the quantum circuit language. describes, optimizes, and dispatches quantum computation — the co-processor interface for quantum hardware

| Op | Action |
|---|---|
| `qubit(n)` | Allocate n qubits in |0⟩ state |
| `h(q)` | Hadamard gate — create superposition |
| `cx(control, target)` | CNOT — entangle two qubits |
| `t(q)` | T gate (φ*/8 rotation) — the expensive universal gate |
| `rz(q, θ)` | Z-rotation by angle θ (encoded as F_p² phase) |
| `measure(q)` | Collapse qubit — returns classical bit |
| `barrier()` | Scheduling boundary — no reordering across |
| `phase_poly(f, qubits)` | Apply phase polynomial f(x) to register |

## why Qu is a co-processor language

[[no-cloning theorem]] makes standalone quantum computers structurally impossible:

```
Classical input → State preparation → Quantum gates → Measurement → Classical output
       ↑                                                                    ↓
       └────────────────── classical control loop ──────────────────────────┘
```

1. input: state preparation is classical (prepare |0⟩, sequence gates)
2. output: measurement is classical (collapse to bits)
3. storage: decoherence kills quantum state in microseconds
4. error correction: requires classical feedback (syndrome → decode → correct)
5. programming: circuit description is a classical object

quantum hardware is to [[nox]] what GPU is to CPU — a co-processor dispatched via host jets. the computation crosses the [[proof]] boundary: [[zheng]] proves the classical orchestration, the quantum execution is trusted hardware.

## three layers

```
Qu description (classical)
  ↓
Qu compiler (classical optimization)
  ↓
host::quantum jet (dispatch to hardware, return classical result)
```

### layer 1: description

quantum circuits as [[nox]] programs over F_p² ([[Goldilocks field]] quadratic extension, already in [[Trident]]). amplitudes are complex numbers: a + bi where i² = -1, encoded as F_p[x]/(x²+1). gate matrices are 2×2 over F_p². circuit = product of gate matrices applied to state vector.

a 3-qubit circuit:

```
q = qubit(3)
h(q[0])
cx(q[0], q[1])
cx(q[1], q[2])
result = measure(q)   // GHZ state → collapses to 000 or 111
```

this compiles to a [[nox]] program that describes the circuit structure. the program itself is classical — only the execution target is quantum.

### layer 2: compiler

T-count optimization via Reed-Muller decoding (Amy-Mosca framework):

1. extract phase polynomial f(x) with Z₈ coefficients from Clifford+T circuit
2. reduce odd coefficients mod 2 → binary vector w ∈ F₂^(2ⁿ-1)
3. decode: find nearest codeword c* in punctured RM(n-4, n)* — minimum distance problem
4. lift correction back to Z₈, add to original coefficients
5. synthesize optimized circuit with fewer T gates

the optimization uses two regimes:
- [[kuro]] (F₂): Reed-Muller codes, binary vectors, Möbius transforms, algebraic normal form
- tropical: minimum distance decoding (find nearest codeword = optimization), T-depth scheduling (graph coloring)

T gates are the bottleneck in fault-tolerant quantum computing (magic state distillation). reducing T-count directly reduces physical resource cost. the compiler runs classically on [[nox]], produces an optimized circuit, then dispatches.

### layer 3: quantum jets

host jets that dispatch to quantum hardware and return classical measurement results:

| jet | quantum algorithm | speedup | classical equivalent |
|-----|------------------|---------|---------------------|
| jet_q_grover | amplitude amplification | O(√n) vs O(n) | unstructured search |
| jet_q_shor | period finding + QFT | poly vs sub-exp | integer factoring |
| jet_q_hhl | quantum linear solver | exp (with caveats) | linear systems Ax=b |
| jet_q_vqe | variational eigensolver | heuristic | ground state energy |
| jet_q_qaoa | approx optimization | heuristic | combinatorial optimization |
| jet_q_walk | quantum random walk | O(√n) vs O(n) | graph traversal |
| jet_q_qft | quantum Fourier transform | O(n log n) vs O(n 2ⁿ) | Fourier over Z/2ⁿ |

each jet: nox prepares circuit description → host::quantum dispatches → hardware executes → measurement returns classical bits → nox continues.

the proof boundary is explicit: [[zheng]] proves "given this measurement result, the classical processing was correct." the quantum execution itself is trusted hardware — same as host::gpu or host::infer in [[rune]].

## QKD integration

[[quantum key distribution]] operates below computation — physical transport layer:

```
Alice prepares photons → quantum channel → Bob measures → classical shared key
```

BB84 protocol: security from physics ([[no-cloning theorem]]), not from computational hardness. information-theoretic guarantee — even unbounded adversary cannot break.

QKD output: classical bits. used by [[genies]] / [[nebu]] as any other key material. QKD does not change the four execution regimes — it adds a physical security layer beneath them.

| property | [[genies]] | QKD |
|----------|-----------|-----|
| security basis | math (isogeny hardness) | physics (no-cloning) |
| breaks if | math breakthrough | physics wrong |
| needs | classical CPU | quantum photon channel |
| range | internet (any distance) | fiber ~100km / satellite |
| output | classical shared key | classical shared key |

## what Qu is not

Qu is not a quantum VM — [[nox]] axioms (copyable nouns, deterministic branch, observable trace) are structurally incompatible with quantum mechanics (no-cloning, probabilistic measurement, decoherence). Qu describes circuits that execute on external quantum hardware, not inside nox.

Qu is not a simulation language — simulating n qubits requires O(2ⁿ) classical state. [[Ten]] over F_p² can simulate small circuits (< 30 qubits). Qu targets real quantum hardware, not simulation.

Qu is not a fifth execution regime — the algebra is F_p² (already in [[nebu]] / [[Trident]]). the compilation uses [[kuro]] + tropical. the execution is a host jet. no new regime needed.

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture
