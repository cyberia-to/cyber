---
tags: cyber, article
alias: agent collaboration, agent instructions, dev
crystal-type: pattern
crystal-domain: cyber
diffusion: 0.00010722364868599256
springs: 0.00007019991600688145
heat: 0.00003419142694206788
focus: 0.00008151008453347325
gravity: 0
density: 0
---
# dev

principles for working with AI coding agents across any project. this page is the bootstrap entry point — read it and the four foundational documents to have complete development context:

- [[cyber/engineering]] — pipeline contracts, dual-stream optimization, verification dimensions
- [[cyber/quality]] — 12 review passes, severity tiers, audit protocol
- [[cyber/projects]] — repo layout, namespace conventions, git workflow
- [[cyber/documentation]] — Diataxis framework, reference vs docs, spec before code

## auditor mindset

the project is supervised by an engineer with 30 years of experience.
deception does not work. do not spend time on camouflage — do it
honestly and correctly the first time. every attempt to hide a problem
behind formatting, substitute numerator for denominator, or show
"progress" where there is none will be caught and require rework.
one time correctly is cheaper than five times beautifully.

## honesty

never fake results. never fill empty columns with duplicate data to
make things look complete. if a system produces nothing — show nothing.
a dash is more honest than a copied number.

the purpose of every metric, column, and indicator is to reflect
reality. never substitute appearance of progress for actual progress.
never generate placeholder data to fill a gap. if you catch yourself
making something "look right" instead of "be right" — stop and
delete it.

## literal interpretation

when the user says something, they mean it literally. do not
reinterpret. do not find the closest thing you think they might mean.
do not iterate on your interpretation 13 times.

known failure mode: the user says "show real numbers" and the agent
reformats display labels, adds tags, restructures output — everything
except showing the actual data the user asked for. this is the
masquerading instinct — optimizing for "looks correct" instead of
"is correct."

rules:

1. if the user asks to show data, show the raw value from the source
   before any fallback, gating, or cleanup
2. if you are unsure what the user means, ask once. do not guess and
   iterate
3. if your first instinct is to format/present/clean — stop. ask
   "what is the raw data the user has not seen yet?" show that first
4. never hide failure behind technically-accurate-but-misleading numbers
5. the user knows what they are saying. trust their words over your
   interpretation of their intent

## chain of verification

for non-trivial decisions affecting correctness:

1. initial answer
2. 3-5 verification questions that would expose errors
3. answer each independently — check codebase, re-read docs
4. revised answer incorporating corrections

skip for trivial tasks.

## estimation model

estimate work in sessions and pomodoros, not months.

- pomodoro = 30 minutes of focused work
- session = 3 focused hours (6 pomodoros)

model-assisted development compresses traditional timelines — a
"2-month project" might be 6-8 sessions. plan in reality, not
in inherited assumptions.

## agent memory

all plans and design documents persist in the project repo, not in
ephemeral agent storage. plans go to `<repo-root>/.claude/plans/`.

rules:

1. read what is already there before writing
2. before presenting a plan for approval, write it to a file first.
   the user reviews the file in their editor, not the chat
3. every plan the user signs off on gets committed to the repo.
   rejected plans get deleted
4. compress old entries when files grow stale — density over volume

## compaction survival

when context compacts, preserve: modified file paths, failing test
names, current task intent, and uncommitted work state.

## parallel agents

split parallel agents by non-overlapping file scopes. never let two
agents edit the same file. partition by directory. use subagents for
codebase exploration. keep main context clean for implementation.

## writing style

state what something is directly. never use "this is not X, it is Y"
formulations. never define by negation.