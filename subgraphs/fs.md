---
name: fs
repo: ../fs
subgraph: true
visibility: private
archived: false
---

# fs

Cyber filesystem and particle-editing protocol layer for [[cyb]]. Defines the edit, patch, and sync primitives used across the [[cyber]] stack. Editing is non-mutating: each write creates a new [[particle]] with a [[cyberlink]] from old to new recording succession, preserving the full provenance chain in the [[cybergraph]].
