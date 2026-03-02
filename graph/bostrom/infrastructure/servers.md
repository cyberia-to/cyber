---
icon: 🖥️
alias: cybernode servers, bostrom servers
tags: bostrom, infrastructure, servers, hardware
crystal-type: entity
crystal-domain: cyber
stake: 48773364902086448
---

- Back to [[bostrom infrastructure]]

## Overview

- The Bostrom infrastructure runs on 7 dedicated and cloud servers hosted at Hetzner data centers in Germany and Finland.

## Server Fleet

### Deimos — Archive Node

- Role: Full blockchain history + [[cyberindex]] GraphQL API
- Type: Dedicated server with GPU
- Location: Falkenstein, Germany
- Hardware:
	- CPU: Intel Xeon (8 cores)
	- RAM: 64 GB
	- GPU: NVIDIA GTX 1080 8GB (required for consensus)
	- Storage: 6.8 TB ZFS (RAID + NVMe)
- Services:
	- `bostrom_arch` — Archive node container (full history from genesis)
	- `cyberindex` — Blockchain indexer
	- `hasura` — GraphQL API engine
	- `postgres` — Index database
- Endpoints powered:
	- `index.bostrom.cybernode.ai` — GraphQL API

### Jupiter — RPC Node + IBC Relayer

- Role: Public RPC/LCD/gRPC endpoints + IBC packet relay
- Type: Dedicated server with GPU
- Location: Falkenstein, Germany
- Hardware:
	- CPU: Intel Xeon (8 cores)
	- RAM: 64 GB
	- GPU: NVIDIA GTX 1080 8GB (required for consensus)
	- Storage: 6.9 TB (ZFS + NVMe)
- Services:
	- `bostrom_pruned` — Pruned RPC node (recent state only)
	- `space-pussy-arch` — Space Pussy testnet archive
	- [[Hermes]] IBC relayer (systemd service)
- Endpoints powered:
	- `rpc.bostrom.cybernode.ai` — Tendermint RPC
	- `lcd.bostrom.cybernode.ai` — Cosmos REST API
	- `grpc.bostrom.cybernode.ai` — gRPC-Web proxy

### IO — IPFS Storage

- Role: Content-addressed storage for [[knowledge graph]]
- Type: Dedicated server
- Location: Falkenstein, Germany
- Hardware:
	- CPU: Intel Xeon
	- RAM: 64 GB
	- Storage: 3.5 TB HDD RAID1
- Services:
	- IPFS daemon (Kubo 0.22)
	- IPFS cluster (replication)
	- libp2p swarm bootstrap
- Endpoints powered:
	- `gateway.ipfs.cybernode.ai` — Public IPFS gateway
	- `swarm.io.cybernode.ai` — libp2p bootstrap

### Cyberproxy — Reverse Proxy + Frontend

- Role: SSL termination, load balancing, static hosting
- Type: Cloud VPS (CX31)
- Location: Nuremberg, Germany
- Hardware:
	- vCPU: 2 cores
	- RAM: 8 GB
	- Storage: 80 GB SSD
- Services:
	- Nginx reverse proxy
	- [[cyb.ai]] static frontend
	- Webhook receiver for CI/CD
- Endpoints powered:
	- `cyb.ai` — Web interface
	- `*.bostrom.cybernode.ai` — API proxy

### Mimas — Monitoring

- Role: Metrics collection, visualization, alerting
- Type: Cloud VPS (CX41)
- Location: Falkenstein, Germany
- Hardware:
	- vCPU: 4 cores
	- RAM: 16 GB
	- Storage: 160 GB SSD
- Services:
	- Grafana 12.3 — Dashboards and alerting
	- Prometheus 2.54 — Metrics collection
	- Blackbox exporter — HTTP/SSL probing
- Endpoints powered:
	- `cybernode.ai` — Public status dashboard
	- `cybernode.ai/grafana/` — Grafana dashboards

### Port — API Services

- Role: Market data and DEX APIs
- Type: Cloud VPS (CX21)
- Location: Falkenstein, Germany
- Hardware:
	- vCPU: 2 cores
	- RAM: 4 GB
	- Storage: 40 GB SSD
- Services:
	- market-data API — Token supply endpoints
	- warp-dex API — DEX trading data (when available)
- Endpoints powered:
	- `market-data.cybernode.ai` — Supply/price data

### Helia-Relay — libp2p Circuit Relay

- Role: Browser-to-browser IPFS connectivity
- Type: Cloud VPS (CX32)
- Location: Helsinki, Finland
- Hardware:
	- vCPU: 4 cores
	- RAM: 8 GB
	- Storage: 80 GB SSD
- Services:
	- libp2p circuit relay
	- Caddy (auto-renewing SSL)
- Purpose:
	- Enables browsers to connect to IPFS network
	- Relay for NAT-traversal

## Hardware Summary

| Server | Type | RAM | Storage | GPU |
|--------|------|-----|---------|-----|
| Deimos | Dedicated | 64 GB | 6.8 TB | GTX 1080 |
| Jupiter | Dedicated | 64 GB | 6.9 TB | GTX 1080 |
| IO | Dedicated | 64 GB | 3.5 TB | — |
| Cyberproxy | Cloud | 8 GB | 80 GB | — |
| Mimas | Cloud | 16 GB | 160 GB | — |
| Port | Cloud | 4 GB | 40 GB | — |
| Helia-Relay | Cloud | 8 GB | 80 GB | — |

## GPU Requirement

- Bostrom uses GPU-accelerated PageRank for the [[knowledge graph]] ranking algorithm.
- Minimum: NVIDIA GTX 1080 (8GB VRAM)
- CUDA: 11.4+
- Both Deimos and Jupiter run GPU nodes for consensus participation.
- #+BEGIN_NOTE
	  Without GPU, a node cannot participate in consensus or validate blocks. Read-only nodes (for querying) can run without GPU.
	  #+END_NOTE

## Storage Architecture

- ### ZFS
	- Deimos and Jupiter use ZFS for data integrity and snapshots
	- Automated snapshots every 12 hours with 2-day retention
	- Manual snapshots before major upgrades
- ### Data Volumes

| Data | Size | Location |
|------|------|----------|
| Bostrom archive | ~4.4 TB | Deimos |
| Bostrom pruned | ~500 GB | Jupiter |
| Cyberindex DB | ~200 GB | Deimos |
| IPFS blocks | ~1.7 TB | IO |

## Software Versions

| Component | Version |
|-----------|---------|
| go-cyber | v7.0.1 |
| Cosmos SDK | v0.47.16 |
| CometBFT | v0.37.18 |
| CosmWasm | v1.5.2 |
| IPFS (Kubo) | 0.22.0 |
| Hermes | 1.13.2 |
| Grafana | 12.3.1 |
| Prometheus | 2.54.1 |

## Redundancy & Backup

| Component | Strategy |
|-----------|----------|
| Blockchain data | ZFS snapshots + archive node |
| IPFS content | Cluster replication |
| Configuration | Git-managed |
| Cloud servers | Hetzner snapshots |

## Related

- [[bostrom architecture]]
- [[bostrom monitoring]]
- [[API endpoints]]
