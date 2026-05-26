---
tags: cyb, cyber, core
alias: component particle, aip, composed interface, interactive application
crystal-type: entity
crystal-domain: cyb
---
composition as [[particle]]. the native format for interactive applications, dashboards, tools, and any knowledge that combines multiple content types into a unified, stateful experience

source format: native component language — the composition primitives of [[cyb/architecture|PureRender]]

---

## rendering

```
component definition → compile to WASM (logic) + WGSL (render) → nested render passes → GPU composite
```

a component particle compiles to a single WASM binary. logic and render are one object. the runtime instantiates the WASM module, passes it GPU resources, and the component manages its own render pass. the parent frame composites the result. nesting is efficient: each inner component owns its render budget

## the component is the contract

in the [[cybergraph]], a component particle can fuse UI and [[smart contracts]] into one binary. the component renders state; the contract enforces rules; both compile to the same WASM module. no network round-trip between frontend and logic

```
component/contract Token {
  state balances: Map<Address, u128>
  <stream>
    <text>Balance: {balances[viewer]}</text>
    <input text bind={recipient} />
    <input range bind={amount} max={balances[viewer]} />
    <action>Send -> transfer(recipient, amount)</action>
  </stream>
  fn transfer(to: Address, amount: u128) { ... }
}
```

the [[cyberlink]] between the UI and the state machine is internal to the particle. external [[cyberlinks]] connect this component particle to the particles it renders, the particles it modifies, and the neurons that created it

## in the cybergraph

component is the language of software-as-knowledge. an AIP is a component particle. a scientific instrument interface is a component particle. a governance voting surface is a component particle. they are permanent, addressable, linked objects in the graph — not apps downloaded from stores

types of component particles: [[cyb]] AIPs (oracle, brain, sense, sigma, portal), interactive molecular viewers, live market dashboards, scientific instrument control panels, genomic browsers, governance proposal interfaces, educational simulations, multimedia encyclopedias, calculation tools, collaborative annotation tools, interactive proofs

## properties

- all nine languages composable — a component particle can contain any of the other eight content types. a scientific paper as component: [[text]] body, [[formula]] equations, [[table]] datasets, [[pixels]] figures, [[vector]] diagrams — all unified under one interactive shell
- state is explicit — component particles declare their state model. reactivity only where state exists. dead code eliminated at compile time
- metered — every component particle executes under the epoch budget allocator. a malicious or infinite-looping component cannot starve other cells. safety is structural
- hot-linked — a component particle can link to live data from other particles. a table particle updates; the component re-renders. the cybergraph is the state store

## relation to other languages

component is the meta-language — it contains all others. [[text]] flows inside it. [[formula]] renders within it. [[table]] binds to it as data. [[pixels]] and [[video]] display inside it. [[vector]] composes into its layout. [[sound]] plays through it. [[struct]] configures it

see [[cyb/architecture]] for the component compilation pipeline. see [[prysm]] for the design system. see [[aip]] for the application layer. see [[smart contracts]] for the contract half of component/contract

discover all [[concepts]]
