icon:: 🖥️
alias:: [[cybernode]], [[bostrom infrastructure]]
tags:: bostrom, infrastructure, cybernode

- # Bostrom Infrastructure
- The Bostrom network is supported by a decentralized infrastructure operated by the [[cyberia]] team and community validators. This documentation covers the public infrastructure available to developers and users.
-
- ## Quick Start
	- | I want to... | Go to... |
	  |--------------|----------|
	  | **Connect my wallet** | [[chain config]] |
	  | **Use the API** | [[API endpoints]] |
	  | **Bridge tokens** | [[IBC bridge]] |
	  | **Check network status** | https://cybernode.ai |
	  | **Run a node** | [[go-cyber]] |
-
- ## Documentation
	- [[chain config]] — Chain ID, RPC, token info for wallets
	- [[API endpoints]] — Public API endpoints for developers
	- [[bostrom architecture]] — How the infrastructure is organized
	- [[bostrom monitoring]] — Network status and uptime
	- [[bostrom IBC]] — Cross-chain connectivity
	- [[bostrom security]] — Security practices and considerations
-
- ## Overview
	- The infrastructure consists of several components:
		- **Archive Node** — Full blockchain history for indexing and historical queries
		- **RPC Node** — Pruned node for fast public API access
		- **Indexer** — [[cyberindex]] GraphQL API for complex queries
		- **IPFS Storage** — Decentralized content storage for the [[knowledge graph]]
		- **Reverse Proxy** — Load balancing and SSL termination
		- **IBC Relayer** — Cross-chain packet relay to [[Osmosis]] and [[Cosmos Hub]]
-
- ## Networks
	- | Network | Chain ID | Status |
	  |---------|----------|--------|
	  | **Bostrom** | `bostrom` | ✅ Mainnet |
	  | **Space Pussy** | `space-pussy-1` | 🧪 Experimental |
-
- ## Software Stack
	- | Component | Technology |
	  |-----------|------------|
	  | Blockchain | [[go-cyber]] (Cosmos SDK v0.47, CometBFT v0.37) |
	  | Smart Contracts | [[CosmWasm]] v1.5 |
	  | Indexer | [[cyberindex]] (Hasura GraphQL) |
	  | Content Storage | [[IPFS]] (Kubo) |
	  | IBC Relayer | [[Hermes]] v1.13 |
	  | Monitoring | Prometheus + Grafana |
-
- ## For Developers
	- See [[API endpoints]] for API documentation
	- See [[go-cyber]] for node operation guides
	- See [[cyb]] for frontend integration
-
- ## Community & Support
	- | Resource | Link |
	  |----------|------|
	  | **Telegram** | https://t.me/bostrom_news |
	  | **Discord** | https://discord.gg/cyber |
	  | **GitHub** | https://github.com/cybercongress |
	  | **Forum** | https://commonwealth.im/bostrom |
	  | **Twitter/X** | https://x.com/cyber_devs |
-
- ## Status
	- **Live Status**: https://cybernode.ai
	- Real-time monitoring of all endpoints and services with 90-day uptime history
-
- ## Contributing
	- Infrastructure is open source. Contributions welcome:
		- [[go-cyber]] — Blockchain node
		- [[cyb-ts]] — Web frontend
		- [[cyberindex]] — GraphQL indexer