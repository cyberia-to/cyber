---
icon: 🏗️
alias: bostrom architecture
tags: bostrom, infrastructure, architecture
crystal-type: entity
crystal-domain: cyber
stake: 32557072345120688
---
# Architecture

- Back to [[bostrom infrastructure]]

## System Overview

- ![Infrastructure Diagram](../media/Bostrom_infra.drawio.svg)
- 📎 [[../media/cyber-full-architecture.drawio][Full Architecture Map (Draw.io)]]
- ### Server Details

| Server | Role | Specs |
|--------|------|-------|
| Cyberproxy | Nginx, Frontend | CX31 |
| Deimos | Archive + Cyberindex | GTX 1080, 62GB, 6.8TB |
| Jupiter | RPC + IBC Hermes | GTX 1080, 62GB, 6.9TB |
| IO | IPFS + Cluster | 62GB, 3.5TB |
| Mimas | Grafana + Prometheus | CX41 |
| Port | Market APIs | CX21 |
| Helia-Relay | libp2p relay | CX32 |

## Node Roles

- ### Archive Node
	- Maintains complete blockchain history from genesis
	- Powers the [[cyberindex]] GraphQL API
	- Used for historical queries and block explorer
	- NOT a validator — infrastructure node only
- ### RPC Node
	- Pruned node with recent blockchain state
	- Serves public RPC, LCD, and gRPC endpoints
	- Runs [[Hermes]] IBC relayer for cross-chain connectivity
	- NOT a validator — infrastructure node only
- ### IPFS Storage
	- Stores content-addressed data for the [[knowledge graph]]
	- Provides public IPFS gateway
	- Runs IPFS cluster for redundancy

## Hardware Requirements

- Running a Bostrom node requires:
	- GPU: NVIDIA GTX 1080 or better (required for consensus)
	- RAM: 64GB recommended
	- Storage:
		- Archive node: 5TB+ (grows over time)
		- Pruned node: 500GB
	- CPU: Modern multi-core processor
- #+BEGIN_NOTE
	  GPU is required for Bostrom consensus. The network uses GPU-accelerated PageRank for the [[knowledge graph]] ranking.
	  #+END_NOTE

## Data Flow

- 1. Users interact with [[cyb.ai]] web interface
	  2. Frontend connects to RPC/LCD endpoints via reverse proxy
	  3. Queries that need historical data go to the indexer (Archive Node)
	  4. Content is fetched from IPFS gateway
	  5. Cross-chain transfers use IBC through the relayer

## Redundancy

| Service | Redundancy |
|---------|------------|
| RPC endpoints | Multiple nodes behind load balancer |
| Blockchain data | ZFS snapshots (12h intervals) |
| IPFS content | Cluster replication |
| IBC relaying | Automatic restart on failure |

## Related

- [[API endpoints]]
- [[bostrom monitoring]]
- [[go-cyber]]
