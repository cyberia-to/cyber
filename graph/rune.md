---
tags: cyber, cyb, language
crystal-type: entity
crystal-domain: cyber
alias: rune language, rune scripting
---
# rune

[[Rs]] syntax executed via [[Nox]] tree rewriting — the nervous system of the robot. ms-start, async, dynamic, with native access to WASM, GPU, and neural inference

rune is not a separate language. it is Rs syntax parsed to [[Nox]] nouns and interpreted via tree rewriting, extended with three capabilities pure Rs does not have:

| capability | [[Nox]] mechanism | what it does |
|---|---|---|
| `hint` | pattern 16 (non-deterministic) | async input — yields, resumes when data arrives |
| `host(target, args)` | host jet dispatch | calls WASM/GPU/ONNX — exits [[proof]] boundary, returns noun |
| `eval(noun)` | quote + reduce | runtime metaprogramming — execute a dynamically constructed formula |

## ms start

parsing Rs to a [[Nox]] noun is milliseconds — just tree construction. [[Nox]] reduction starts immediately via tree rewriting. no compilation step for interactive use. a [[prog]] that needs to react to a [[cybergraph]] event in real-time does not wait for a compiler

## three jet categories

[[Nox]] reduction is the universal executor. when reduction hits a recognized pattern, the jet mechanism dispatches:

```
Nox reduction (tree rewriting)
  │
  ├── pure jets → proven computation (14 languages)
  │     fma, ntt, p2r, lut, conservation...
  │
  ├── host jets → practical computing
  │     ├── wasm(module, fn, args)  → wasmi execution
  │     ├── gpu(shader, data)       → wgpu compute dispatch
  │     └── infer(model, input)     → burn-webnn ONNX
  │
  └── hint → async input from the world
        ├── network event ([[radio]])
        ├── user input ([[cyb]] UI)
        ├── timer (epoch tick)
        └── [[cybergraph]] change ([[particle]]/[[cyberlink]] event)
```

pure jets stay inside [[Nox]] — provable. host jets exit to the host runtime — practical. hints yield to the world — async. the rune script does not know the difference — it is just reducing nouns

## data structures as nouns

[[Nox]] nouns ARE the dynamic data structures. no heap, no garbage collector — allocation is `cons`, freeing is not referencing

| Rs lacks | [[Nox]] noun equivalent | how |
|---|---|---|
| `Vec<T>` | cons-list | `cons(head, cons(head, cons(head, nil)))` |
| `HashMap<K,V>` | Merkle tree | balanced binary tree of key-value pairs, authenticated |
| `String` | [[Hemera]] hash | content-addressed in the [[cybergraph]] — a string IS a [[particle]] |
| `Box<T>` | subtree | `cons(value, nil)` — allocation is tree growth |
| dynamic growth | `cons` | every `cons` creates new structure, immutable |

syntax sugar makes this practical:
- `[1, 2, 3]` → `cons(1, cons(2, cons(3, nil)))`
- `{ key: value }` → balanced Merkle tree
- `"text"` → `Hemera(bytes)` — a [[particle]] hash

## the proof story

every pure reduction in the script IS provable — the [[Nox]] trace captures it. host jets and hints are NOT provable — they cross the [[proof]] boundary. but the boundary is explicit and typed:

```
proven:     Tri::normalize, Arc::rank, Tok::link, Ten::matmul
not proven: hint::event, host::infer, host::gpu, host::wasm
```

the trace says: "given these hint values and these host jet results, the pure computation was correct." the hints and host results are the witnesses. the [[proof]] verifies everything except the external inputs — which is the best any system can do

## example: trading prog

```rust
fn trading_prog(pair: Hash, model: OnnxModel) {
    let threshold = 0.7;

    loop {
        // async: wait for price event
        let price = hint::cybergraph_event(pair);

        // host jet: run neural model on NPU
        let signal = host::infer(model, price.history);

        // pure: proven field arithmetic
        let confidence = Tri::normalize(signal);

        // pure: proven graph query
        let sentiment = Arc::rank(pair, 100);

        // pure: proven UTXO transition
        if confidence > threshold {
            Tok::link(me, pair, CYB, confidence * stake, +1);
        }

        // host jet: update GPU visualization
        host::gpu(chart_shader, [price, signal, sentiment]);
    }
}
```

one language ([[Rs]] syntax), one VM ([[Nox]] reduction), three execution domains: pure reductions (provable), host jets (practical), hints (async)

## example: semcon definition

```rust
semcon Causation {
    fn apply(subject: Particle, object: Particle) -> Sentence {
        let causes = Arc::resolve("causes");
        sentence [
            subject -> causes,
            causes -> object,
            object -> TRUE,
        ]
    }

    fn query(subject: Particle) -> RankedSet<Particle> {
        Arc::resolve("causes")
            |> Arc::follow(subject)
            |> Ten::ranked()
    }
}
```

[[semcons]] are first-class in rune — the grammar of [[neural language]] is native syntax

## the stack

```
neural language           ← meaning emerges from the [[cybergraph]]
──────────────────────────────────────────────────────────────
rune (Rs + hint + host)   ← nervous system: ms start, async, host access
  pure reductions         ← proven (14 [[cyb/languages]] over [[Nox]])
  host jets               ← practical (WASM, GPU, ONNX)
  hints                   ← async input from the world
──────────────────────────────────────────────────────────────
14 languages              ← proven computation over [[Nox]] patterns
```

rune does not sit ABOVE the fourteen [[cyb/languages]] — it USES them via pure [[Nox]] reduction, and EXTENDS them with host jets and hints for real-world interaction

## what rune is not

it is not a separate VM — [[Nox]] is the VM, rune is a syntax and jet configuration

it is not unprovable glue — pure reductions are proven, only host jets and hints cross the [[proof]] boundary explicitly

it is not a general-purpose scripting language — it is Rs with cybergraph-native builtins (`link`, `search`, `rank`, `focus`), [[neural language]] primitives (`semcon`, `sentence`, `motif`), and host access (`wasm`, `gpu`, `infer`)

see [[cyb/languages]] for the fourteen computation languages. see [[cyb/multiproof]] for the proving architecture. see [[neural language for superintelligence]] for the semantic layer. see [[prog]] for autonomous rune scripts
