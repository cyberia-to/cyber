---
icon: 🔌
alias: API endpoints, bostrom endpoints
tags: bostrom, infrastructure, api, endpoints
crystal-type: entity
crystal-domain: cyber
stake: 35574057006881760
---
- # Public Endpoints
	- Back to [[bostrom infrastructure]]
-
- ## Bostrom Network

| Service | URL | Description |
|---------|-----|-------------|
| RPC | `https://rpc.bostrom.cybernode.ai` | Tendermint RPC (WebSocket + HTTP) |
| LCD/REST | `https://lcd.bostrom.cybernode.ai` | Cosmos SDK REST API |
| gRPC-Web | `https://grpc.bostrom.cybernode.ai` | gRPC-Web proxy (for browser clients) |
| GraphQL | `https://index.bostrom.cybernode.ai/v1/graphql` | Cyberindex GraphQL API |

-
- ## IPFS

| Service | URL | Description |
|---------|-----|-------------|
| Gateway | `https://gateway.ipfs.cybernode.ai` | Public IPFS gateway |
| Swarm | `swarm.io.cybernode.ai` | libp2p bootstrap node |

-
- ## Monitoring

| Service | URL | Description |
|---------|-----|-------------|
| Status | `https://cybernode.ai` | Public status dashboard |
| Grafana | `https://cybernode.ai/grafana/` | Detailed metrics (public dashboards) |

-
- ## Frontend

| Service | URL | Description |
|---------|-----|-------------|
| cyb.ai | `https://cyb.ai` | Main web interface |

-
- ## API Usage Examples
	- ### RPC — Get Latest Block
		- ```bash
		  curl -s https://rpc.bostrom.cybernode.ai/status | jq '.result.sync_info.latest_block_height'
		  ```
	- ### LCD — Get Account Balance
		- ```bash
		  curl -s "https://lcd.bostrom.cybernode.ai/cosmos/bank/v1beta1/balances/bostrom1..." | jq
		  ```
	- ### LCD — Get Staking Params
		- ```bash
		  curl -s https://lcd.bostrom.cybernode.ai/cosmos/staking/v1beta1/params | jq
		  ```
	- ### GraphQL — Query Cyberlinks
		- ```graphql
		  query {
		    cyberlink(limit: 10, order_by: {height: desc}) {
		      particle_from
		      particle_to
		      neuron
		      height
		    }
		  }
		  ```
	- ### WebSocket — Subscribe to New Blocks
		- ```javascript
		  const ws = new WebSocket('wss://rpc.bostrom.cybernode.ai/websocket');
		  ws.onopen = () => {
		    ws.send(JSON.stringify({
		      jsonrpc: '2.0',
		      method: 'subscribe',
		      params: ["tm.event='NewBlock'"],
		      id: 1
		    }));
		  };
		  ```
-
- ## Rate Limits
	- Public endpoints have reasonable rate limits to ensure fair usage:
		- RPC: No strict limit, but please be considerate
		- LCD: No strict limit
		- GraphQL: Query complexity limits apply
		- IPFS Gateway: Standard gateway limits
	- For high-volume applications, consider running your own node. See [[go-cyber]] for setup instructions.
-
- ## Cosmos SDK v0.47 Notes
	- Bostrom runs Cosmos SDK v0.47 which uses gRPC-gateway REST endpoints:
		- ✅ Use: `/cosmos/*/v1beta1/*` and `/cyber/*/v1beta1/*`
		- ❌ Deprecated: Legacy REST endpoints (`/staking/parameters`, etc.)
	- Example parameter endpoints:
		- Staking: `/cosmos/staking/v1beta1/params`
		- Slashing: `/cosmos/slashing/v1beta1/params`
		- Governance: `/cosmos/gov/v1beta1/params/{params_type}`
		- Bandwidth: `/cyber/bandwidth/v1beta1/params`
		- Rank: `/cyber/rank/v1beta1/params`
-
- ## Related
	- [[bostrom architecture]]
	- [[cyberindex]]
	- [[go-cyber]]
