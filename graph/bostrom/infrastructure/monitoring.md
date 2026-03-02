---
icon: 📊
alias: bostrom monitoring, cybernode status
tags: bostrom, infrastructure, monitoring, status
crystal-type: process
crystal-domain: cyber
stake: 27762117110846052
---
# Monitoring & Status

- Back to [[bostrom infrastructure]]

## Public Status Dashboard

- URL: https://cybernode.ai
- The status page shows real-time health of all public services with 90-day uptime history.

## What's Monitored

- ### Endpoint Health

| Endpoint | Check |
|----------|-------|
| RPC | HTTP 200 + valid response |
| LCD | HTTP 200 + valid JSON |
| GraphQL | Query execution success |
| IPFS Gateway | Content retrieval |
| cyb.ai | Page load + content check |
- ### SSL Certificates
	- All endpoints are monitored for SSL certificate expiry with alerts 30 days before expiration.
- ### Blockchain Sync
	- Block height is monitored to detect if nodes fall behind the network.
- ### IBC Relayer
	- Wallet balances and packet relay success rate are monitored.

## Dashboards

- Public Grafana dashboards (no login required):
	- HTTPS Endpoints Status: https://cybernode.ai/grafana/public-dashboards/48ffa0bb018e424bb6aa71c2bcab42c9
	- Real-time HTTP probe results for all public endpoints

## Metrics Stack

| Component | Purpose |
|-----------|---------|
| Prometheus | Time-series metrics collection |
| Grafana | Visualization and alerting |
| Blackbox Exporter | HTTP/SSL endpoint probing |
| Node Exporter | Server hardware metrics |

## Alert Categories

- ### Infrastructure Alerts
	- Disk space, RAM, CPU, system load
	- Block counter stalls (node not producing blocks)
	- ZFS pool health
	- GPU status (required for consensus)
- ### Service Alerts
	- API endpoint availability
	- SSL certificate expiry
	- IPFS gateway responsiveness
	- IBC relayer wallet balance

## Uptime Targets

| Service | Target |
|---------|--------|
| RPC/LCD | 99.9% |
| GraphQL | 99.5% |
| IPFS Gateway | 99% |
| cyb.ai | 99.9% |

## Incident Response

- Alerts are routed to the infrastructure team via Telegram.
- Critical services auto-restart on failure.
- ZFS snapshots enable quick rollback if needed.

## Historical Data

- Prometheus retains 90 days of metrics history, enabling:
	- Trend analysis
	- Capacity planning
	- Post-incident investigation

## Related

- [[bostrom architecture]]
- [[API endpoints]]
