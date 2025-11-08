tags:: cip
status:: accepted
- currently [[bostrom]] [[mint]] [[$H]] on every staking operation automatically
- burn happens also automatically when [[neuron]] during unstake operation
- that fact create serious confusion for [[avatars]]
- and blur the line between [[staking]] and [[staking loans]]
- however these systems must be completely independent
- to overcome this lets add two explicit methods in protocol
	- [[mint fuel]]: mint $H, increase [[frozen]] $BOOT
	- [[burn fuel]]: burn $H, decrease [[frozen]] $BOOT
- by doing so we gain atomic functional unit for [[value extraction]]
- we suggest to add new token to simplify implementation: [[$STL]]
- [[$STL]] is short for [[staking loan position]]
- suggested that [[$STL]] will be minted automatically the same way current [[$H]] is minted
- in order to remove complexity and free margin trade [[$STL]] must be non transferable
- [[fixed fee on H burn]] discussing extension of this idea