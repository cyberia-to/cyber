tags:: page, prism, cyb
crystal-type:: entity
crystal-domain:: cyber
- # Sigma
- wallet and balance [[aip]] in [[cyb]]
- widget molecule and full application in [[prism]]
- the economic interface between a [[neuron]] and the [[cybergraph]]
- ## interface
	- inputs
		- token balances: [[CYB]], [[HYDROGEN]], [[BOOT]], [[VOLT]], [[AMPERE]], and IBC tokens
		- staking state: delegations, rewards, unbonding
		- portfolio value: aggregated across all [[token]] types
	- outputs
		- send action → token transfer
		- stake action → delegation to subnet
		- navigate action → opens token detail or [[cyberver]]
- ## as widget (molecule)
	- compact balance display in the [[prism/hud]]
	- shows total portfolio value as [[counter]]
	- token breakdown on expand
	- [[emotion]] color reflects portfolio trend (green rising, red falling)
- ## as aip
	- full-screen token management
	- pages
		- [[coins]]: fungible token balances and transfers
		- [[uniqs]]: unique tokens and collectibles
		- [[scores]]: reputation and contribution metrics
		- [[badges]]: achievement tokens
	- with [[focus]] on [[value optimization]]