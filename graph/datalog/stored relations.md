---
tags: cyber
crystal-type: entity
crystal-domain: cyber
alias: stored relation
stake: 40275280678849256
---
stored relations are how data persists in [[datalog]]. where inline rules exist only during query execution, stored relations survive across sessions — they are the permanent memory of the [[cybergraph]]

every stored relation has a schema that defines its columns, types, and key structure. mutations write data into stored relations. [[transactions]] group mutations atomically. together these form the data layer beneath all [[datalog]] queries

## schema definition

a stored relation is defined with `:create` or `:replace`, specifying columns separated into keys and values by the `=>` marker

```datalog
:create particles { cid: String => content_type: String, size: Int, created: Validity }
```

columns before `=>` are keys. columns after `=>` are values. keys determine the sort order and enforce uniqueness — no two rows can share the same key combination. if every column is a key (no `=>`), the relation is a set of tuples with no associated values

```datalog
:create tags { cid: String, tag: String }
```

## column types

| type | description |
|---|---|
| String | UTF-8 text |
| Int | 64-bit signed integer |
| Float | 64-bit floating point |
| Bool | true or false |
| Null | the null value |
| Bytes | raw byte array |
| List | heterogeneous list |
| Json | arbitrary JSON value |
| Validity | transaction-aware timestamp for time-travel queries |
| Vec | fixed-length float vector for HNSW indices |

omitting the type annotation makes the column accept any type. this is useful for flexible schemas but loses the safety of type checking

## default values

columns can have defaults, applied when a mutation omits that column

```datalog
:create neurons {
    address: String
    =>
    stake: Int default 0,
    karma: Float default 0.0,
    active: Bool default true
}
```

## explicit binding mapping

when query variable names differ from column names, map them explicitly

```datalog
?[a, b, c] <- [["cosmos1abc", 1000, 0.5]]
:put neurons { address = a, stake = b, karma = c }
```

this decouples the query namespace from the relation schema

## mutation operations

| operation | behavior |
|---|---|
| `:create` | create a new relation with schema; error if it already exists |
| `:replace` | create or overwrite a relation; schema changes are allowed |
| `:put` | upsert rows — insert if key is new, update if key exists |
| `:insert` | insert rows — error if any key already exists |
| `:update` | modify specific columns — provide keys and only the changed values |
| `:rm` | remove rows by key — no error if key is missing |
| `:delete` | remove rows by key — error if any key is missing |
| `:ensure` | assert rows exist with given values — error on mismatch (read-write consistency) |
| `:ensure_not` | assert rows do not exist — error if any key is found (read-write consistency) |

`:put` is the workhorse for most writes. `:insert` and `:delete` are strict variants that enforce expectations. `:ensure` and `:ensure_not` enable optimistic concurrency — the transaction aborts if reality diverges from assumption

```datalog
?[address, stake] <- [["bostrom1abc", 5000]]
:put neurons { address, stake }
```

```datalog
?[address] <- [["bostrom1abc"]]
:rm neurons { address }
```

## transaction chaining

multiple queries wrapped in `{ }` braces execute as a single atomic transaction. all succeed or all fail

```datalog
{
    ?[cid, content_type, size, created] <- [["Qm123", "text/plain", 256, "2024-01-15T00:00:00"]]
    :put particles { cid, content_type, size, created }

    ?[neuron, from_cid, to_cid, weight, timestamp] <- [["bostrom1abc", "Qm123", "Qm456", 1.0, "2024-01-15T00:00:00"]]
    :put cyberlinks { neuron, from_cid, to_cid, weight, timestamp }
}
```

this guarantees that a [[particle]] and its [[cyberlink]] are stored together — no partial writes

## ephemeral relations

relations prefixed with underscore (`_`) are ephemeral — they exist only within the current transaction and vanish afterward

```datalog
{
    ?[cid, score] := *focus{particle: cid, score}, score > 0.5
    :replace _high_focus { cid: String, score: Float }

    ?[cid, score] := *_high_focus{cid, score}
    :put spotlight { cid, score }
}
```

ephemeral relations pass intermediate results between transaction steps without polluting persistent storage

## control flow

CozoScript supports control flow directives within transaction blocks

```datalog
{
    ?[count] := count = count(*cyberlinks{})
    %if count > 1000000
        %then ?[msg] <- [["graph is large"]]
        %else ?[msg] <- [["graph is small"]]
    %end
}
```

`%loop` / `%break` / `%continue` / `%end` enable iteration within transactions. `%return` exits the transaction block early, returning the current query result

## :returning option

append `:returning` to a mutation to get back the affected rows with a `_kind` field indicating the operation performed

```datalog
?[address, stake] <- [["bostrom1abc", 5000], ["bostrom1def", 3000]]
:put neurons { address, stake }
:returning
```

the result includes `_kind` values: `"inserted"`, `"updated"`, or `"removed"` — useful for logging, debugging, and reactive pipelines

## cybergraph schema

the core [[cybergraph]] can be modeled with four stored relations

```datalog
:create particles { cid: String => content_type: String, size: Int, created: Validity }
:create cyberlinks { neuron: String, from_cid: String, to_cid: String => weight: Float, timestamp: Validity }
:create neurons { address: String => stake: Int, karma: Float }
:create focus { particle: String => score: Float }
```

[[particles]] are content-addressed objects identified by CID. [[cyberlinks]] are directed weighted edges created by [[neurons]]. each [[neuron]] carries [[stake]] and [[karma]]. [[focus]] is a derived score computed by [[cyberank]]

querying across these relations composes naturally

```datalog
?[particle, score, neuron_karma] :=
    *cyberlinks{neuron, to: particle},
    *focus{particle, score},
    *neurons{address: neuron, karma: neuron_karma},
    score > 0.1
:sort -score
:limit 50
```

this retrieves the top 50 [[particles]] by [[focus]] score, joined with the [[karma]] of the [[neuron]] that linked them — a single declarative query across the entire graph state

## relation to the stack

stored relations are the persistence layer. [[rune]] writes into them via mutations. [[datalog/queries]] read from them via pattern matching. [[datalog/algorithms]] operate over them as graph structures. time-travel queries (using [[Validity]] columns) reconstruct any past state of the [[cybergraph]]
