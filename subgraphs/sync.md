---
name: sync
repo: ../sync
subgraph: true
visibility: private
archived: false
---

# sync

Sync process crystal for [[cyb]] and [[cyber]]. Handles particle sync, config management, and state reconciliation between local and on-chain state. Works alongside [[fs]] to keep the local [[cybergraph]] view consistent with the network, driving the live-reload loop used by the [[cyber]] build and serve workflows.
