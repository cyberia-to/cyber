---
tags: cyber, article
alias: document refinement, spec polishing, cleanup principles
crystal-type: pattern
crystal-domain: cyber
---
# refinement

principles for turning cluttered documents into finished specs. extracted from real editing sessions — each principle solved a recurring problem that wasted time.

## kill the zoo

when multiple variants of the same thing exist, collapse to a canonical set. five encodings instead of a GGUF zoo. one program slot instead of program + program-native. fewer options = stronger each option.

the urge to keep variants "just in case" is the enemy. every variant doubles the testing surface, documentation burden, and reader confusion. if you can define a conversion path from variant to canonical form — kill the variant, keep the converter.

## one concept, one home

if an idea appears in two places, one is wrong or both are incomplete. find the natural home for each concept and put it there once. "behavior is code" as a standalone section AND as program intro text = duplication. merge into the stronger location, delete the weaker.

test: grep the document for your key phrases. if any phrase appears twice, you have a merge to do.

## call things what they are

if a name needs parenthetical explanation, the name is wrong. "tri ([[trident]])" means "tri" is not self-sufficient — fix the context so "tri" is clear, or use "trident" directly. "parts" that are actually files should be called "files". "mmap" that is actually zero-copy load should be called "load".

the parenthetical test: remove all parentheses from a sentence. if the sentence still works, the parenthetical was bloat. if it breaks, the wrong word is outside the parentheses.

## remove the scaffolding

transitional structures that helped during design (dual options, compatibility shims, "legacy" alternatives) must go when the design settles. scaffolding left in place becomes permanent confusion. if a reader asks "when would I use the other option?" and the answer is "never" — delete it.

## inline over monolith

one giant example that demonstrates everything forces the reader to hold the entire structure in memory while reading any part. replace with inline examples — one per section, right where the concept is explained. the reader sees what they need when they need it.

exception: a runtime load sequence or full file layout benefits from one unified view. put it at the end as a reference, after all inline examples.

## hunt every violation

if a document declares a rule ("no floats", "integers only", "frozen spec"), audit every line for violations. a rule stated but not enforced is worse than no rule — it teaches the reader that rules here are aspirational. find every float hiding in an epsilon, every temperature stored as 0.7, every eval score as 0.991. fix them all. then document the integer conventions in one place.

## real code, not pseudocode

if an example looks like code but cannot compile or parse, it lies to the reader. pseudocode that resembles a real language creates false confidence — "I could run this" when they cannot. use actual syntax from the actual language, or use plaintext prose. never the uncanny valley between them.

## structural mirroring

the order of items in a table should match the order in the frontmatter, the order of sections in the document, and the order in the runtime sequence. when all orderings agree, the reader builds one mental model. when they disagree, the reader spends energy on mapping instead of understanding.

check: list every place where items appear in sequence. do all sequences match?

## state the positive

"not hardcoded" describes infinite things. "fully parametric" describes one thing. negation hides meaning behind the thing being negated — the reader must first understand X, then invert it. positive statements land directly.

pattern to catch: "not X but Y" → just say Y. "without X" → say what it has instead. "unlike X" in a comparison table is fine — comparisons require a reference point.

## visual aesthetics = credibility

a spec that wraps ugly on screen loses trust before the reader starts. long inline TOML that breaks at random points → vertical tables with aligned keys. dense paragraphs → short sentences with whitespace. tables where every column has purpose and nothing wraps.

render the document and look at it. if something looks cluttered, it is cluttered — regardless of whether the content is correct.

## delete before adding

when something feels wrong, the instinct is to add explanation. the fix is usually to remove. a confusing section with an added clarification paragraph = two things to read instead of one. a confusing section deleted and its core idea merged into an existing section = net reduction in reader effort.

metric: if your edit makes the document longer, verify that every added line carries information that did not exist anywhere in the document before.

## multiple focused passes

one giant cleanup pass misses things because your eye cannot track naming, structure, terminology, formatting, and logic simultaneously. split into focused passes:

1. structure — section order, heading levels, table of contents
2. terminology — every term used consistently, same name everywhere
3. examples — every example is real, parseable, matches the spec it illustrates
4. integer/rule discipline — every stated constraint is enforced in every example
5. links — every wiki-link resolves, cross-references are bidirectional
6. negation — no "not X" formulations, no defining by what something lacks
7. visual — render and read on screen, check wrapping, alignment, whitespace

each pass catches what the previous ones missed. a document needs 5-7 passes to reach finished state. this is normal — not a sign that the first pass was bad.

## the seven-round pattern

round 1: get the content right (facts, structure, completeness).
round 2: kill the zoo (collapse variants, remove legacy).
round 3: naming audit (call things what they are, remove parenthetical bloat).
round 4: rule enforcement (hunt every violation of stated constraints).
round 5: examples audit (real code, inline placement, structural mirroring).
round 6: negation and style (state the positive, visual aesthetics).
round 7: fresh eyes (read as if seeing it for the first time, catch what all previous rounds missed).

each round gets shorter. round 1 is heavy rewriting. round 7 is minor fixes. if round 7 finds major issues, the earlier rounds were not thorough enough — go back.
