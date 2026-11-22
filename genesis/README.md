# genesis

How block 0 of cyber is made, in four files.

| file | what it holds |
|---|---|
| [sources.toml](sources.toml) | the input: the proven states of pussy and bostrom, by chain id |
| [distribution.toml](distribution.toml) | who holds what: nobody at block 0; every TOCYB of bostrom claims 666 CYB |
| [graph.toml](graph.toml) | the starting graph: empty |
| [genesis.lock](genesis.lock) | the result, by digest: not built yet |

cyber differs from its two elder networks in one thing: it buries no chain. pussy and bostrom carry the balances and the cyberlinks of the chains they are reborn from. cyber starts empty and reads them: a holder of TOCYB proves the balance in the state of bostrom and mints CYB here.

## what is missing before it can be built

- the claim program: the relation that checks a balance against a proven state of another network (soft3 [present proves the past](https://github.com/cyberia-to/soft3/blob/release/phase1/projects/present-proves-the-past.md));
- the node reading a network from its repository (soft3 [networks as repositories](https://github.com/cyberia-to/soft3/blob/release/phase1/projects/network-repos.md), phase N4);
- the two networks themselves, launched, with final chain ids.

The standard of these files: cybernode [`specs/network.md`](https://github.com/cyberia-to/cybernode/blob/main/specs/network.md).
