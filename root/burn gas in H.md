---
tags: bip
crystal-type: process
crystal-domain: cyber
status: accepted
stake: 11513360236727042
diffusion: 0.00015862656569068747
springs: 0.00007613470641567186
heat: 0.00010946740640862273
focus: 0.00012404717605177048
gravity: 3
density: 5.59
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