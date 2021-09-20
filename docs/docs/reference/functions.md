---
sidebar_position: 4
---

# Functions

The following are all builtin functions in MistQL

### `count`
| Arity | Parameter Type | Return Type |
|---|---|---|
| 1 | `array` | `number` |

Returns the length of an array.

#### Example
The following counts the number of occurrences of the number `3` in the array

```
[1, 2, 3, 2, 2, 3] | filter @ == 3 | count
```

### `filter`
| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
|---|---|---|---|
| 2 | `any` | `array` | `array` |

Filters an array based on a condition.

#### Example
Query: 
```
[
  {animal: "cat", name: "Millie"},
  {animal: "dog", name: "Alfred"},
  {animal: "cat", name: "Mulberry"},
] | filter animal == "cat"
```

Result:
```
[
  {animal: "cat", name: "Millie"},
  {animal: "cat", name: "Mulberry"},
]
```

### `filterkeys`

### `filtervalues`

### `find`

### `first`

### `groupby`

### `head`

### `if`

### `index`

### `keys`

### `last`

### `log`

### `map`

### `mapkeys`

### `mapvalues`

### `reduce`

### `reverse`

### `sequence`

### `sort`

### `sortby`

### `sum`

### `summarize`

### `tail`

### `values`
