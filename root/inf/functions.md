---
tags: cyber
crystal-type: entity
crystal-domain: cyber
---
built-in function reference for [[datalog]] (CozoScript). all functions are available in rule bodies, expressions, and aggregation contexts

## math

| function | description |
|---|---|
| `add` / `+` | addition |
| `sub` / `-` | subtraction |
| `mul` / `*` | multiplication |
| `div` / `/` | division |
| `mod` / `%` | modulo |
| `pow` / `^` | exponentiation |
| `minus` | unary negation |
| `abs` | absolute value |
| `signum` | sign (-1, 0, 1) |
| `sqrt` | square root |
| `floor`, `ceil`, `round` | rounding |
| `exp`, `exp2` | exponential |
| `ln`, `log2`, `log10` | logarithmic |
| `sin`, `cos`, `tan` | trigonometric |
| `asin`, `acos`, `atan`, `atan2` | inverse trigonometric |
| `sinh`, `cosh`, `tanh` | hyperbolic |
| `asinh`, `acosh`, `atanh` | inverse hyperbolic |
| `deg_to_rad`, `rad_to_deg` | angle conversion |
| `haversine`, `haversine_deg_input` | geographic distance |

## comparison and boolean

| function | description |
|---|---|
| `eq` / `==` | equality |
| `neq` / `!=` | inequality |
| `gt` / `>`, `ge` / `>=` | greater than / or equal |
| `lt` / `<`, `le` / `<=` | less than / or equal |
| `max`, `min` | extrema |
| `and` / `&&` | logical conjunction |
| `or` / `\|\|` | logical disjunction |
| `negate` / `!` | logical negation |
| `assert` | returns true or raises error |

## string

| function | description |
|---|---|
| `length` | character count |
| `concat` / `++` | concatenation |
| `str_includes` | substring check |
| `lowercase`, `uppercase` | case conversion |
| `trim`, `trim_start`, `trim_end` | whitespace removal |
| `starts_with`, `ends_with` | prefix/suffix check |
| `unicode_normalize` | normalization |
| `chars` | split into character list |
| `from_substrings` | join from list |

## list

| function | description |
|---|---|
| `list` | construct list |
| `is_in` | membership test |
| `first`, `last` | endpoint access |
| `get`, `maybe_get` / `->` | indexed access |
| `length` | list size |
| `slice` | subsequence |
| `concat` / `++` | concatenation |
| `prepend`, `append` | add element |
| `reverse` | reverse order |
| `sorted` | sort |
| `chunks`, `chunks_exact` | partition |
| `windows` | sliding window |
| `union`, `intersection`, `difference` | set operations on lists |

## vector

relevant for embedding-based queries over [[particles]]

| function | description |
|---|---|
| `vec` | construct vector from numbers |
| `rand_vec` | random vector generation |
| `l2_normalize` | L2 normalization |
| `l2_dist` | Euclidean distance |
| `ip_dist` | inner product distance |
| `cos_dist` | cosine distance |

```datalog
// find particles with embeddings similar to a query vector
?[particle, distance] := *particle_embeddings{particle, embedding},
                         distance = cos_dist(embedding, vec([0.1, 0.5, 0.3, ...]))
:sort distance
:limit 10
```

## JSON

| function | description |
|---|---|
| `json` | convert to JSON |
| `is_json` | type check |
| `json_object` | create object |
| `dump_json` | serialize to string |
| `parse_json` | parse from string |
| `get`, `maybe_get` / `->` | access element |
| `set_json_path` | modify value |
| `remove_json_path` | delete element |
| `json_to_scalar` | convert to scalar |
| `concat` / `++` | deep merge |

## regex

| function | description |
|---|---|
| `regex_matches` | pattern match test |
| `regex_replace` | replace first match |
| `regex_replace_all` | replace all |
| `regex_extract` | extract all matches |
| `regex_extract_first` | extract first match |

## timestamp

| function | description |
|---|---|
| `now` | current timestamp |
| `format_timestamp` | to RFC3339 string |
| `parse_timestamp` | from RFC3339 string |
| `validity` | create validity object for time-travel |

## type checking and conversion

| function | description |
|---|---|
| `coalesce` / `~` | first non-null value |
| `to_string`, `to_float`, `to_int` | type conversion |
| `to_unity`, `to_bool`, `to_uuid` | specialized conversion |
| `uuid_timestamp` | extract UUID timestamp |
| `is_null`, `is_int`, `is_float`, `is_num` | type checks |
| `is_finite`, `is_infinite`, `is_nan` | number checks |
| `is_bytes`, `is_list`, `is_string`, `is_uuid`, `is_json` | type checks |

## random

| function | description |
|---|---|
| `rand_float` | random float in [0,1] |
| `rand_bernoulli` | random boolean with probability |
| `rand_int` | random integer in range |
| `rand_choose` | random element from list |
| `rand_uuid_v1`, `rand_uuid_v4` | UUID generation |
| `rand_vec` | random vector |

## aggregation operators

used in rule heads for grouping and reduction

| operator | description | semi-lattice |
|---|---|---|
| `count` | row count | no |
| `sum` | sum values | no |
| `min` | minimum | yes |
| `max` | maximum | yes |
| `mean` | arithmetic mean | no |
| `collect` | gather into list | no |
| `choice` | pick arbitrary value | yes |
| `min_cost` | minimum with associated cost | yes |
| `shortest` | shortest path collector | yes |
| `bit_and`, `bit_or` | bitwise aggregation | yes |
| `union_is_in` | set union | yes |

semi-lattice aggregation allows self-recursion — required for recursive shortest-path and reachability queries over the [[cybergraph]]

see [[inf/queries]] for how aggregation works in rule heads. see [[inf/algorithms]] for fixed-rule graph algorithms