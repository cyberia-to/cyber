---
tags: cyb, cyber, core
alias: struct particle, structured data, json, toml
crystal-type: entity
crystal-domain: cyb
---
trees, configurations, records, and schemas as [[particle]]. the native format for machine-readable knowledge in the [[cybergraph]]

source format: JSON, TOML — any hierarchical key-value structure

---

## rendering

```
json/toml source → parse → tree layout → collapsible glyph tree → GPU render
```

nodes expand and collapse. keys and values have distinct styling. depth encoded visually. the robot renders any struct particle as an interactive tree regardless of nesting depth or key count

## in the cybergraph

struct is how machines describe their own state — and how the graph describes complex objects with named parts

types of struct particles: smart contract ABIs, network configurations, protocol parameters, API schemas, scientific metadata, genomic annotations, experimental conditions, machine learning model configs, governance proposals, identity documents

a struct particle is often the metadata companion to another particle: the pixels particle of a satellite image may have a struct particle linked that contains coordinates, timestamp, sensor calibration, and resolution

## properties

- machine-readable natively — parseable by any conformant JSON or TOML parser without transformation
- schema-flexible — struct does not require a fixed schema. the [[cybergraph]] discovers schema by topology: particles that share struct shapes cluster via [[motifs]]
- queryable by [[datalog]] — any key path in a struct particle is accessible as a datalog term
- composable — struct particles embed in [[component]] particles as data sources for tables and forms

## relation to other languages

struct is the configuration language of the cybergraph. [[text]] carries argument; struct carries specification. a [[component]] reads a struct and renders it as interactive form fields. [[datalog]] queries struct fields directly

see [[json]] for the primary source format. see [[table]] for 2D structured data. see [[component]] for interactive composition

discover all [[concepts]]
