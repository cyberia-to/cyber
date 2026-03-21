---
tags: cyb, cyber
alias: edit particle, edit
crystal-type: process
crystal-domain: cyb
stake: 10825995446474682
diffusion: 0.0002464341112236965
springs: 0.0019627620925224297
heat: 0.0013857735755647559
focus: 0.0009892003984815333
gravity: 3
density: 9.46
---
create a new [[particle]] with modified content and link it to the previous version

editing does not mutate — it creates. the old [[particle]] keeps its [[Hemera]] hash. the new [[particle]] gets a new hash. a [[cyberlink]] from old → new records the succession

```
particle_v1 (hash_1) ──"next"──→ particle_v2 (hash_2)
```

the version chain is traversable in both directions (via [[backlinks]]). every version is permanent, addressable, and carries its own [[cyberank]]

see [[cyb/fs]] for the filesystem model. see [[cyb/fs/patch]] for batch operations over multiple [[particles]]