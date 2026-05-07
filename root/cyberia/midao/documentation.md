---
tags: cyber, article
alias: documentation methodology, diataxis, docs
crystal-type: pattern
crystal-domain: cyber
---
# documentation methodology

how to organize documentation across projects.

## diataxis framework

all documentation follows [[https://diataxis.fr][Diataxis]] — four types
organized by two axes.

|  | learning | working |
|---|---|---|
| practical | tutorials | how-to guides |
| theoretical | explanation | reference |

### tutorials (learning-oriented)

teach by doing. the reader follows steps and builds something.
the tutorial controls the experience. no choices, no alternatives.
success is guaranteed if the reader follows along.

### how-to guides (task-oriented)

solve a specific problem. the reader already knows what they want
to do. steps are direct. no teaching, no background. assumes
competence.

### reference (information-oriented)

describe the machinery. types, functions, APIs, configuration.
austere and complete. organized by the structure of the code, not
by the reader's journey.

### explanation (understanding-oriented)

illuminate concepts. why things work the way they do. background,
context, design decisions. not tied to a specific task.

## directory layout

```
docs/
├── tutorials/       learning by building
├── guides/          task-oriented how-tos
├── explanation/     conceptual understanding
└── README.md        index with links to all four quadrants

reference/           canonical specification (separate from docs/)
├── language.md      types, operators, builtins
├── grammar.md       formal grammar
├── ir.md            intermediate representation
├── props/           design proposals (not yet spec)
│   └── README.md    proposal lifecycle: draft → accepted → rejected → implemented
└── quality.md       review methodology
```

## source of truth

`reference/` is canonical. if reference/ and code disagree, resolve
in reference/ first, then propagate to code. if implementation reveals
the reference is wrong or incomplete, update the reference to match
reality.

reference/ is not documentation — it is specification. docs/ explains
and teaches. reference/ defines.

## design proposals

proposals for future changes live in `reference/props/`. each proposal
is a standalone markdown file with status frontmatter:

| status | meaning |
|--------|---------|
| draft | idea captured, open for discussion |
| accepted | approved — ready to implement and move to spec |
| rejected | decided against, kept for rationale |
| implemented | done — migrated to the relevant spec file |

proposals are not spec. they document desire before commitment.

## spec before code

write the reference entry before writing the implementation. the spec
is the contract. the code fulfills it. if you cannot specify what a
feature does before building it, you do not understand it yet.