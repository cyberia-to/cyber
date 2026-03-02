---
icon: ⛓️
alias: chain config, bostrom chain config
tags: bostrom, infrastructure, wallet, keplr, config
crystal-type: entity
crystal-domain: cyber
stake: 41108246711070904
---
- # Chain Configuration
	- Back to [[bostrom infrastructure]]
-
- ## Network Details

| Parameter | Value |
|-----------|-------|
| Chain ID | `bostrom` |
| Chain Name | Bostrom |
| Native Token | BOOT |
| Bech32 Prefix | `bostrom` |
| Coin Type | 118 (Cosmos) |
| Block Time | ~5 seconds |
| Consensus | CometBFT (Tendermint) |

-
- ## RPC Endpoints

| Type | URL |
|------|-----|
| RPC | `https://rpc.bostrom.cybernode.ai` |
| REST/LCD | `https://lcd.bostrom.cybernode.ai` |
| WebSocket | `wss://rpc.bostrom.cybernode.ai/websocket` |

-
- ## Tokens

| Token | Denom | Decimals | Description |
|-------|-------|----------|-------------|
| BOOT | `boot` | 0 | Native staking and gas token |
| HYDROGEN | `hydrogen` | 0 | Resource token for bandwidth |
| MILLIAMPERE | `milliampere` | 3 | Resource token (1/1000 AMPERE) |
| MILLIVOLT | `millivolt` | 3 | Resource token (1/1000 VOLT) |

-
- ## Add to Keplr Wallet
	- Bostrom is automatically suggested when you visit https://cyb.ai
	- Or add manually using this chain info:
		- ```json
		  {
		    "chainId": "bostrom",
		    "chainName": "Bostrom",
		    "rpc": "https://rpc.bostrom.cybernode.ai",
		    "rest": "https://lcd.bostrom.cybernode.ai",
		    "bip44": {
		      "coinType": 118
		    },
		    "bech32Config": {
		      "bech32PrefixAccAddr": "bostrom",
		      "bech32PrefixAccPub": "bostrompub",
		      "bech32PrefixValAddr": "bostromvaloper",
		      "bech32PrefixValPub": "bostromvaloperpub",
		      "bech32PrefixConsAddr": "bostromvalcons",
		      "bech32PrefixConsPub": "bostromvalconspub"
		    },
		    "currencies": [
		      {
		        "coinDenom": "BOOT",
		        "coinMinimalDenom": "boot",
		        "coinDecimals": 0
		      }
		    ],
		    "feeCurrencies": [
		      {
		        "coinDenom": "BOOT",
		        "coinMinimalDenom": "boot",
		        "coinDecimals": 0,
		        "gasPriceStep": {
		          "low": 0,
		          "average": 0,
		          "high": 0.01
		        }
		      }
		    ],
		    "stakeCurrency": {
		      "coinDenom": "BOOT",
		      "coinMinimalDenom": "boot",
		      "coinDecimals": 0
		    }
		  }
		  ```
-
- ## Add to Leap Wallet
	- Leap should auto-suggest Bostrom. If not, use the same chain config as Keplr above.
-
- ## Add to Cosmostation
	- Bostrom is listed in Cosmostation's default chains.
-
- ## Block Explorers

| Explorer | URL |
|----------|-----|
| cyb.ai Oracle | https://cyb.ai/oracle |
| Mintscan | https://mintscan.io/bostrom |
| ATOMScan | https://atomscan.com/bostrom |

-
- ## Faucet
	- There is no public faucet. To get BOOT:
		- 1. Bridge from Osmosis (if you have OSMO) via [[bostrom IBC]]
		  2. Ask in the community Telegram
		  3. Buy on Osmosis DEX
-
- ## Gas Fees
	- Bostrom has very low gas fees:
		- Minimum gas price: `0 boot` (most txs are free)
		- Recommended: `0.01 boot` per gas unit for priority
		- Average transaction cost: < 1 BOOT
-
- ## Staking

| Parameter | Value |
|-----------|-------|
| Unbonding Period | 21 days |
| Max Validators | 100 |
| Inflation | Dynamic (see governance) |

	- Stake via:
		- https://cyb.ai/sphere (official)
		- Keplr wallet staking interface
		- Cosmostation
-
- ## Governance

| Parameter | Value |
|-----------|-------|
| Voting Period | 7 days |
| Min Deposit | 42,000,000,000 BOOT |
| Quorum | 33.4% |

	- Vote on proposals at https://cyb.ai/senate
-
- ## Related
	- [[API endpoints]]
	- [[bostrom IBC]]
	- [[cyb]]
