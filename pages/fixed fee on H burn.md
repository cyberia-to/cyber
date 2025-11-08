tags:: [[cip]]
status:: accepted
- [[$H]] [[staking loan]] proved its utility and reliability during last several years
- simplicity of initial implementation did not include any [[value extraction]] from discussed mechanism
- lets add simple and effecient value extraction
- on each operation of [[$H]] [[burn]] collect and [[burn]] fixed fee defined by consensus [[param]]
- proposal of fee value: 2%
- example
	- i want to burn 400 [[$H]]
	- my fee is 4 [[$H]]
		-
	- | state | [[frozen]] [[$BOOT]] of [[neuron]] | [[$H]] of [[neuron]] | [[$H]] [[supply]] |
	  | starting | 1000 | 1000 | 10000 |
	  | ending | 604 | 600 | 9996 |
- you see that in proposed scenario [[neuron]] must [[supply]] a bit more [[$H]] in order to [[unstake]] all [[$BOOT]]
- that could be a problem in case of [[automatic fuel ]]
	- but in [[explicit mint and burn of H]]
	- [[neuron]] make conscious decision to
	- pay for using [[staking loan]]
- this simple model is sustainable source of value for [[superintelligence]]