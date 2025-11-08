- [[go-cyber]]
	- [[cyber/bandwidth]]
	- [[cyber/clocks]]
	  :LOGBOOK:
	  CLOCK: [2022-12-15 Thu 18:57:30]
	  :END:
	- [[cyber/cyberbank]]
	- [[cyber/dmn]]
	- [[cyber/graph]]
	- [[cyber/grid]]
	- [[cyber/liquidity]]
	- [[cyber/rank]]
	- [[cyber/resources]]
	- [[cyber/staking]]
	- [[cyber/tokenfactory]]
	- [[cyber/wasm]]
- [[cyber/cli]]
- [[cw-cyber]]
  collapsed:: true
	- names
	- addresses
- [[cosmos-sdk]]
	- bank
	  
	  collapsed:: true
		- send
		  
		  collapsed:: true
			- from-address
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- to-address
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlw,
			- denom
				- boot
				- 1000000
	- staking
		- delegate
		  
		  collapsed:: true
			- delegatorAddress
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- validatorAddress
				- bostromvaloper135ca8hdpy9sk0ntwqzpzsvatyl48ptx52tn60p,
			- denom
				- boot
				- 1000000
		- redelegate
		  
		  collapsed:: true
			- delegatorAddress
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- validatorSrcAddress
				- bostromvaloper135ca8hdpy9sk0ntwqzpzsvatyl48ptx52tn60p,
			- validatorDstAddress
				- bostromvaloper135ca8hdpy9sk0ntwqzpzsvatyl48ptx52tn60p,
			- amount: denom: boot, amount: 1000000
		- undelegate => unbond
		  
		  collapsed:: true
			- delegatorAddress
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- validatorAddress
				- bostromvaloper135ca8hdpy9sk0ntwqzpzsvatyl48ptx52tn60p,
			- amount
				- boot
				- 1000000
		- NOW create-validator
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 05:39:50]
		  
		  :END:
		- NOW edit-validator
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 05:39:56]
		  
		  CLOCK: [2022-12-16 Fri 05:39:58]
		  
		  :END:
	- gov
		- submit-proposal
		  
		  collapsed:: true
			- NOW types
			  
			  :LOGBOOK:
			  
			  CLOCK: [2022-12-16 Fri 06:42:58]
			  
			  :END:
				- default
					- community-pool-spend
						- Submit a community pool spend proposal
					- param-change
						- Submit a parameter change proposal
				- upgrade
					- cancel-software-upgrade
						- Cancel the current software upgrade proposal
					- software-upgrade
						- Submit a software upgrade proposal
				- wasm
					- wasm-store
						- Submit a wasm binary proposal
					- instantiate-contract
						- Submit an instantiate wasm contract proposal
					- update-instantiate-config
						- Submit an update instantiate config proposal.
					- execute-contract
						- Submit a execute wasm contract proposal (run by any address)
					- pin-codes
						- Submit a pin code proposal for pinning a code to cache
					- unpin-codes
						- Submit a unpin code proposal for unpinning a code to cache
					- sudo-contract
						- Submit a sudo wasm contract proposal (to call privileged commands)
					- migrate-contract
						- Submit a migrate wasm contract to a new code version proposal
					- set-contract-admin
						- Submit a new admin for a contract proposal
					- clear-contract-admin
						- Submit a clear admin for a contract to prevent further migrations proposal
				- ibc
					- ibc-upgrade
						- Submit an IBC upgrade proposal
					- update-client
						- Submit an update IBC client proposal
			- proposer
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- content
				- LATER type-url
				- title
					- title
				- description
					- description
			- LATER initial-deposit
				- boot
				- 1000000
		- deposit
		  
		  collapsed:: true
			- proposalId
				- LATER low: 1, high: 0, unsigned: true,
			- depositor
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- denom
				- boot
				- 1000000
		- vote
		  
		  collapsed:: true
			- proposalId
				- low
					- 1
				- LATER high: 0, unsigned: true,
			- voter
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- option
				- 1
		- LATER weighted-vote
	- ibc -> ibc transfer
		- transfer > transfer-send
		  
		  collapsed:: true
			- source-port
				- transfer
			- source-channel
				- channel-95
			- sender
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq
			- receiver
				- osmo1snkhz3snfeyxkmyw6zutwjlarkf9pq5vfrla7w
			- timeout-timestamp
				- low
					- 1581366208
				- high
					- 383158952
				- unsigned
					- false
			- token
				- denom
					- boot
					- 1000000
			- timeout-height
				- revision-number
					- LATER low: 0, high: 0, unsigned: true,
				- revision-height
					- LATER low: 0, high: 0, unsigned: true
		- TODO transfer-receive => recv-packet
		  :LOGBOOK:
		  CLOCK: [2022-12-15 Thu 18:57:13]
		  :END:
		- TODO transfer-ack => acknowledge-packet
		  :LOGBOOK:
		  CLOCK: [2022-12-16 Fri 07:51:04]
		  :END:
		- TODO transfer-timeout => timeout-packet
		  :LOGBOOK:
		  CLOCK: [2022-12-16 Fri 07:47:12]
		  CLOCK: [2022-12-16 Fri 07:47:14]
		  :END:
		- TODO transfer-timeout-on-close => transfer-on-close
		  :LOGBOOK:
		  CLOCK: [2022-12-16 Fri 07:57:23]
		  :END:
	- ibc
		- NOW channel
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 06:19:53]
		  
		  :END:
			- open-init
			- open-try
			- open-ack
			- open-confirm
			- close-init
			- close-confirm
		- connection
			- open-init
			- open-try
			- open-ack
			- open-confirm
		- LATER client
			- create
			- misbehaviour
			- update
			- upgrade
	- authz
		- revoke
		  
		  collapsed:: true
			- grantee
				- bostrom1…,
			- granter
				- bostrom1…,
			- msg_type_url
				- LATER /cosmos.bank.v1beta1.MsgSend
		- grant
		  
		  collapsed:: true
			- granter
				- bostrom1…,
			- grantee
				- bostrom1…,
			- grant
				- authorization
					- type
						- /SendAuthorization
					- spend_limit
						- denom
							- boot
							- 1000000
					- expiration
						- 2022-06-29T08:18:15Z
		- exec
		  
		  collapsed:: true
			- grantee
				- bostrom1...
			- msgs
				- type
					- bank/MsgSend
				- from_address: bostrom1...,
				- to_address: bostrom1...,
				- amount:
					- denom
						- boot
						- 999
	- distribution
		- LATER fund-community-pool
		- NOW set-withdraw-addr
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 06:16:09]
		  
		  CLOCK: [2022-12-16 Fri 06:16:10]
		  
		  :END:
		- NOW withdraw-all-rewards
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 06:16:12]
		  
		  :END:
		- withdraw-delegator-reward => withdraw-rewards
		  
		  collapsed:: true
			- delegatorAddress
				- bostrom1frk9k38pvp70vheezhdfd4nvqnlsm9dw3j8hlq,
			- validatorAddress
				- bostromvaloper135ca8hdpy9sk0ntwqzpzsvatyl48ptx52tn60p
	- NOW vesting
	  
	  :LOGBOOK:
	  
	  CLOCK: [2022-12-16 Fri 06:08:19]
	  
	  :END:
		- create-vesting-account
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 05:40:16]
		  
		  CLOCK: [2022-12-16 Fri 05:40:17]
		  
		  :END:
	- NOW feegrant
	  
	  :LOGBOOK:
	  
	  CLOCK: [2022-12-16 Fri 06:07:58]
	  
	  :END:
		- grant
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 06:06:48]
		  
		  :END:
		- revoke
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 06:06:51]
		  
		  :END:
	- consensus
		- evidence
		  
		  collapsed:: true
			- submit-evidence
		- crisis
		  
		  collapsed:: true
			- invariant-broken
	- NOW slashing
	  
	  :LOGBOOK:
	  
	  CLOCK: [2022-12-16 Fri 06:08:11]
	  
	  :END:
		- unjail
		  
		  :LOGBOOK:
		  
		  CLOCK: [2022-12-16 Fri 05:40:12]
		  
		  CLOCK: [2022-12-16 Fri 05:40:13]
		  
		  :END:
	- offline
		- encode
		- decode
		- sign
		- sign-batch
		- multisign
		- multisign-batch
		- validate-signatures
		- broadcast
- [[cyb-ts]]
- cy
	-