---
tags: cyber
crystal-type: entity
crystal-domain: computer science
stake: 5088110459094623
---
dynamic async scripting [[language]] built on [[rust]] for [[cybergraph]] operations

where [[trident]] compiles to the proof VM, rune speaks to humans and AIs who construct, query, and reason over the [[cybergraph]]. first-class support for [[neural language]] primitives: [[semcons]], [[sentences]], [[motifs]], [[linkchains]]

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

rune runs inside the [[cyb]] runtime and [[neural]] — callable from executable [[particles]] via the standard `ctx` API. async by default: every graph operation is non-blocking, suited for real-time [[cyberlink]] construction and streaming queries

see [[neural language for superintelligence]] for the full language design
