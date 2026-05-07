---
tags: cyb, cyber, core
alias: table particle, tabular data, csv, dataset
crystal-type: entity
crystal-domain: cyb
---
2D data — rows and columns — as [[particle]]. the native format for datasets, time series, grids, rankings, and measurements in the [[cybergraph]]

source format: CSV, TSV — any rectangular data with a header row and consistent columns

---

## rendering

```
csv source → parse → column type inference → virtualized grid → GPU text cells
```

only the visible slice renders regardless of row count. a table particle with 10M rows renders as fast as one with 10. column types (number, text, date, address) infer automatically and render with appropriate alignment and formatting. sortable. filterable. resizable columns

## in the cybergraph

table is the native format of the knowledge economy. quantitative knowledge lives as table particles — and quantitative knowledge is what earns

types of table particles: [[karma]] ledgers, [[cyberank]] scores, experimental data, genomic expression matrices, financial time series, sensor streams, trial results, population statistics, astronomical catalogs, election results, market orders, protein interaction networks, benchmark scores

a table particle is often the most linked type in scientific domains: the data that other papers cite is a table. the dataset that underlies a finding is a table. the [[cybergraph]] makes data citation first-class — link to the table, not to the paper that contains it

## properties

- losslessly portable — CSV is the most universal data format. any tool can read it
- column semantics emerge from topology — the [[cybergraph]] discovers what a column means by seeing how particles with similar columns cluster
- streaming-capable — large table particles load progressively. the robot renders partial data while the rest transfers
- composable as state — a [[component]] particle binds a table particle as its data source, rendering interactive charts, forms, and dashboards from live table data

## relation to other languages

table carries the numbers. [[formula]] carries the equations that model those numbers. [[text]] carries the interpretation of those numbers. [[vector]] visualizes them. together they are the complete scientific record

see [[csv]] for the source format. see [[datalog]] for querying table particles. see [[component]] for binding table data to interactive UI

discover all [[concepts]]
