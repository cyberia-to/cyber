---
tags: cyber, article
alias: engineering patterns, engineering methodology
crystal-type: pattern
crystal-domain: cyber
diffusion: 0.00011661740354397796
springs: 0.00008685778795571257
heat: 0.00003896356362984299
focus: 0.00009215875088467182
gravity: 0
density: 0
---
# engineering patterns

architectural patterns that apply across projects.

## pipeline contract

```
Stage_0 → Stage_1 → Stage_2 → ... → Stage_N → Output
```

output of stage N must be valid input for stage N+1. when modifying
a stage, verify both its input and its output still connect.

define a clear pipeline boundary — the point where the compiler/tool
ends and the runtime begins. everything before the boundary is the
build system. everything after is execution.

## dual-stream optimization

two independent optimization streams run in parallel:

1. hand-written baseline: write from first principles — algorithm +
   target machine, never from compiler output. ask "what is the
   minimum instruction sequence for this operation?" not "how can I
   improve what the compiler emitted?"

2. automated pipeline: improve codegen to approach hand baselines.
   every baseline with ratio > 1.5x is an optimization target.

the streams must stay independent. hand baselines set the floor —
the automated pipeline races toward it. when the pipeline catches up,
push the baseline lower. neither stream is dogma; both improve
continuously.

the benchmark suite is the scoreboard. regressions in either
direction are bugs.

## self-verification

every commit:

- type check / lint — zero warnings
- test suite — all tests pass
- benchmark — no regressions vs baselines
- audit — formal properties still hold (where applicable)
- if anything fails, fix before reporting done

## multi-dimensional verification

every function that targets provable execution is verified across
independent dimensions:

| dimension | source | role |
|-----------|--------|------|
| reference | ground truth implementation | generates inputs, computes expected outputs |
| automated | default build pipeline | standard compilation |
| manual | hand-optimized baseline | expert floor |
| learned | ML/neural optimizer | exploration of solution space |

four metrics compared across all dimensions:

1. correctness — output matches reference on all test inputs
2. execution speed — cycle count or runtime
3. proving time — proof generation cost (where applicable)
4. verification time — proof checking cost (where applicable)

slow code is a bug. incorrect code is a soundness hole.

## companion repo pattern

when a project splits into a compiler/tool and a runtime/executor:

- keep them in separate repos with path dependencies
- use repo-qualified paths when referencing files across repos
- after editing the upstream repo, rebuild downstream too
- patches over forks: fetch upstream, apply diff, vendor result

## sync rules

when a concept must stay synchronized across multiple locations
(spec, type checker, code generator, cost model), document the
list of locations explicitly. every change to one location requires
updating all others in the same commit.