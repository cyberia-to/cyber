---
tags: cyber, bostrom, infrastructure
alias: cybernode, bostrom infrastructure
icon: 🖥️
crystal-type: entity
crystal-domain: cyber
---

Cybernode is the infrastructure toolkit for deploying and operating [[bostrom]] [[validators]], [[IPFS]] nodes, and associated services. It provides automation scripts, configuration templates, and monitoring dashboards for running the full [[cyber]] node stack.

A typical cybernode deployment includes a consensus node for block production, an [[IPFS]] gateway for content addressing, indexers for query access, and relayers for [[IBC]] connectivity to other [[Cosmos]] chains.

The repository contains Docker compositions, Ansible playbooks, and operational runbooks that standardize the process of bringing new infrastructure online. This reduces the barrier for [[neurons]] who want to run their own [[validators]].

Cybernode operators contribute to network decentralization by running geographically distributed infrastructure. Each node independently validates the [[state transition function]], stores [[cybergraph]] data, and serves queries to clients.

Monitoring and alerting configurations help operators maintain high uptime, which directly affects their [[staking]] rewards and reputation within the [[hero]] set.

The project evolves alongside the [[cyber]] protocol, incorporating support for new features as they land in successive versions from [[bostrom]] through [[v6]].

discover all [[concepts]]
