# Comprehensive Research on 2D Layout Systems — Past, Present, and a Design for the Future

## Table of Contents

1. [The Fundamental Problem](#1-the-fundamental-problem)
2. [Historical Lineage](#2-historical-lineage)
3. [Deep Analysis of Major Systems](#3-deep-analysis-of-major-systems)
4. [Emerging Solutions](#4-emerging-solutions)
5. [Comparative Matrix](#5-comparative-matrix)
6. [Key Design Tensions & Tradeoffs](#6-key-design-tensions--tradeoffs)
7. [Brainstorm: Essential Properties for Future Layout](#7-brainstorm-essential-properties-for-future-layout)
8. [Proposed System: Frame — A Layout System for Centuries](#8-proposed-system-frame--a-layout-system-for-centuries)

---

## 1. The Fundamental Problem

Every 2D layout system must answer five fundamental questions:

| # | Question | Spectrum |
|---|----------|----------|
| 1 | **SIZE** — How big is each element? | Intrinsic (content-determined) ↔ Extrinsic (container-imposed) |
| 2 | **POSITION** — Where does each element go? | Flow (automatic) ↔ Explicit (coordinates) |
| 3 | **NEGOTIATE** — Who decides, parent or child? | Top-down ↔ Bottom-up ↔ Bidirectional |
| 4 | **OVERFLOW** — What happens when it doesn't fit? | Clip, scroll, wrap, shrink |
| 5 | **RELATION** — Can elements reference each other? | Independent ↔ Relational |

The design space is enormous, and every system makes different tradeoffs. Modern systems fail not because they pick wrong answers, but because they accumulate too many *rules* instead of building on sound *axioms*. Only topological and geometric foundations — rectangle algebra, alignments, proportional division — will endure indefinitely. The math doesn't expire.

---

## 2. Historical Lineage

```
1978  TeX "Boxes & Glue"
       │   Rigid boxes + flexible glue (min/preferred/max)
       │   Single best abstraction for 1D space distribution
       │
1984  Smalltalk Morphic — constraint-based, bidirectional
       │
1990s HTML Flow Layout — block + inline, document-centric
       │
1996  CSS Box Model — margin → border → padding → content
       │   Float-based hacks for decades
       │
1997  Java Swing LayoutManagers — composable layout managers
       │
2003  WPF/XAML — StackPanel, Grid, Canvas, DockPanel
       │   MeasureOverride + ArrangeOverride protocol
       │
2006  Apple Auto Layout (Cassowary constraint solver)
       │   Linear constraints, O(n³) worst case
       │
2014  CSS Flexbox — 1D layout done right
       │
2016  Yoga (Facebook) — cross-platform Flexbox in C
       │
2017  CSS Grid — native 2D grid layout
       │
2019  SwiftUI Layout Protocol — propose → choose → place
       │
2021  Jetpack Compose — single-pass measure + intrinsics
       │
2022  Typst Layout Engine — modern document layout, simpler than TeX
       │
2023  CSS Anchor Positioning, Container Queries
       │
2024  Taffy (Rust), Morphorm — portable layout libraries
       │
       ▼
    Future compositional layout as a portable, renderer-independent protocol
```

---

## 3. Deep Analysis of Major Systems

### 3.1 TeX — Boxes and Glue (1978)

**Model:** Everything is a box (hbox, vbox). Between boxes is "glue" — a spring with natural width, stretch, and shrink.

```
Glue = {
  natural:  12pt,    // preferred size
  stretch:  6pt,     // can grow by this much
  shrink:   4pt      // can shrink by this much
}
```

Infinity orders (`fil`, `fill`, `filll`) allow priority-based space distribution: infinite stretch at order 2 completely overrides finite stretch at order 1. This elegantly handles centering, justification, and push-to-edge spacing with a single unified primitive.

**Layout algorithm:**
1. Collect all boxes and glue in a line
2. Compute natural total width
3. If too wide → shrink glue proportionally (up to shrink limit)
4. If too narrow → stretch glue proportionally (up to stretch limit)

**Pros:**
- Mathematically elegant — only two primitives (box, glue)
- Deterministic, linear-time
- Handles justification, centering, and spacing all through glue
- Battle-tested for 45+ years — proof that good foundations endure
- Knuth-Plass paragraph-optimal line-breaking is still unmatched

**Cons:**
- Fundamentally 1D; 2D is achieved only through nesting
- No native 2D grid concept
- No constraint relationships between distant elements
- Macro language is notoriously difficult to use and extend
- Hard to express responsive/adaptive layouts

**Key lesson:** The glue abstraction (natural + stretch + shrink) is the single best idea in layout history. Any future system should incorporate it.

---

### 3.2 CSS — The Largest Real-World Layout System

CSS is the most broadly deployed layout system in history, and the only one that must handle documents, applications, responsive design, internationalization, and accessibility simultaneously.

**What CSS supports today:**
- **Normal flow** — block + inline formatting contexts, floats (legacy)
- **Flexbox** — 1D flexible boxes with grow/shrink/basis
- **Grid** — true 2D grid with `fr` units, named areas, auto-placement, subgrid
- **Multi-column layout** — newspaper-style columns
- **Positioning** — relative, absolute, fixed, sticky
- **Container queries** — component-level responsiveness
- **Logical properties** — `inline-start`/`block-end` instead of `left`/`top`
- **Intrinsic sizing** — `min-content`, `max-content`, `fit-content`
- **Aspect ratio, containment, scroll snap, anchor positioning**

#### CSS Flexbox

1D layout along a main axis. Children have `flex-grow`, `flex-shrink`, and `flex-basis`.

```css
.container { display: flex; flex-direction: row; gap: 16px; }
.child     { flex: 1 0 200px; }  /* grow shrink basis */
```

**Algorithm (simplified):**
1. Compute hypothetical sizes from basis
2. Determine free space (positive → grow, negative → shrink)
3. Distribute according to grow/shrink factors
4. Handle min/max constraints; freeze violated items; redistribute

**Pros:** Intuitive 1D model, alignment is first-class, good wrapping support.
**Cons:** Only 1D, multi-pass with freezing loops (can be O(n²)), `flex-shrink` + `min-width: auto` interactions are surprising, specification is ~80 pages.

#### CSS Grid

True 2D grid with explicit row/column track definitions.

```css
.grid {
  display: grid;
  grid-template-columns: 200px 1fr 2fr;
  grid-template-rows: auto 1fr;
  gap: 16px;
}
```

**Track sizing functions:** fixed (`200px`), flexible (`1fr`), content-based (`auto`, `min-content`, `max-content`), range (`minmax(200px, 1fr)`), fit (`fit-content(300px)`).

**Pros:** True 2D, named grid areas are semantically brilliant, `fr` unit is elegant, `minmax()` handles responsive sizing, subgrid enables nested alignment.
**Cons:** Complex specification (~200 pages), track sizing algorithm has 12+ steps, spanning items + intrinsic sizing = intricate interactions, masonry still under proposal.

#### CSS Strengths
- **Breadth:** No other system covers documents, apps, responsiveness, print, i18n simultaneously
- **Flexbox + Grid together** are excellent and cover most modern needs
- **Container queries** are one of the most important recent advances — responsive behavior depends on the component's container, not the viewport. Essential for reusable components.
- **Subgrid** solves the critical encapsulation-vs-alignment tension: nested components can align to parent grid tracks
- **Huge ecosystem** of browsers, tools, frameworks, education, devtools

#### CSS Weaknesses
- **Legacy complexity:** Floats, margin collapsing, block formatting context quirks, specificity wars
- **Global cascade** is brilliant for theming but bad for component-scoped reasoning
- **Not component-native:** Tension between global styling and encapsulation
- **Paged layout** is not the web's strength — browsers are scroll-first

#### What to learn from CSS
**Keep:** Grid, Flex concepts, logical properties, intrinsic sizing, container queries, subgrid, alignment model, aspect ratio, containment.
**Avoid:** Floats, margin collapsing, overly global cascade as the only styling model, too many overlapping legacy modes.

---

### 3.3 Typst — Modern Document Typesetting

Typst is one of the most interesting modern systems: a clean, fast (Rust-based) alternative to LaTeX with integrated scripting.

```typst
#grid(
  columns: (1fr, 2fr),
  rows: (auto, 1fr),
  gutter: 12pt,
  [Cell 1], [Cell 2],
  [Cell 3], [Cell 4],
)
```

**Core concepts:**
- **Region:** Available space passed down (can be multiple regions for page breaks)
- **Frame:** Output of layout — positioned content with a size
- **Layout function:** Takes regions, returns frames

**Sizing killer feature:** Arithmetic on sizes: `100% - 24pt` just works. No `calc()` wrapper needed.

**Pros:**
- Exceptionally clean, readable syntax
- Blazing fast incremental compilation
- Strong math and typography out of the box
- Regions handle pagination elegantly (content flowing across pages)
- Show/set rules allow style customization without layout complexity
- Programmable — layouts as code with a functional model
- Deterministic, reproducible outputs

**Cons:**
- Document-oriented, not for interactive app UIs
- Younger ecosystem than TeX/LaTeX
- Grid is less powerful than CSS Grid (no named areas, limited auto-placement)
- No animation or interaction model
- No constraint system between non-adjacent elements

**Key lesson:** Size arithmetic, clean composability, and the region/frame model for pagination are ideas worth stealing. A future system should combine Typst's cleanliness with app-UI capability.

---

### 3.4 TeX / LaTeX

TeX proved what a durable typesetting system looks like. Its boxes-and-glue model and Knuth-Plass line-breaking algorithm are mathematically rigorous and have lasted 45+ years. The macro language is horrifying, but the core geometry is timeless.

**Key lesson:** Good text layout and pagination matter enormously. A future system should learn from TeX's *quality* and *mathematical foundations*, but not from its macro complexity.

---

### 3.5 SwiftUI Layout System

Three-step negotiation protocol:

```swift
protocol Layout {
    func sizeThatFits(
        proposal: ProposedViewSize,  // parent's offer
        subviews: Subviews,
        cache: inout Cache
    ) -> CGSize                       // container's chosen size

    func placeSubviews(
        in bounds: CGRect,           // allocated rectangle
        proposal: ProposedViewSize,
        subviews: Subviews,
        cache: inout Cache
    )
}
```

**The protocol:**
1. **Parent proposes** size to child (nil = "ideal", 0 = "minimum", ∞ = "maximum")
2. **Child chooses** its own size (child sovereignty — parent cannot override)
3. **Parent places** child at a position

**Built-in:** `HStack`, `VStack`, `ZStack`, `Grid`, custom `Layout` protocol implementations.

**Pros:** Clean separation of sizing and placement, custom layouts are first-class, caching prevents redundant calculations, every view is a layout participant.
**Cons:** Child sovereignty can cause surprises, `GeometryReader` breaks the model, stack distribution algorithm is undocumented internally, limited to rectangles, `layoutPriority` is coarse.

**Performance:** O(n) per container.

---

### 3.6 Jetpack Compose

Single-pass measure with intrinsic size queries.

```kotlin
Layout(
    content = { /* children */ },
    measurePolicy = { measurables, constraints ->
        val placeables = measurables.map { it.measure(constraints) }
        layout(width, height) {
            placeables.forEach { it.place(x, y) }
        }
    }
)
```

**Critical rule:** Each child can only be measured **ONCE** (enforced at runtime). Intrinsic queries (`minIntrinsicWidth`, `maxIntrinsicHeight`) serve as "dry run" queries that don't count.

**Pros:** Single-measure rule *prevents O(n²) layouts by design* — the strongest performance guarantee of any system. Constraints are explicit and clean (4 values: minWidth, maxWidth, minHeight, maxHeight). Custom layouts are easy and encouraged.
**Cons:** Single-measure rule can be too restrictive (needs `SubcomposeLayout` workaround). Modifier order matters and confuses beginners.

---

### 3.7 WPF/XAML

Two-pass measure + arrange. Pioneered star sizing (`*`, `2*`) — fractional units before CSS had `fr`.

**Built-in panels:** StackPanel, Grid, Canvas, DockPanel, WrapPanel, UniformGrid.
**Attached properties** (`Grid.Row="1"`) elegantly set per-child layout parameters.

**Key lesson:** The convergence of multiple retained-mode UI systems (WPF, Qt, Swing) onto a small set of panel/container primitives (stack, grid, canvas, dock, wrap) is strong evidence that **a small set of composable containers is sufficient**.

---

### 3.8 Flutter

**Protocol:** "Constraints go down, sizes go up, parent sets position."

One of the cleanest modern layout mental models. `Row`/`Column` (flex), `Stack` (z-layering), `Wrap` (flow), `Table` (grid). Slivers for lazy/virtualized scrolling layouts.

**Pros:** Clear parent-child contract, predictable, efficient, composable.
**Cons:** Widget nesting hell, `IntrinsicWidth`/`IntrinsicHeight` can be O(2^n) in theory, no first-class 2D grid, too many wrapper widgets (`ConstrainedBox`, `SizedBox`, `FractionallySizedBox`).

---

### 3.9 Apple Auto Layout (Constraint-Based)

Linear constraint system solved by the Cassowary algorithm.

```swift
view.widthAnchor.constraint(equalTo: otherView.widthAnchor, multiplier: 0.5)
view.leadingAnchor.constraint(equalTo: parent.leadingAnchor, constant: 16)
```

**Pros:** Maximum expressiveness — any linear relationship between any elements, not just parent-child. Priority system handles soft constraints.
**Cons:** **O(n³) worst case** for the simplex algorithm, ambiguous/unsatisfiable layouts produce unpredictable or cryptic results, debugging is notoriously difficult, constraints are global (not scoped to a subtree), not composable, high conceptual overhead.

**Verdict for the future:** Constraint solvers are too complex, too hard to debug, and too performance-risky to serve as the universal default. They should exist only as a *limited optional tool*, not the foundation.

---

### 3.10 Qt QML

Three coexisting systems: **Anchors** (relative positioning), **Positioners** (Row/Column/Grid/Flow — positioning only), **Layouts** (RowLayout/GridLayout — full resize + fill).

**Key lesson:** Three overlapping systems create confusion about which to use. A future system should offer *one* clear path, not multiple half-solutions.

---

### 3.11 Yoga / Taffy — Portable Layout Engines

**Yoga** (Facebook, C): Cross-platform Flexbox used by React Native, Litho, ComponentKit. Deterministic across platforms but limited to Flexbox only.

**Taffy** (Rust): More modern, adding Block + Flex + Grid. Used in Bevy, Dioxus. Represents the critical trend toward **layout-as-a-portable-library** decoupled from any renderer.

**Key lesson:** The future is probably a portable layout engine core (like Taffy) that can target web, native, PDF, VR, or console output from a single declarative description.

---

### 3.12 Figma Auto Layout

Not a runtime layout system, but extremely influential. Makes padding, gap, alignment, and stacking utterly intuitive for designers. Popularized stack-based component thinking.

**Key lesson:** Good layout systems should feel as approachable as Figma Auto Layout. If designers can't understand the model, it's too complex.

---

### 3.13 Immediate-Mode GUIs (Dear ImGui, egui)

**Pros:** Fast to build tools, simple programming model, great for debug/editor interfaces.
**Cons:** Weak semantics/accessibility, not ideal for long-lived standards, less natural for rich responsive layout.

**Verdict:** Useful for tools, not the basis for a universal future layout spec.

---

### 3.14 Paged CSS Engines (PrinceXML, Vivliostyle, Paged.js)

These use CSS for print/book-like output, bridging web skills and publishing workflows. Browser-native support is uneven, and the web is still more scroll-first than page-first.

**Key lesson:** A future system should ideally support **both scroll and paged output from the same structured source**. This is a requirement most UI-centric proposals miss entirely.

---

## 4. Emerging Solutions

### CSS Container Queries
One of the most important recent advances. Responsive behavior depends on the **component's container**, not the viewport. Essential for truly reusable components.

### CSS Subgrid
Allows nested components to align to parent grid tracks. Solves the critical tension between component encapsulation and cross-component alignment.

### CSS Anchor Positioning
Elements explicitly reference other elements' positions with fallback chains for overflow. Critical for tooltips, popovers, badges, dropdowns.

### CSS Masonry (Proposed)
Useful for card walls and media grids, but should be an **extension**, not a core primitive — it can harm reading order and complicate predictability.

### CSS Houdini Layout API
Custom layout algorithms in JavaScript. Low adoption but conceptually important: it proves the need for extensible layout primitives.

### Taffy, Morphorm — Standalone Layout Engines
Layout as a renderer-independent, platform-independent, portable library. This is the likely architecture of the future.

### Incremental Layout
Only re-layout what changed. Typst does this well. Key challenge: correctly invalidating dependents without over-invalidating.

### AI-Assisted Layout
AI may help *generate* or *suggest* layouts, but it is **not** a layout model. The execution model still needs deterministic, testable primitives.

---

## 5. Comparative Matrix

```
┌──────────────────┬────────┬──────┬───────┬────────┬─────────┬──────────┬───────────┐
│ System           │Dim.    │Perf. │Simple │Express.│Composab.│Determin. │Custom Ext.│
├──────────────────┼────────┼──────┼───────┼────────┼─────────┼──────────┼───────────┤
│TeX Boxes+Glue    │  1D*   │ O(n) │ ★★★★★ │ ★★★☆☆  │ ★★★★☆   │ ★★★★★    │ ★★☆☆☆    │
│CSS Flexbox       │  1D    │~O(n) │ ★★★★☆ │ ★★★★☆  │ ★★★★☆   │ ★★★★☆    │ ☆☆☆☆☆    │
│CSS Grid          │  2D    │O(n·m)│ ★★★☆☆ │ ★★★★★  │ ★★★☆☆   │ ★★★★☆    │ ☆☆☆☆☆    │
│SwiftUI           │  1D*   │ O(n) │ ★★★★☆ │ ★★★★☆  │ ★★★★★   │ ★★★★★    │ ★★★★★    │
│Compose           │  1D*   │ O(n) │ ★★★★☆ │ ★★★★☆  │ ★★★★★   │ ★★★★★    │ ★★★★★    │
│Auto Layout       │  2D    │ O(n³)│ ★★☆☆☆ │ ★★★★★  │ ★★☆☆☆   │ ★★★☆☆    │ ★★★☆☆    │
│WPF/XAML          │  1D*   │ O(n) │ ★★★☆☆ │ ★★★★☆  │ ★★★★☆   │ ★★★★★    │ ★★★★☆    │
│Flutter           │  1D*   │ O(n) │ ★★★☆☆ │ ★★★★☆  │ ★★★★★   │ ★★★★☆    │ ★★★★☆    │
│Typst             │  1D*   │ O(n) │ ★★★★★ │ ★★★☆☆  │ ★★★★☆   │ ★★★★★    │ ★★★☆☆    │
│Qt QML            │  1.5D  │ O(n) │ ★★★☆☆ │ ★★★★☆  │ ★★★☆☆   │ ★★★★☆    │ ★★★☆☆    │
│Yoga              │  1D    │ O(n) │ ★★★★☆ │ ★★★☆☆  │ ★★★★☆   │ ★★★★★    │ ☆☆☆☆☆    │
│Figma AutoLayout  │  1D    │ O(n) │ ★★★★★ │ ★★☆☆☆  │ ★★★★☆   │ ★★★★★    │ ☆☆☆☆☆    │
└──────────────────┴────────┴──────┴───────┴────────┴─────────┴──────────┴───────────┘
* = 1D composed to 2D via nesting
```

### Size Negotiation Protocols Compared

| System | Protocol |
|--------|----------|
| TeX | Parent computes total → distributes via glue |
| CSS | Complex multi-pass (depends on layout mode) |
| SwiftUI | Parent proposes → Child chooses → Parent places |
| Compose | Parent constrains → Child measures (ONCE) → Parent places |
| WPF | Child measures(available) → Parent arranges(final) |
| Flutter | Parent constrains → Child layouts → Parent positions |
| Auto Layout | Global constraint satisfaction |
| Typst | Regions flow down → Frames flow up |

---

## 6. Key Design Tensions & Tradeoffs

### Tension 1: Expressiveness vs. Performance

Auto Layout achieves maximum expressiveness at O(n³) cost. SwiftUI and Compose achieve near-maximal expressiveness at O(n). The Pareto frontier lies at **bounded negotiation with extensible custom layouts** — not global constraint solving.

### Tension 2: Parent Control vs. Child Autonomy

| Approach | Who decides? | Example |
|----------|-------------|---------|
| Parent dictates | Parent | Canvas / absolute positioning |
| Child dictates | Child | SwiftUI (child sovereignty) |
| Negotiation with bounds | Both, clamped | Compose (constraints → choice) |
| Constraint solver | Neither directly | Auto Layout |

**Position taken:** Negotiation with clamping is the right default. Parent provides a min/max constraint range; child chooses within that range; parent places the result. Children **must** respect constraints (clamped) to guarantee determinism and prevent layout "escape" — unlike SwiftUI's child sovereignty model, which creates surprises when children ignore parent intent. If overflow occurs, the parent handles it via explicit overflow policy (clip, scroll, etc.).

### Tension 3: 1D Composition vs. Native 2D

**1D Composed (Stack-based):** Simpler mental model, each container is independent, O(n). But can't align items across rows; nesting changes negotiation dynamics.

**Native 2D (Grid-based):** Items in different rows/columns can relate, spanning is possible. But more complex algorithm, harder to make fully composable.

**Position taken:** Both are needed. A stack for simple linear layouts, a grid for when 2D relationships matter. Not everything is a grid; not everything is a stack.

### Tension 4: Implicit vs. Explicit Placement

Implicit (flow/auto-placement): less specification, less control. Explicit (coordinates/names): full control, more verbose. Both have their place; the system should support both with clear semantics.

### Tension 5: Intrinsic vs. Extrinsic Sizing

The real world needs both simultaneously. A text label has intrinsic width but might need to wrap within an extrinsic container width. The sizing protocol must support this negotiation smoothly.

### Tension 6: Simplicity vs. Completeness

Systems that try to handle everything (CSS) become enormously complex. Systems that stay simple (TeX boxes) can't express certain layouts. The answer is finding the **minimal complete set** of orthogonal primitives, with a well-defined escape hatch for custom algorithms.

---

## 7. Brainstorm: Essential Properties for Future Layout

### Must-Have Properties

| Property | Why | Best Existing Example |
|----------|-----|----------------------|
| **Composability** | Components must nest infinitely | SwiftUI, Compose |
| **Determinism** | Same input → same output, always | TeX, Compose |
| **O(n) performance** | Layout must scale linearly | Compose's single-measure rule |
| **Bidirectional negotiation** | Parent and child both influence size | SwiftUI propose→choose→place |
| **Intrinsic sizing** | Content can determine size | All modern systems |
| **Fractional distribution** | Remaining space distributed proportionally | CSS `fr`, TeX glue |
| **Min/max constraints** | Bounds on negotiation | Compose Constraints |
| **Custom extensibility** | New layout algorithms as first-class | SwiftUI Layout protocol |
| **Logical directions** | Start/end/block/inline, not left/right | CSS logical properties |
| **Container-based responsiveness** | Adapt to container, not viewport | CSS container queries |
| **Strong text handling** | Unicode shaping, bidi, hyphenation, wrapping, baselines, vertical writing | TeX, Typst, CSS |
| **Pagination/fragmentation** | Unified scroll and paged output | Typst regions |
| **Accessibility/semantic structure** | Source order = reading order, roles preserved | HTML semantics model |

### Should-Have Properties

| Property | Why | Best Existing Example |
|----------|-----|----------------------|
| **Size arithmetic** | `100% - 24px` type expressions | Typst, CSS `calc()` |
| **Flexible spacing** | Spacing that stretches/shrinks with priority | TeX glue |
| **Overflow control** | Per-element, per-axis overflow strategy | CSS overflow |
| **Incremental relayout** | Only recompute what changed | Typst |
| **Virtualization** | Large lists render only visible items | Flutter Slivers |
| **Subgrid participation** | Nested components align to parent tracks | CSS subgrid |
| **Component-exposed metrics** | Intrinsic size, baselines, named anchors | SwiftUI alignment guides |
| **Deterministic rounding** | Spec-defined rounding and unit behavior | Critical for cross-impl consistency |

### Nice-to-Have Properties

| Property | Why | Best Existing Example |
|----------|-----|----------------------|
| **Anchor positioning** | Reference other elements | CSS anchor positioning |
| **Animation integration** | Layout properties interpolate smoothly | SwiftUI |
| **Baseline alignment** | Text baselines across elements | CSS, SwiftUI |
| **Multiple output targets** | Same tree → screen, print, VR, audio | Typst, paged CSS |

### Dangerous Properties to Ban from Core

| Property | Why it's dangerous |
|----------|-------------------|
| Floats | Layout dead end, creates clearing hacks |
| Margin collapsing | Unpredictable, context-dependent |
| Unrestricted global cascade | Specificity wars, non-local reasoning |
| Unrestricted constraint solving | O(n³), cycles, unsatisfiable states |
| Layout cycles | Infinite loops if parent and child depend on each other |
| Visual reorder without semantic reorder | Breaks accessibility |

### Underrated Properties

- **Formal algorithm descriptions** — ambiguity kills cross-implementation consistency and longevity
- **Deterministic rounding** — fractional pixels handled identically across implementations
- **Fixed unit semantics** — precise definitions of pt, em, rem, px
- **Multiple output targets from one layout tree** — the same source rendering to screen, print, VR, or audio description
- **Debuggability** — visual overlays, constraint visualization, measurement introspection (addressed as an open consideration below)

---

## 8. Proposed System: Frame — A Layout System for Centuries

### Design Philosophy

> *The minimum set of orthogonal primitives that can express any rectangular 2D layout, with O(n) performance guaranteed, fully composable, and extensible for unforeseen needs. Built on axioms, not rules.*

**Principles:**
1. Everything is a **Frame** — the unified, composable component
2. **Five layout modes** — flow, stack, grid, layer, region — covering text, 1D, 2D, overlay, and pagination
3. **One sizing protocol** — constrain → size → place
4. **Flexible spacing** — TeX-derived glue (natural, stretch, shrink) as a unified spacing model
5. **Arithmetic sizes** — sizes are expressions, not just values
6. **Platform and renderer independent** — describes intent and constraints, not pixels

---

### 8.1 Core Data Types

```
╔═══════════════════════════════════════════════════════════╗
║  FRAME LAYOUT SYSTEM — CORE TYPES                         ║
╚═══════════════════════════════════════════════════════════╝

┌──────────────────────────────────────────┐
│ Size                                     │
├──────────────────────────────────────────┤
│ fixed(value)        12pt, 3em            │  ← absolute
│ fraction(f)         1fr, 2fr             │  ← share of remaining space
│ fit                                      │  ← shrink to content (intrinsic)
│ fill                                     │  ← expand to available (= 1fr)
│ range(min, max)     range(100pt, 1fr)    │  ← constrained
│ expr(fn)            fill - 24pt          │  ← computed expression
╘══════════════════════════════════════════╛

┌──────────────────────────────────────────┐
│ Constraints                              │
├──────────────────────────────────────────┤
│ min_width:   number | 0                  │
│ max_width:   number | ∞                  │
│ min_height:  number | 0                  │
│ max_height:  number | ∞                  │
╘══════════════════════════════════════════╛

┌──────────────────────────────────────────┐
│ Space (TeX glue — the unified spacing)   │
├──────────────────────────────────────────┤
│ natural:  number     preferred size      │
│ stretch:  number     can grow by         │
│ shrink:   number     can shrink by       │
│                                          │
│ Constructors:                            │
│   rigid(16)      = {16, 0, 0}            │
│   flex(8, 16, 32)= {16, 16, 8}          │
│   spring         = {0, ∞, 0}            │
│   soft(n)        = {n, n, n}            │
╘══════════════════════════════════════════╛

┌──────────────────────────────────────────┐
│ Align                                    │
├──────────────────────────────────────────┤
│ start | center | end | stretch           │
│ baseline (block axis only)               │
│                                          │
│ Applied as:                              │
│   self_align:    own position in parent  │
│   content_align: children within self    │
╘══════════════════════════════════════════╛

┌──────────────────────────────────────────┐
│ Overflow                                 │
├──────────────────────────────────────────┤
│ clip | scroll | visible | wrap           │
│ Applied per axis                         │
╘══════════════════════════════════════════╛
```

**Size expressions** can mix units and use functions:

```
width: 50%                          // relative to parent content area
width: fill - 24pt                  // remaining space minus 24pt
width: min(300pt, fill)             // responsive capping
width: clamp(200pt, 50%, 600pt)     // responsive range
width: range(200pt, 1fr)            // min 200pt, else fractional
```

---

### 8.2 The Frame — Universal Component

```
Frame {
  ── Sizing ──
  width:         Size = fit
  height:        Size = fit
  aspect_ratio:  optional number

  ── Spacing ──
  padding:       Edges<Space> = 0
  margin:        Edges<Space> = 0

  ── Layout ──
  layout:        Layout = stack(block)      // see 8.3

  ── Alignment ──
  self_align:    Axes<Align> = (start, start)
  content_align: Axes<Align> = (start, start)

  ── Overflow ──
  overflow:      Axes<Overflow> = (clip, clip)

  ── Content ──
  children:      [Frame]
  content:       optional Content           // text, image, vector, etc.

  ── Exposed Metrics ──
  baselines:     [named baseline positions]  // for cross-component alignment
  anchors:       [named anchor points]       // for anchor positioning
}
```

**Position on margin and padding:** Both exist as first-class concepts using the flexible `Space` type. Margin collapsing is **banned** — margins always add. This preserves the ergonomic convenience of padding/margin (which every framework has converged on) while eliminating the unpredictable behavior that CSS margin collapsing creates. The flexible `Space` type (natural/stretch/shrink) subsumes CSS `gap`, `justify-content` modes, `margin: auto`, and TeX's `\hfill` into a single unified primitive.

---

### 8.3 The Five Layout Modes

#### Why five, not three or six?

After analyzing all systems, the evidence shows:

- **Flow** for text is irreducible — text wrapping, line breaking, and inline layout are fundamentally different from box stacking and cannot be adequately modeled as a property of a stack with wrapping. Every system that tries (e.g., treating text as just another flex-wrapping container) produces inferior typography and loses critical features like hyphenation, justification, widow/orphan control, and bidi.
- **Stack** for 1D box arrangement is the dominant layout primitive in every modern system.
- **Grid** for 2D is essential and cannot be replaced by nested stacks (cross-axis alignment is lost).
- **Layer** for overlapping/z-stacking is needed for badges, modals, backgrounds — distinct from other modes.
- **Region** for pagination/fragmentation is essential for century-scale durability. Most UI-centric proposals miss this entirely, but any system claiming longevity must handle the transition from scroll to pages/columns/spreads — otherwise it cannot serve documents, reports, books, or print. Typst's region model proves this can be clean.

A `Spacer` (as some frameworks use) is not a separate primitive — it's simply a Frame with `width: spring` or `height: spring` using the flexible Space type.

---

#### 1. Flow — Text and Inline Content

```
layout: flow(direction, wrap, justify, line_spacing)

direction:     inline | block        // logical direction
wrap:          true (default)
justify:       start | center | end | justify | justify-all
line_spacing:  Space
```

Handles paragraphs, inline elements, text wrapping, hyphenation, justification, baseline alignment, bidi/RTL, vertical writing modes.

**Typography requirements (non-negotiable for longevity):**
- Unicode text shaping (HarfBuzz-level)
- Bidirectional text (UAX #9)
- Line breaking (UAX #14) and hyphenation
- Justification with quality metrics
- Widow/orphan control
- Baseline alignment groups
- Vertical writing modes
- Footnotes/endnotes (when used with Region)

```
┌──────────────────────────────────────┐
│  flow(justify: justify)              │
│  The quick brown fox jumps over the  │
│  lazy dog. Each line is justified    │
│  with proper hyphenation and base-   │
│  line alignment.                     │
└──────────────────────────────────────┘
```

---

#### 2. Stack — 1D Sequential

```
layout: stack(axis, gap, distribute)

axis:        inline | block
gap:         Space = rigid(0)          // flexible gaps!
distribute:  start | center | end | between | around | evenly
```

Children placed sequentially along the axis. Cross-axis behavior controlled by `content_align` or each child's `self_align`.

```
┌──────────────────────────────────────┐
│  stack(inline, gap: rigid(8))        │
│  ┌─────┐  ┌──────────┐  ┌──────┐   │
│  │  A  │  │    B     │  │  C   │   │
│  └─────┘  └──────────┘  └──────┘   │
│  ◄─8pt─►  ◄───8pt───►              │
└──────────────────────────────────────┘
```

**Sizing algorithm (linear time):**
1. Compute available space on main axis
2. Subtract fixed-size children and rigid gaps
3. Subtract natural size of flexible gaps
4. Distribute remaining space:
   - To `fraction`-sized children (proportional to fr value)
   - If still positive: stretch flexible gaps (proportional to stretch)
   - If negative: shrink flexible gaps (proportional to shrink)
   - If still negative: shrink `fit`-sized children proportionally
5. Cross-axis: each child gets full cross-axis available space, constrained by alignment

**Flexible spacing unification — how Space replaces many CSS concepts:**

| CSS Concept | Frame Equivalent |
|-------------|-----------------|
| `gap: 16px` | `gap: rigid(16)` |
| `justify-content: space-between` | `gap: spring` (between children) |
| `justify-content: center` | children with `spring` margins on both sides |
| `margin: auto` | `margin: spring` on the auto axis |
| TeX `\hfill` | `Space(0, ∞, 0)` = `spring` |

---

#### 3. Grid — 2D Sequential

```
layout: grid(columns, rows, gap_inline, gap_block)

columns:     [Size]                    // track definitions
rows:        [Size]                    // auto-generated if omitted
gap_inline:  Space = rigid(0)
gap_block:   Space = rigid(0)
areas:       optional named area map   // semantic grid areas
```

Children placed left-to-right, top-to-bottom (or per writing mode). Children can specify `span_inline`, `span_block`.

```
┌──────────────────────────────────────────┐
│  grid(columns: [1fr, 2fr],              │
│       rows: [fit, 1fr])                  │
│  ┌──────────┬─────────────────────────┐  │
│  │  A (fit) │  B (fit)                │  │
│  ├──────────┼─────────────────────────┤  │
│  │  C (1fr) │  D (1fr)                │  │
│  │          │                         │  │
│  └──────────┴─────────────────────────┘  │
└──────────────────────────────────────────┘
```

**Track sizing algorithm (simplified):**
1. Resolve fixed tracks
2. Resolve `fit` tracks: measure items, use max intrinsic size across items in each track
3. Compute remaining space after fixed + fit
4. Distribute remaining to `fraction` tracks proportionally
5. Handle spanning items (distributed across spanned tracks)
6. Place items in resolved cells

**Subgrid support:** A grid child can declare `subgrid` on one or both axes, inheriting the parent's track definitions. This solves the encapsulation-vs-alignment tension: components remain encapsulated but can participate in parent alignment when explicitly opted in.

---

#### 4. Layer — Overlapping (Z-Stack)

```
layout: layer(align)

align:  Axes<Align> = (center, center)
```

Children stacked on top of each other. Each child can set `self_align` to position within the layer. Drawing order = declaration order (last = on top).

```
┌──────────────────────────────────┐
│  layer                           │
│  ┌────────────────────────────┐  │
│  │  Background (fill × fill)  │  │
│  │    ┌──────────────┐        │  │
│  │    │ Content (fit) │        │  │
│  │    └──────────────┘        │  │
│  └────────────────────────────┘  │
│           ┌─────┐                │
│           │Badge│ ← self_align: (end, start)
│           └─────┘                │
└──────────────────────────────────┘
```

Supports **anchor positioning** as a limited relation: a child can reference a sibling's named anchor to position itself relative to that sibling, with fallback strategies for overflow. This replaces full constraint systems for the most common relational use cases (tooltips, popovers, badges).

---

#### 5. Region — Output Fragmentation

```
layout: region(mode, columns)

mode:     scroll | page(size, margins) | columns(count, gap)
columns:  optional column specification
```

A Region is a fragmentation container that determines how content is output. This unifies app and document output from the same source tree.

```
┌─────────────────────────────────────────┐
│  region(mode: page(A4, margin: 20mm))   │
│  ┌─────────────┐  ┌─────────────┐      │
│  │   Page 1     │  │   Page 2     │     │
│  │  ┌────────┐  │  │  ┌────────┐  │     │
│  │  │content │  │  │  │content │  │     │
│  │  │ flows  │  │  │  │ cont.  │  │     │
│  │  └────────┘  │  │  └────────┘  │     │
│  └─────────────┘  └─────────────┘      │
└─────────────────────────────────────────┘
```

**Why this is essential for longevity:** Any system claiming century-scale durability that can only scroll will be obsolete the moment someone needs a printed report, a PDF, or an e-book. Typst proved that regions/frames can be clean and efficient. A Region is simply a container that decides how to fragment its children's overflow: scrolling (interactive), pagination (print/PDF), or multi-column (newspaper-style). The children don't need to know.

**Fragmentation properties on children:**
- `break_before: auto | always | avoid`
- `break_after: auto | always | avoid`
- `break_inside: auto | avoid`
- `orphans: number`, `widows: number`

---

### 8.4 The Sizing Protocol

Inspired by Compose (single-measure guarantee) and SwiftUI (clear negotiation), unified:

```
╔═════════════════════════════════════════════════════════════╗
║  THE SIZING PROTOCOL                                        ║
║                                                             ║
║  1. CONSTRAIN                                               ║
║     Parent passes Constraints to child:                     ║
║     { min_width, max_width, min_height, max_height }        ║
║                                                             ║
║  2. SIZE                                                    ║
║     Child computes its size within constraints               ║
║     Returns chosen (width, height)                          ║
║     ⚠ Child MUST respect constraints (clamped to range)     ║
║                                                             ║
║  3. PLACE                                                   ║
║     Parent assigns position (x, y) to child                 ║
║     Position is relative to parent's content area           ║
║                                                             ║
║  INTRINSIC QUERIES (for content-aware negotiation):         ║
║     min_size(axis, cross_constraint) → number               ║
║     max_size(axis, cross_constraint) → number               ║
║     These are "dry runs" — don't count as the real measure  ║
║                                                             ║
║  EXPOSED METRICS (optional, for cross-component alignment): ║
║     baselines:  [named baseline positions]                  ║
║     anchors:    [named anchor points]                       ║
╚═════════════════════════════════════════════════════════════╝
```

**Key rules:**
1. Children are clamped to constraints — no sovereignty exceptions
2. Each child measured at most **twice** (once for intrinsics, once for real)
3. Result is deterministic: same tree + same root constraints = identical output
4. Components expose baselines and named anchors through their boundary for cross-component alignment, solving the encapsulation-vs-alignment tension

**Performance guarantee:** O(n) total. Each frame measured once (at most twice with intrinsics). No recursion beyond tree depth. No constraint solver. No multi-pass redistribution.

**Pseudocode for the protocol:**

```
fn measure(frame, constraints) → LayoutResult:
    // Phase 1: Intrinsic query (if layout mode needs it)
    for child in frame.children:
        if layout needs intrinsic info:
            child.min_size(axis, cross) → cached
            child.max_size(axis, cross) → cached

    // Phase 2: Resolve track/slot sizes
    resolved = frame.layout.resolve(constraints, children_intrinsics)

    // Phase 3: Measure each child with resolved constraints
    for child in frame.children:
        child_constraints = resolved.constraints_for(child)
        child.result = measure(child, child_constraints)  // exactly once

    // Phase 4: Place children
    placements = frame.layout.place(children_results)

    // Phase 5: Compute own size, clamped to constraints
    own_size = clamp(
        frame.layout.compute_size(children_results),
        constraints.min, constraints.max
    )

    return { own_size, placements, baselines, anchors }
```

---

### 8.5 Component Model

A component is a parameterized Frame template:

```
component Card(title, body, actions?) {
  layout: stack(block, gap: rigid(12))
  padding: rigid(16)

  Frame(layout: flow) { content: title }
  Frame(layout: flow) { content: body }

  if actions {
    Frame(layout: stack(inline, gap: rigid(8), distribute: end)) {
      ...actions
    }
  }
}
```

**Components expose, when needed:**
- Intrinsic size (via the protocol)
- Named baselines (for cross-component text alignment)
- Named anchors (for anchor positioning)
- Optional subgrid participation (inheriting parent tracks)

**Styling model:** Styles are **scoped by component** by default. Global theming uses **design tokens** (spacing scale, colors, type scale). Text-related styles may inherit; layout styles do not. No complex specificity system — explicit override order only. This preserves theming without losing locality, avoiding CSS cascade wars.

```
tokens {
  spacing.small:  4pt
  spacing.medium: 8pt
  spacing.large:  16pt
  type.body:      { family: "...", size: 11pt, line_height: 1.5 }
}
```

---

### 8.6 Responsive Design

No media queries as the primary mechanism. Sizes are responsive by construction via size expressions:

```
width: clamp(300pt, 50%, fill)               // responsive range
grid(columns: [range(250pt, 1fr)])           // auto-fill: as many 250pt+ columns as fit
width: min(400pt, fill)                      // capped width
```

For layout mode changes, use container-responsive matching (evaluated against the component's own available space, not the viewport):

```
Frame.responsive(
  compact:  stack(block),           // < 600pt available
  regular:  stack(inline),          // < 1200pt
  expanded: grid(columns: [...])   // ≥ 1200pt
)
```

This makes components truly self-contained and reusable in any context.

---

### 8.7 Scrolling and Virtualization

Scrolling is modeled as overflow behavior, not a separate concept:

```
Frame(width: 300pt, height: 400pt, overflow_block: scroll) {
  Frame(layout: stack(block), height: fit) {
    // Content taller than 400pt → scrolls vertically
  }
}
```

Scrollable content gets `max_height: ∞` in its constraints. It sizes to fit content. The parent clips and translates.

**Virtualization** for large lists:

```
Frame(layout: stack(block), overflow_block: scroll, virtual: true) {
  source: items.map(item => Frame { ... })
  // Only visible children are measured and drawn
  // Others use estimated sizes from cache or heuristic
}
```

---

### 8.8 Custom Layout Protocol

For layouts beyond the five built-in modes:

```
trait CustomLayout {
    fn intrinsic_size(
        axis: Axis,
        children: [ChildProxy],
        cross_constraint: number
    ) → number

    fn layout(
        constraints: Constraints,
        children: [ChildProxy]
    ) → LayoutResult {
        // For each child:
        //   child.measure(child_constraints) → child_size
        //   ⚠ May call child.measure() AT MOST ONCE
        //
        // Return: own_size + [(child, position)] pairs
    }
}
```

**Examples of custom layouts:**
- `circular(radius, start_angle)` — children arranged in a circle
- `masonry(columns, gap)` — variable-height items packed into columns
- `dock(edges)` — WPF-style dock layout
- `treemap(data)` — area-proportional rectangles

Custom layouts reduce to the same protocol and maintain O(n) guarantees via the single-measure rule.

---

### 8.9 Accessibility and Semantics

A long-lived layout system must keep source structure meaningful:

- **Logical reading order is source order by default.** Visual reordering must be explicit and carries an accessibility warning.
- **Semantic roles** (heading, list, table, figure, landmark, navigation) are separate from layout but travel with Frames.
- **Layout and semantics are orthogonal layers** — you can change layout without losing semantic meaning.
- **Reduced motion, high contrast, and user preferences** are respected via token-level adaptation.

---

### 8.10 Complete Example

A responsive application shell with content:

```
Frame(layout: region(mode: scroll)) {

  // App shell
  Frame(layout: grid(
    columns: [range(200pt, 18rem), 1fr],
    rows: [fit, 1fr],
    areas: ["nav header", "nav main"],
    gap: rigid(16)
  ), width: fill, height: fill) {

    // Navigation sidebar
    Frame(area: "nav", layout: stack(block, gap: rigid(4)), padding: rigid(12)) {
      Frame(content: text("Home"))
      Frame(content: text("Settings"))
    }

    // Header
    Frame(area: "header", layout: stack(inline, gap: spring), padding: rigid(16)) {
      Frame(content: text("Dashboard"), self_align_block: baseline)
      Frame(content: icon("menu"), self_align_block: center)
    }

    // Main content
    Frame(area: "main", layout: stack(block, gap: rigid(16)), padding: rigid(16)) {

      // Responsive card grid
      Frame(layout: grid(
        columns: [range(250pt, 1fr)],
        gap: rigid(16)
      )) {
        Card(title: "Revenue", body: "$42,000")
        Card(title: "Users", body: "1,234")
        Card(title: "Orders", body: "567")
      }

      // Content with overlay
      Frame(layout: layer) {
        Frame(layout: flow, content: long_text)
        Frame(content: badge("New"), self_align: (end, start))
      }
    }
  }
}
```

---

### 8.11 Complete Specification Summary

```
╔═══════════════════════════════════════════════════════════════════╗
║  THE FRAME LAYOUT SYSTEM — COMPLETE SPECIFICATION                 ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  TYPES (6):                                                       ║
║    Frame, Size, Space, Constraints, Align, Overflow               ║
║                                                                   ║
║  LAYOUTS (5 + custom):                                            ║
║    flow       — text and inline content                           ║
║    stack      — 1D sequential (horizontal/vertical)               ║
║    grid       — 2D tracks with areas and subgrid                  ║
║    layer      — overlapping with anchor positioning               ║
║    region     — fragmentation (scroll / page / columns)           ║
║    custom     — user-defined via protocol                         ║
║                                                                   ║
║  SIZES (6 kinds):                                                 ║
║    fixed, fraction, fit, fill, range, expr                        ║
║                                                                   ║
║  SPACES (1 type, 3 params):                                       ║
║    Space(natural, stretch, shrink)                                 ║
║    Constructors: rigid, flex, spring, soft                         ║
║                                                                   ║
║  ALIGNMENTS (5):                                                  ║
║    start, center, end, stretch, baseline                          ║
║                                                                   ║
║  OVERFLOWS (4):                                                   ║
║    clip, scroll, visible, wrap                                    ║
║                                                                   ║
║  PROTOCOL (3 steps):                                              ║
║    constrain → size (clamped) → place                             ║
║    + intrinsic queries + exposed baselines/anchors                 ║
║                                                                   ║
║  GUARANTEES:                                                      ║
║    • O(n) layout time (single-measure rule)                       ║
║    • Deterministic output (spec-defined rounding)                 ║
║    • Composable to arbitrary depth                                ║
║    • Logical directions throughout (start/end/block/inline)       ║
║    • No margin collapsing, no floats, no global cascade           ║
║    • Platform-agnostic, renderer-independent                      ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

---

### 8.12 Open Considerations

The following areas are essential for a production-quality system but are acknowledged as requiring deeper specification work:

#### Animation and Layout Transitions

Layout properties (sizes, positions, spacing) should be **interpolatable** by default. When a component transitions between layout states (e.g., compact → expanded responsive mode), the system should support declarative transitions:

```
Frame(
  layout: stack(block),
  transition: { duration: 300ms, easing: ease-out }
)
```

The key design constraint: animations must not violate the sizing protocol. Intermediate animation frames are computed by interpolating between two fully-resolved layout states, not by running the layout algorithm at sub-frame increments. This keeps animation from becoming a layout performance hazard.

#### Error States and Graceful Degradation

The system must define behavior for:
- **Malformed trees** (e.g., circular references): Reject at parse time where possible; at layout time, break the cycle by treating one reference as `fit`.
- **Conflicting sizes** (e.g., `min_width: 500, max_width: 300`): Resolve deterministically by clamping: `min` wins over `max` (consistent with CSS behavior).
- **Impossible constraints** (parent too small for all fixed children): Overflow policy applies. If `clip`, content is clipped. If `scroll`, scrollbar appears. If `visible`, content overflows visually. The system never silently drops content.

A conforming implementation must produce the same output for the same malformed input — errors are deterministic, not undefined behavior.

#### Tooling and Debugging

A layout system is only as good as its debuggability. The specification should mandate:
- **Inspection API:** Every Frame exposes its resolved constraints, chosen size, placement coordinates, and overflow state.
- **Visual overlay mode:** Borders around every Frame showing its content box, padding, and margin (like browser DevTools).
- **Constraint trace:** For any Frame, the ability to see "I was offered [min, max] and chose [size] because [reason]."
- **Layout performance profiling:** Per-subtree measurement counts and timing.

Implementations are encouraged to provide these as built-in development tools.

#### Migration from Existing Systems

For real-world adoption, the system must coexist with existing layout systems:
- **CSS mapping:** The five layout modes map cleanly to CSS: `flow` → normal flow, `stack` → Flexbox, `grid` → CSS Grid, `layer` → positioned overlay, `region` → paged media / overflow. A transpiler or polyfill path is feasible.
- **Native toolkit mapping:** `stack` → SwiftUI HStack/VStack, Flutter Row/Column, WPF StackPanel. `grid` → SwiftUI Grid, WPF Grid. The protocol maps to Flutter's constraints-down/sizes-up model.
- **Incremental adoption:** Frame can be used as the layout engine within an existing rendering framework, replacing only the layout pass. Taffy demonstrates this architecture.

#### Mixed Content Embedding

When a Frame tree needs to embed foreign content (a web view, a 3D canvas, a video player):
- The foreign content is treated as a `Leaf` Frame with **externally reported intrinsic size**.
- The Frame system constrains and places it like any other Frame.
- Rendering of the foreign content is delegated to the appropriate engine.
- The Frame system does not attempt to layout *inside* foreign content.

#### Governance and Standardization

For century-scale durability:
- **Open specification:** Plain text, version-controlled, freely available.
- **Core v1 frozen:** Once ratified, the core primitives and protocol are immutable. Innovation happens through additive extension modules.
- **Reference implementation:** A portable, open-source layout engine (Rust or similar systems language) with a comprehensive test suite.
- **Conformance tests:** A canonical set of input trees + expected output geometries. Any compliant implementation must pass all tests with bit-identical results (given spec-defined rounding rules).
- **Extension registry:** Named, versioned extensions (e.g., `masonry-v1`, `circular-v1`) that are optional but standardized when adopted.
- **No single-vendor dependence:** Multiple independent implementations from day one.

#### Adversarial and Pathological Inputs

The O(n) guarantee holds because:
- The single-measure rule bounds per-node work to O(1) amortized.
- Intrinsic queries add at most one additional pass per node.
- Grid track sizing is O(tracks × items-in-track) per grid, which is O(n) overall across the tree.

**Potential hazards and mitigations:**
- **Deeply nested trees** (10,000+ levels): Stack depth is bounded by tree depth; implementations may use iterative traversal or a trampoline to avoid stack overflow.
- **Thousands of grid items:** The grid algorithm remains linear in item count. Track resolution is linear in track count.
- **Recursive intrinsic queries:** Bounded to one level of "dry run" queries; no recursive intrinsic-within-intrinsic calls allowed.
- **Adversarial size expressions:** Expressions are evaluated in constant time (no loops, no recursion in the expression language).

---

### 8.13 Why This Works for Centuries

| Property | How Frame achieves it |
|----------|----------------------|
| **Mathematical soundness** | Built on TeX glue (proven 45+ years) and constraint intervals (proven in PL type theory) |
| **Minimal core** | 5 layouts, 6 size types, 1 space type, 1 protocol. Orthogonal — nothing can be removed, nothing overlaps |
| **Complete** | Any rectangular 2D layout expressible: grid + layer = arbitrary placement; flow handles text; region handles output |
| **Composable** | Everything is a Frame. Frames nest. Custom layouts are first-class. |
| **Efficient** | O(n) guaranteed. No constraint solving. No multi-pass. |
| **Deterministic** | Same tree + same root constraints = bit-identical output |
| **Extensible** | Custom layout protocol handles unforeseen needs without core changes |
| **Platform-agnostic** | Describes intent and constraints, not pixels. Targets screen, print, VR, audio |
| **Archival** | Open text format, formal spec, frozen core, conformance tests, multiple implementations |
| **Governable** | Frozen core + extension modules = stability without stagnation |

**What could change in 100+ years?**

| Future change | Resilient? | Why |
|---------------|------------|-----|
| Non-rectangular screens | ✅ | Custom layout can handle any geometry |
| 3D/spatial interfaces | ⚠️ | Needs z-axis extension — but 2D layout remains necessary as 3D panels are 2D surfaces |
| Neural/AI layout | ✅ | Custom layout can wrap a model; core protocol unchanged |
| Variable-shape elements | ⚠️ | Frame assumes rectangles — a future extension could add shape masks |
| Infinite canvas | ✅ | region(scroll) + virtual handles any scale |
| Thought-based input | ✅ | Layout is spatial arrangement regardless of input method |
| New display technologies | ✅ | Output-neutral; renderers adapt, the layout tree stays the same |

**The core abstractions — boxes, flexible spacing, negotiated constraints, composability, and fragmentation — are not technological artifacts. They are mathematical realities of dividing bounded 2D space. The math doesn't expire.**


Here is a comprehensive research overview of modern 2D layout systems, analyzing their philosophies, efficiencies, and tradeoffs. Finally, based on this research, I will propose a theoretical, century-proof layout system.

Part 1: Modern Layout Systems & Approaches
Modern layout systems generally fall into categories based on their target medium: the Web (CSS), Documents (LaTeX, Typst), Native Apps (Apple Auto Layout, Flutter), and Games/UI (Canvas/Immediate mode).

1. The Web Paradigm: CSS (Grid & Flexbox)
CSS evolved from a document layout language into a massive UI layout engine. Modern CSS layout relies entirely on Flexbox (1D layout) and Grid (2D layout).

How it works: A multi-pass rule-based engine. It resolves extrinsic constraints (the size of the window) against intrinsic constraints (the size of the text/images) through a complex cascade of properties.
Pros: Supremely expressive. Separates content (HTML) from presentation. Handles text-wrapping, writing modes (RTL, vertical), and responsiveness brilliantly.
Cons: Monumental complexity. The CSS specification is tens of thousands of pages long. It has legacy baggage (margin collapse, float clearing). Resolving deep layout trees can be slow because it often requires multiple passes (e.g., measuring a child, calculating the parent, passing the new constraint back down).
Efficiency: Variable. Generally highly optimized by browser engines (Blink, WebKit), but computationally heavy due to its rule complexity.
2. The Document Paradigm: TeX & Typst
These systems focus on static, paginated 2D planes, prioritizing perfect typography over real-time responsiveness.

TeX / LaTeX (The Box & Glue Model): Invented by Donald Knuth. Characters are "boxes" connected by "glue" (springs). The algorithm calculates line breaks by looking at entire paragraphs to minimize jaggedness.
Cons: Slow, archaic macro-based language, highly rigid.
Typst: The modern successor to LaTeX. Written in Rust.
How it works: A highly pure, functional, rule-based styling system. It relies heavily on incremental compilation (only recalculating what changed). It uses a hybrid approach: grids, blocks, and absolute positioning, tightly integrated with a modern programming language.
Pros: Blazing fast, readable syntax, deterministic.
Cons: Unsuited for interactive, resizable app UIs.
3. Constraint-Based Systems: Apple Auto Layout (Cassowary Algorithm)
Used primarily in iOS/macOS development.

How it works: You define declarative math equations. "Button A's left edge = Button B's right edge + 10px." An linear equation solver (Cassowary) resolves all equations simultaneously.
Pros: Incredibly flexible. You can express almost any relationship between elements.
Cons: The "Performance Cliff." The constraint solver has an exponential worst-case time complexity (O(n³), though mostly O(n) in practice). Deep UI trees can cause massive lag. Furthermore, contradictory constraints crash or break the UI unpredictably.
4. Single-Pass Compositional Systems: Flutter & Jetpack Compose
The dominant approach in modern native cross-platform UI.

How it works (The Protocol): "Constraints go down. Sizes go up. Parent sets position." A parent widget passes maximum/minimum width and height to a child. The child looks at these constraints, determines its own size, and tells the parent. The parent then places the child.
Pros: O(n) time complexity. It is strictly a single-pass algorithm. Extremely fast, deeply predictable, and developer-friendly. Everything is a nested component.
Cons: "Callback hell" or deep widget trees. Layout logic is often tangled tightly with UI components, unlike CSS.
5. Emerging Solutions & Paradigms
Subgrid (CSS): Allows children of a grid to align to the parent's grid lines, solving a long-standing issue in component-based web design.
Container Queries (CSS): Elements query the size of their parent container, not the viewport. This makes components truly modular and self-contained.
Taffy / Yoga: Standalone Rust/C++ layout engines that implement CSS Flexbox/Grid but decouple them from the browser, allowing web-like layout in native apps and game engines.
Part 2: Brainstorming Properties of the "Ultimate" Layout System
To design a system that outlives current tech, we must isolate the fundamental truths of two-dimensional space. Modern systems fail because they build rules instead of axioms. CSS has too many rules; Cassowary has too much math.

Crucial Features for the Future:

Determinism: Given tree A and constraints B, the visual output must be identical across all compilers, screens, and centuries.
O(n) Efficiency: Layout must be solvable in a single pass of the tree, from root to leaf and back. No infinite loops. No multi-pass measuring.
Resolution Independence: No "pixels." Space must be divided via vectors, ratios, and relative constraints.
Bidirectional Flow: Must seamlessly support Left-to-Right, Right-to-Left, and Top-to-Bottom.
Separation of State and Space: The layout system should only care about geometry, not UI state, colors, or interactivity.
Part 3: Designing the "Century" Layout System
Name: CoreGrid (or the Axiomatic Layout Protocol)

To last centuries, the system cannot rely on web browsers, specific graphics APIs, or current trends. It must be a purely mathematical algorithm based on topology and geometry, simple enough to be taught to a student on a chalkboard, yet powerful enough to layout a dashboard.

Phase 1: The Core Axiom (The Protocol)
Like Flutter, we use a strict, single-pass protocol guaranteed to run in O(n) time.

Proposal: Parent offers Space (Min/Max bounding box).
Decision: Child consumes Space, returning its Volume.
Placement: Parent assigns a Coordinate (X, Y) to the child's Volume.
Phase 2: Sizing Primitives (How things get big)
Forget pixels. Space is defined by three unchangeable concepts:

Intrinsic (Fix): The exact size of the content (e.g., the bounding box of a font glyph or an image).
Extrinsic (Fill): Consumes all remaining available space offered by the parent. Can be weighted (e.g., Fill(2) takes twice as much space as Fill(1)).
Relative (Ratio): A strict mathematical percentage of the parent's space.
Phase 3: The 4 Universal Components
Instead of 100 HTML tags or thousands of CSS rules, all 2D layouts in the universe can be constructed using only four primitive containers.

1. Stack (The Z-Axis)
Overlays children directly on top of one another.

Use: Background images, modals, badges, absolute positioning.
Alignment: Centers, Edges, or exact X/Y offsets within the stack.
2. Track (The 1D Axis - Replaces Flexbox)
Lays out children in a single direction (Horizontal or Vertical).

Property - Wrapping: If a child exceeds the track's max-size, it pushes to a new parallel track. (Solves text-wrapping and responsive flex-wrapping).
Use: Toolbars, paragraphs of text, lists.
3. Lattice (The 2D Axis - Replaces Grid)
A rigid 2D division of space. You define rows and columns using the Sizing Primitives (Fix, Fill, Ratio), and assign children to cells.

Use: Complex app layouts, tables, dashboard panels.
4. Leaf (The Content)
The terminal node of the tree. A Leaf cannot have children. It only contains fundamental data: Text, Vector paths, or Raster graphics.

Phase 4: Why this system works for centuries
Let's look at the pros of the Axiomatic Layout Protocol:

Zero Edge Cases: Because there are no "margin collapsing" rules, floats, or z-indexes, the system behaves exactly as the math dictates. Spacing is achieved simply by placing an empty Fix box between elements.
Future-Hardware Proof: Whether rendering on a 2D screen, e-ink, projected holograms, or spatial computing (AR/VR), laying out a 2D plane requires the same topological math.
Performant by Definition: Because a parent defines the constraints, and a child reports its size, rendering an endlessly scrolling list is trivial. The system only processes the Tracks currently within the viewport's bounding box.
Framework Agnostic: This isn't tied to React, Rust, or C. It is a mathematical protocol that could be implemented in any syntax:

# Example syntax of a Century-proof UI
Layout: Stack
  Children:
    - Image: "background.jpg"
      Size: [Fill(1), Fill(1)] # Fills the screen
    
    - Layout: Track
      Direction: Vertical
      Alignment: Center
      Children:
        - Text: "Welcome"
          Size: [Intrinsic, Intrinsic]
        - Space: [Fix(20), Fix(20)] # Replaces Margin/Padding
        - Button: ...
Summary Conclusion
Modern systems (CSS, Auto Layout) are buckling under the weight of their own historical additions. Typst and Flutter point toward the future: purely mathematical, single-pass, component-driven hierarchies.

By stripping away rendering paradigms and focusing purely on the topological relationship between boundary boxes via Tracks, Lattices, and Stacks—and sizing them only by Intrinsic content and Extrinsic fractions—we create a layout system that requires no updates, has no bugs, and runs at native mathematical speeds across any medium forever.
