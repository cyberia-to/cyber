tags:: cip
status:: accepted
- proposal to improve [[cybernomics]] of [[fuel]]
- two possible of implementation vectors
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