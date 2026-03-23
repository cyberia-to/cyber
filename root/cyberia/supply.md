---
tags: cyberia
crystal-type: entity
crystal-domain: cyberia
stake: 4855408837394604
diffusion: 0.00011233815923477823
springs: 0.00008246788381555257
heat: 0.000037649645370807764
focus: 0.00008843937383621902
gravity: 0
density: 0
---
- {{query (property :supply "yes")}}
-
- ## ironwoods
- {{query (and (property :supply "next-month") (property :market "ironwoods"))}}
- ## hardwoods
- {{query (and (property :supply "next-month") (property :project "hardwoods"))}}
- ## resins
- {{query (and (property :supply "next-month") (property :project "resins"))}}
- ## rhizomes
- {{query (and (property :supply "next-month") (property :project "rhizomes"))}}
- {{query (property :supply "next-month")}}
-
- {{query (and (page-property :supply "next-month") (page-property :project "edible-oil"))}}
-
-
-
- {{query (property :state "supply")}}