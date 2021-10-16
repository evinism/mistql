---
sidebar_position: 4
---

# Functions

The following are all builtin functions in MistQL

### `count`

| Arity | Parameter Type | Return Type |
| ----- | -------------- | ----------- |
| 1     | `array`        | `number`    |

Returns the length of an array.

#### Example

The following counts the number of occurrences of the number `3` in the array

```
[1, 2, 3, 2, 2, 3] | filter @ == 3 | count
```

### `filter`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `any`            | `array<t>`       | `array<t>`  |

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

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `any`            | `struct`         | `struct`    |

Filters a struct's keys based on a condition

#### Example

Query:

```
{apple: "good", banana: "very good", carrot: "excellent"}
| filterkeys @ > "apricot"
```

Result:

```
{ banana: "very good", carrot: "excellent" }
```

### `filtervalues`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `any`            | `struct`         | `struct`    |

Filters a struct's values based on a condition

#### Example

Query:

```
{
  apple: {score: 5},
  banana: {score: 4},
  carrot: {score: 7},
  donut: {score: 10}
} | filterkeys score >= 7
```

Result:

```
{
  carrot: {score: 7},
  donut: {score: 10}
}
```

### `find`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type   |
| ----- | ---------------- | ---------------- | ------------- |
| 2     | `any`            | `array<t>`       | `t` or `null` |

Finds the first item that matches the condition

#### Example

Query:

```
[
  { fruit: "apple", score: 5},
  { fruit: "banana", score: 4},
  { fruit: "carrot", score: 7},
  { fruit: "donut", score: 10}
 ] | find (score % 2) == 0
```

Result:

```
{ fruit: "banana", score: 4},
```

### `groupby`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `any`            | `array`          | `struct`    |

Groups the items in the array based on some condition

#### Example

Query:

```
[
  { gender: "female", name: "hayley" },
  { gender: "female", name: "emily" },
  { gender: "male", name: "abhik" },
  { gender: "female", name: "carmen" },
  { gender: "male", name: "ori" },
  { gender: "male", name: "adrian" },
  { gender: "nonbinary", name: "ash" },
  { gender: "nonbinary", name: "remy" },
 ] | groupby gender
```

Result:

```
{
  "female": [
    { "gender": "female", "name": "hayley"},
    {"gender": "female", "name": "emily"},
    {"gender": "female", "name": "carmen"}
  ],
  "male": [
    {"gender": "male", "name": "abhik"},
    {"gender": "male", "name": "ori"},
    {"gender": "male", "name": "adrian"}
  ],
  "nonbinary": [
    {"gender": "nonbinary", "name": "ash"},
    { gender: "nonbinary", name: "remy" }
  ]
}
```

### `if`

| Arity | Parameter 1 Type | Parameter 2 Type | Parameter 3 Type | Return Type |
| ----- | ---------------- | ---------------- | ---------------- | ----------- |
| 3     | `any`            | `t`              | `k`              | `t` or `k`  |

If the condition is true, returns the second argument. Otherwise returns the third argument.

```
if 1 > 2 "foo" "bar"
```

Result:

```
"bar"
```

### `index`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type   |
| ----- | ---------------- | ---------------- | ------------- |
| 2     | `number`         | `array<t>`       | `t` or `null` |

Returns the nth item in an array, or null if no such item exists.

```
[1, 2, 3] | index 1
```

Result:

```
2
```

### `keys`

| Arity | Parameter 1 Type | Return Type     |
| ----- | ---------------- | --------------- |
| 1     | `struct`         | `array<string>` |

Returns an array of all keys of a given struct.

```
{bleep: "bloop", zap: [4, 5, 6]} | keys
```

Result:

```
["bleep", "zap"]
```

### `log`

| Arity | Parameter Type | Return Type |
| ----- | -------------- | ----------- |
| 1     | `t`            | `t`         |

Logs the value to the console, and passes it without modification. This is used for debugging.

#### Example

Query:

```
log ["haha", "blah", "cat"]
```

Result:

```
["haha", "blah", "cat"]
```

Additionally, `MistQL Log: ["haha", "blah", "cat"]` is written to the console.

### `map`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `k`              | `array<t>`       | `array<k>`  |

Runs an expression on every element of an array.

#### Example

Query:

```
[
  {animal: "cat", name: "Millie"},
  {animal: "dog", name: "Alfred"},
  {animal: "cat", name: "Mulberry"},
] | map name + " the " + animal
```

Result:

```
[
  "Millie the cat",
  "Alfred the dog",
  "Mulberry the cat"
]
```

### `mapkeys`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `any`            | `struct`         | `struct`    |

Maps every key in an expression.

#### Example

Query:

```
{
  abhik: true,
  evin: false,
 } | mapkeys  @ + "@example.com"
```

Result:

```
{
  "abhik@example.com": true,
  "evin@example.com": false,
}
```

### `mapvalues`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `any`            | `struct`         | `struct`    |

Runs an expression on every value of a struct.

#### Example

Query:

```
{
  bestInShow: {animal: "cat", name: "Millie"},
  bestBehaved: {animal: "dog", name: "Alfred"},
  coolest: {animal: "cat", name: "Mulberry"},
 } | mapvalues name + " the " + animal
```

Result:

```
{
  bestInShow: "Millie the cat",
  bestBehaved: "Alfred the dog",
  coolest: "Mulberry the cat"
}
```

### `reduce`

| Arity | Parameter 1 Type | Parameter 2 Type | Parameter 3 Type | Return Type |
| ----- | ---------------- | ---------------- | ---------------- | ----------- |
| 3     | `any`            | `b`              | `array<a>`       | `b`         |

Runs a `reduce` operation on every value of an array

#### Example

Query:

```
[1, 2, 3] | reduce (first @) + (last @) 0
```

Result:

```
6
```

### `reverse`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array`          | `array`     |

Reverses an array

#### Example

Query:

```
[1, 2, 3] | reverse
```

Result:

```
[3, 2, 1]
```

### `sequence`

TODO: Explain Sequence

### `sort`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array`          | `array`     |

Sorts an array into ascending order.

#### Example

Query:

```
[3, 1, 2] | sort
```

Result:

```
[1, 2, 3]
```

### `sortby`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array`          | `array`     |

Sorts an array into ascending order by some expression

#### Example

Query:

```
[3, 1, 2, 8] | sortby @ % 4
```

Result:

```
[8, 1, 2, 3]
```

### `sum`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array<number>`  | `number`    |

Adds all numbers in an array together

#### Example

Query:

```
[1, 2, 3, 4, 5, 6] | sum
```

Result:

```
21
```

### `summarize`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array<number>`  | `struct`    |

Gives a struct containing a statistical summary of an array of numbers

#### Example

Query:

```
[1, 2, 3, 4, 5, 6] | summarize
```

Result:

```
{
  "max": 6,
  "min": 1,
  "mean": 3.5,
  "median": 3.5,
  "variance": 2.9166666666666665,
  "stddev": 1.707825127659933
}
```

### `values`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `struct`         | `array`     |

Returns an array of all values of a given struct.

Query:

```
{bleep: "bloop", zap: [4, 5, 6]} | values
```

Result:

```
["bloop", [4, 5, 6]]
```
