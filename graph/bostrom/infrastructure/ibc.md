---
icon: 🌉
alias: bostrom IBC, IBC bridge
tags: bostrom, infrastructure, ibc, interchain
crystal-type: entity
crystal-domain: cyber
stake: 40392445131733184
---
# IBC (Inter-Blockchain Communication)

- Back to [[bostrom infrastructure]]

## Overview

- Bostrom connects to other Cosmos chains via IBC, enabling cross-chain token transfers and interoperability.

## Active Channels

| Counterparty | Bostrom Channel | Remote Channel | Status |
|--------------|-----------------|----------------|--------|
| [[Osmosis]] | `channel-2` | `channel-95` | ✅ Active (restored Feb 2026) |
| [[Cosmos Hub]] | `channel-8` | `channel-341` | ❌ Expired (proposal #1023 failed quorum) |

## Supported Tokens

- ### Bostrom → Other Chains

| Token | Symbol | Can transfer to |
|-------|--------|-----------------|
| BOOT | Native staking token | Osmosis, Cosmos Hub |
| HYDROGEN | Resource token | Osmosis |
| MILLIAMPERE | Resource token | Osmosis |
| MILLIVOLT | Resource token | Osmosis |
- ### Other Chains → Bostrom

| Token | From | IBC Denom |
|-------|------|-----------|
| OSMO | Osmosis | `ibc/13B2C536BB057AC79D5616B8EA1B9540EC1F2170718CAFF6F0083C966FFFED0B` |
| ATOM | Cosmos Hub | `ibc/15E9C5CF5969080539DB395FA7D9C0868265217EFC528433671AAF9B1912D159` |

## How IBC Works

- 1. Light Clients: Each chain maintains a light client that tracks the other chain's consensus
	  2. Channels: Ordered or unordered channels for packet delivery
	  3. Relayers: Off-chain processes that submit proofs between chains
	  4. Packets: Messages (like token transfers) sent through channels

## Using IBC Transfers

- ### Via cyb.ai
	- Go to https://cyb.ai/teleport to use the IBC bridge interface.
- ### Via CLI
```bash
		  # Transfer BOOT to Osmosis
		  cyber tx ibc-transfer transfer transfer channel-2 osmo1... 1000000boot \
		    --from <key> --chain-id bostrom --fees 5000boot

		  # Transfer BOOT to Cosmos Hub
		  cyber tx ibc-transfer transfer transfer channel-8 cosmos1... 1000000boot \
		    --from <key> --chain-id bostrom --fees 5000boot
		  ```

## Relayer Infrastructure

- The [[cyberia]] team operates an IBC relayer using [[Hermes]] to ensure packets are delivered between chains.
- ### Relayer Features
	- Automatic packet relay (transfers, acknowledgements, timeouts)
	- Client updates to prevent expiry
	- Monitoring and alerting for failed packets
	- Auto-restart on failure
- ### Packet Filtering
	- The relayer only processes Bostrom-related channels for efficiency.

## IBC Client Recovery

- IBC clients can expire if not updated regularly (trusting period). If this happens:
	- 1. A new "substitute" client is created
		  2. A governance proposal recovers the expired client using the substitute
		  3. Both chains must pass the recovery proposal
- ### Recovery Status (Feb 2026)

| Chain | Bostrom Prop | Remote Prop | Result |
|-------|--------------|-------------|--------|
| Osmosis | #32 ✅ | #1002 ✅ | RESTORED |
| Cosmos Hub | #33 ✅ | #1023 ❌ | Failed quorum (36.7% vs 40% needed) |
	- #+BEGIN_WARNING
		  Cosmos Hub IBC requires a new proposal. The previous attempt (#1023) had 92.5% YES but failed due to insufficient voter turnout. Top validators didn't participate.
		  #+END_WARNING

## Troubleshooting

- ### Transfer Stuck
	- IBC transfers have a timeout (usually 10 minutes). If your transfer is stuck:
		- Check if the packet was relayed: query pending packets on the channel
		- Wait for timeout — tokens will be refunded automatically
		- Contact the team if issues persist
- ### Channel Status
	- Check channel state via LCD:
```bash
			  curl -s "https://lcd.bostrom.cybernode.ai/ibc/core/channel/v1/channels/channel-2/ports/transfer" | jq '.channel.state'
			  # Should return: "STATE_OPEN"
			  ```

## IBC Tokens on Osmosis

- BOOT and other Bostrom tokens are available on Osmosis DEX:
	- https://app.osmosis.zone
	- Search for "BOOT" to find trading pairs

## Related

- [[bostrom architecture]]
- [[API endpoints]]
- [[Osmosis]]
- [[Cosmos Hub]]
- [[cyb]]
