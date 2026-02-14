icon:: 💎
tags:: cyb, ui
alias:: design system

# Prism

The design system and component library for [[cyb]]. Every screen in cyb is composed from prism components, organized in four levels of composition: atoms → molecules → cells → aips.

## Atoms

The smallest visual units — indivisible primitives.

### Surfaces
- [[glass]] — background surfaces and panes

### Typography
- text — typography blocks (left, center, right, paragraph, image)

### Controls
- [[button]] — call-to-action (default, double, triple, side)
- [[toggle]] — binary switches (on, off, star)
- slider — value ranges (20%, 50%, 80%)

### Indicators
- indicator — progress indicators (50%, full)
- [[counter]] — numeric displays with optional emotion color

### Display Elements
- address — neuron address display (big, small)
- [[ion]] — icon-label pairs in six layouts (centric, horizontal, input, star, trapezoid)
- saber — accent lines and dividers (1px, 2px, horizontal)

### Icons
- [[images]] — icon library at 16×16, 20×20, 32×32, 48×48, 96×96

## Molecules

Functional components assembled from atoms.

### Navigation and HUD
- hud — heads-up display shell
- [[mind]] — navigation awareness
- [[brain]] — graph file manager widget (+memory variant)
- [[sense]] — messaging and notification widget
- [[sigma]] — wallet and balance widget
- [[time]] — personal history widget
- tabs — navigation tabs (3-items, 4-items, 5-items × desktop, mobile)

### Content Display
- [[particle]] — content renderers by type
	- headings (h1–h5), text (+icon variants), number (+indicator), link, picture, video, pdf, audio, avatar
- [[display]] — content containers (empty, highlight, text at various sizes)
- [[neuron]] — neuron identity cards (big, small × default, hover, clicked)
- object — [[particle]], [[neuron]], [[avatar]], [[aip]] cards (2-line, 3-line, +menu)
- subject — [[neuron]]/[[avatar]] identity strips (2-line, chooser)
- adviser — contextual hints (closed, opened/positive, opened/negative, opened/neutral, particle)

### Data Input
- input — data entry (text L/R/LR, neuron, token, select)
- filter — result filtering (3-items, wide)

### Data Display
- table — data grids (line, row-l, row-r, sort, sort/dropdown)
- slider — progress bars (horizontal, mobile, descending)
- saber+[[ion]] — labeled accent bars (1-sided, bi-sided, horizontal × button, input, display)

## Cells

Full page sections composed from molecules.

- [[portal]] — citizenship, gift, hud, cyb-map
- [[cyberver]] — hud, mentors, learner, stats, faculties
- [[oracle]] — aip, mind, particle, content

## AIPs

Complete autonomous applications built from cells.

- [[teleport]]
- [[sphere]]
- [[warp]]
- [[hfr]]
- [[sense]]
- [[cyberver]]
