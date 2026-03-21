---
tags: bip
crystal-type: process
crystal-domain: cyber
status: accepted
stake: 11513360236727042
diffusion: 0.00014644224139516443
springs: 0.00006478519302445612
heat: 0.00009673159543563837
focus: 0.00011200299769204529
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