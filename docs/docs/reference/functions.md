---
sidebar_position: 4
---

# Functions

The following are all builtin functions in MistQL

### `apply`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `@: T -> K`      | `T`              | `K`         |

Takes the second argument and provides it as the context for the first argument

This is almost exclusively used as a utility as part of piping, e.g. `1 | apply @ + 1 | apply @ * 2`

#### Example

```
({a: "foo" } | apply a + "bar") == "foobar"
```

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

### `entries`

| Arity | Parameter Type | Return Type            |
| ----- | -------------- | ---------------------- |
| 1     | `object`       | `array<[string, any]>` |

Gets the entries of an object.

#### Example

Query:

```
{foo: "bar", baz: {boof: "woof"}} | entries
```

Result:

```
[["foo", "bar"] ["baz", {boof: "woof"}]]
```

### `filter`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `@: t -> any`    | `array<t>`       | `array<t>`  |

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

| Arity | Parameter 1 Type   | Parameter 2 Type | Return Type |
| ----- | ------------------ | ---------------- | ----------- |
| 2     | `@: string -> any` | `object`         | `object`    |

Filters a object's keys based on a condition

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

| Arity | Parameter 1 Type    | Parameter 2 Type | Return Type |
| ----- | ------------------- | ---------------- | ----------- |
| 2     | `@: unknown -> any` | `object`         | `object`    |

Filters a object's values based on a condition

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
| 2     | `@: t -> any`    | `array<t>`       | `t` or `null` |

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

### `flatten`

| Arity | Parameter 1 Type  | Return Type |
| ----- | ----------------- | ----------- |
| 1     | `array<array<t>>` | `array<t>`  |

Flattens a doubly nested array into a singly nested array.

#### Example

Query:

```
[
  [1, 2, 3],
  [4, 5],
  [6, 7]
] | flatten
```

Result:

```
[1, 2, 3, 4, 5, 6, 7]
```

### `float`

| Arity | Parameter Type | Return Type |
| ----- | -------------- | ----------- |
| 1     | `any`          | `number`    |

Casts the target value to a float

#### Example

```
(float "1.5") == 1.5
```

### `fromentries`

| Arity | Parameter Type         | Return Type |
| ----- | ---------------------- | ----------- |
| 1     | `array<[string, any]>` | `any`       |

Create an object from a list of entries. The inverse of `entries`.

#### Example

Query:

```
[["foo", "bar"] ["baz", {boof: "woof"}]] | fromentries
```

Result:

```
{foo: "bar", baz: {boof: "woof"}}
```

### `groupby`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `@: t -> any`    | `array<t>`       | `object`    |

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

| Arity | Parameter 1 Type               | Parameter 2 Type (optional) | Parameter 3 Type    | Return Type |
| ----- | ------------------------------ | --------------------------- | ------------------- | ----------- |
| 2-3   | `number` or `string` or `null` | `number` or `null`          | `array` or `object` | `unknown`   |

Performs the indexing operation, returning `null` if no such item exists. Bracket notation is syntactic sugar for calling the above function calls. Below are a number of indexing expressions and their equivalent `index` function calls.

More information can be found in the [indexing](indexing.md) reference document.

| Indexing Expression | Equivalent         |
| ------------------- | ------------------ |
| `arr[1]`            | `index 1 arr`      |
| `arr[1:3]`          | `index 1 3 arr`    |
| `arr[1:]`           | `index 1 null arr` |
| `arr[:1]`           | `index null 1 arr` |
| `obj["key"]`        | `index "key" obj`  |

Example:

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
| 1     | `object`         | `array<string>` |

Returns an array of all keys of a given object.

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

### `match`

| Arity | Parameter 1 Type    | Parameter 2 Type | Return Type |
| ----- | ------------------- | ---------------- | ----------- |
| 2     | `regex` or `string` | `string`         | `boolean`   |

Whether the given string matches the pattern.

If the pattern is a string instead of a regex, it is implicitly converted to a regex with no flags.

`match` returns true if the pattern matches anywhere within the target. For example, `match (regex "be") "abe"` returns true. To match the whole string, use default regex start and end characters, e.g. `match (regex "^be$") value`.

#### Example

Query:

```
match (regex "(ha)*") "hahahaha"
```

Result:

```
true
```

### `map`

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `@: t -> k`      | `array<t>`       | `array<k>`  |

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

| Arity | Parameter 1 Type   | Parameter 2 Type | Return Type |
| ----- | ------------------ | ---------------- | ----------- |
| 2     | `@: string -> any` | `object`         | `object`    |

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

| Arity | Parameter 1 Type    | Parameter 2 Type | Return Type |
| ----- | ------------------- | ---------------- | ----------- |
| 2     | `@: unknown -> any` | `object`         | `object`    |

Runs an expression on every value of a object.

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

| Arity | Parameter 1 Type           | Parameter 2 Type | Parameter 3 Type | Return Type |
| ----- | -------------------------- | ---------------- | ---------------- | ----------- |
| 3     | `@: [acc: b, cur: a] -> b` | `b`              | `array<a>`       | `b`         |

Runs a `reduce` operation on every value of an array

#### Example

Query:

```
[1, 2, 3] | reduce @[0] + @[1] 0
```

Result:

```
6
```

### `range`

| Arity | Parameter 1 Type | Parameter 2 Type (optional) | Parameter 3 Type (optional) | Return Type |
| --- | ---| --- | --- | --- |
| 1 - 3 | `number` | `number` | `number` | `array<number>` |

Takes arguments `start`, `stop`, and `step`. Returns a list of integers starting at `start`, ending at `stop` (non-inclusive), using steps of size `step`. If the sign of `step` does not match the sign of `stop - start`, then an empty list is returned.

Behavior of `range` closely matches python's `range` function.

Below is a table describing how the variables `start`, `stop`, and `step` are constructed at different arities. This corresponds to Python's `range()` method.

| Variable | Arity=1 | Arity=2 | Arity=3 |
| --- | --- | --- | --- |
| `stop` | First Arg | Second Arg | Second Arg |
| `start` | `0` | First Arg | First Arg |
| `step` | `1` | `1` | Third Arg |

Numbers provided to range must all be integers. Step must be greater than zero.

#### Example

`range 5` yields `[0, 1, 2, 3, 4]`

`range 3 8` yields `[3, 4, 5, 6, 7]`

`range 3 8 2` yields `[3, 5, 7]`

### `regex`

| Arity | Parameter 1 Type | Parameter 2 Type (optional) | Return Type |
| ----- | ---------------- | --------------------------- | ----------- |
| 1 - 2 | `string`         | `string`                    | `regex`     |

Constructs a regular expression based on the pattern. The second, optional, argument specifies the flags.

#### Example

`regex "(h.)*" "g"` constructs the regular expression `/(h.)*/g`

### `replace`

| Arity | Parameter 1 Type    | Parameter 2 Type | Parameter 3 Type | Return Type |
| ----- | ------------------- | ---------------- | ---------------- | ----------- |
| 3     | `string` or `regex` | `string`         | `string`         | `string`    |

Replaces the first matching instance of parameter 1 with parameter 2 in parameter 3. If a global regex is used, all instances are replaced.

#### Example

Query:

```
"haha" | replace (regex "h." "g") "z"
```

Result:

```
"zz"
```

### `reverse`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array<t>`       | `array<t>`  |

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

| Arity | Parameter `n` type | Last Parameter Type | Return Type       |
| ----- | ------------------ | ------------------- | ----------------- |
| >3    | `@: t -> boolean`  | `array<t>`          | `array<array<t>>` |

Computes the set of all possible subsequences of the target array that
satisfy the specified conditions.

#### Example

Query:

```
[1, 2, 3, 4] | sequence (@ % 2) ((@ + 1)% 2)
```

Result:

```
[[1, 2], [1, 4], [3, 4]]
```

### `sort`

| Arity | Parameter 1 Type | Return Type |
| ----- | ---------------- | ----------- |
| 1     | `array<t>`       | `array<t>`  |

Sorts an array into ascending order. Strings are sorted alphabetically. Numbers are sorted numerically.

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

| Arity | Parameter 1 Type | Parameter 2 Type | Return Type |
| ----- | ---------------- | ---------------- | ----------- |
| 2     | `@: t -> any`    | `array<t>`       | `array<t>`  |

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

### `split`

| Arity | Parameter 1 Type    | Parameter 2 Type | Return Type     |
| ----- | ------------------- | ---------------- | --------------- |
| 2     | `string` or `regex` | `string`         | `array<string>` |

Splits a string based on a delimiter. Unicode codepoints are not split.

#### Example

Query:

```
"foo,bar,baz" | split ","
```

Result:

```
["foo", "bar", "baz"]
```

### `string`

| Arity | Parameter Type | Return Type |
| ----- | -------------- | ----------- |
| 1     | `any`          | `string`    |

Casts the target value to a string

#### Example

```
(string 1.5) == "1.5"
```


### `stringjoin`

| Arity | Parameter 1 | Parameter 2 Type | Return Type |
| ----- | ----------- | ---------------- | ----------- |
| 2     | `string`    | `array<string>`  | `string`   |

Joins an array of strings together with a given delimiter.

#### Example

```
["foo", "bar", "baz"] | stringjoin ","
```

Result:

```
"foo,bar,baz"
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
| 1     | `array<number>`  | `object`    |

Gives a object containing a statistical summary of an array of numbers

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
| 1     | `object`         | `array`     |

Returns an array of all values of a given object.

Query:

```
{bleep: "bloop", zap: [4, 5, 6]} | values
```

Result:

```
["bloop", [4, 5, 6]]
```

### `withindices`

| Arity | Parameter 1 Type | Return Type          |
| ----- | ---------------- | -------------------- |
| 1     | `array<T>`       | `array<[number, T]>` |

Maps an array to a list of tuple-like arrays with the index as the first member of said tuple, and the original value as the second member.

Query:

```
["a", "b", "c", "d"] | withindices
```

Result:

```
[
  [0, "a"],
  [1, "b"],
  [2, "c"], 
  [3, "d"]
]
```
