---
tags: cyber
crystal-type: entity
crystal-domain: computer science
stake: 5088110459094623
---
dynamic async scripting [[language]] built on [[rust]] — the orchestration glue between human intent and thirteen proven [[cyb/languages]]

the thirteen computation languages are algebraically complete but individually specialized. each has its own type system, its own algebra, its own proof path. rune is what binds them: the unprovable layer above the proof boundary that calls into any algebra, composes results across languages, and writes back to the [[cybergraph]]

where the thirteen languages answer "what CAN be computed and proven," rune answers "what does the USER want to do." it combines [[Arc]] (graph traversal), [[Inf]] (query), and [[Nox]] structural patterns (composition) in a dynamic scripting syntax with first-class support for [[neural language]] primitives: [[semcons]], [[sentences]], [[motifs]], [[linkchains]]

```rune
semcon Causation {
    fn apply(subject: Particle, object: Particle) -> Sentence {
        let causes = resolve("causes");
        sentence [
            subject -> causes,
            causes -> object,
            object -> TRUE,
        ]
    }

    fn query(subject: Particle) -> RankedSet<Particle> {
        resolve("causes")
            |> subject.follow()
            |> ranked()
    }
}
```

the three-layer stack:

```
computation (13 languages)  ← proven algebras — each a type system over nox patterns
rune                        ← orchestration glue — dynamic, async, neural-native
neural language             ← meaning — emerges from the cybergraph at scale
```

rune runs inside the [[cyb]] runtime and [[neural]] — callable from executable [[particles]] via the standard `ctx` API. async by default: every graph operation is non-blocking, suited for real-time [[cyberlink]] construction and streaming queries. rune scripts are not proven — they orchestrate proven computation. a rune program may invoke [[Tri]] arithmetic, [[Ten]] inference, [[Wav]] signal processing, and [[Arc]] graph traversal in a single pipeline, with each segment independently provable through [[nox]] pattern trees

use cases: robot automation, [[cyberlink]] construction on schedule, particle monitoring, inference piping, sigma position management, prog scripting, natural language → graph operations

see [[neural language for superintelligence]] for the full language design
see [[cyb/languages]] for the thirteen computation languages and the algebraic completeness argument
