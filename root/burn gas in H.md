---
tags: bip
crystal-type: process
crystal-domain: cyber
status: accepted
stake: 11513360236727042
diffusion: 0.00015427416842733301
springs: 0.00007381757744013412
heat: 0.00010768138046286858
focus: 0.000120818633538284
gravity: 3
density: 4.66
---
proposal to improve [[cybernomics]] of [[fuel]]

two possible of implementation vectors

- global gas feature of [[cosmos-sdk]]
	- pros: easy
	- cons: need additional consensus parametrs
		- [[param]]
			- [[percent of gas]]
				- which will be burned
				- relative to total transaction fees
- eip1559 like mechanism
	- pros: parametr is optimized by mechinsm
	- cons: need more research how to implement