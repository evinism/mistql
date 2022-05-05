---
sidebar_position: 1
---
# Syntax Overview

While MistQL's syntactic footprint is kept relatively small, it does take a little time to get used to. 

## Initial Context
The simplest query in MistQL is `@`, which simply returns the _data_ that was passed into MistQL. `@` refers to the _current context_, which initially is set to _data_, making `@` the identity query in MistQL. As an example, we could have the following:

| Query | Data | Result |
|---|---|---|
| `@` | `["arbitrary", "data"]` |  ` ["arbitrary", "data"]` |

In the above, we operate the `@` query over the data `["arbitrary", "data"]`, returning the exact same data structure we passed in directly as output.


#### Context variable population
The keys of the current context `@` are populated in the namespace as variables, such that they can be accessed via bare names:

| Query | Data | Result |
|---|---|---|
| `foo` | `{"foo": "bar"}` |  `"bar"` |

In the above, the data has a `foo` key, which gets populated as variable `foo` in the above MistQL query.

## Basic Operators
MistQL supports basic numbers and arithmetic expressions, following standard algebraic notation. For example, consider the following queries.

| Example | Expression | Result |
|---|---|---|
| Simple  | `1 + 5` | `6` |
| Complicated | `10.5 * (30 % 7) - 5 / 8` | `20.375` |

These execute using PEMDAS exactly as expected.

MistQL also allows basic logical operations, using the `||`, `&&`, and `!` operators, as well as the `true` and `false` literals.

| Expression | Result |
|---|---|
| `true ││ false` | `true` |
| `true && false` | `false` |
| `!false` | `true` |

MistQL allows short-circuiting:

| Expression | Result |
|---|---|
| `0 ││ "foo"` | `"foo"` |
| `[1, 2, 3] && null` | `null` |

Finally, MistQL supports standard comparisons and equivalences: `50 > 10` outputs `false`, `10 > 50` outputs `false`, and `10 == 10` outputs `true`. 

| Expression | Result |
|---|---|
| `50 > 10` | `true` |
| `50 < 10` | `false` |
| `10 == 10` | `true` |

MistQL equality always returns `false` for disparate types, e.g. if comparing two different types, equality of the values will always result in `false`. For precise equality semantics, please refer to the Types section of the reference docs.

## Literals
JSON literals are all valid MistQL:

```
{ "foo": 1, "bar": "baz" }
{ "zeep": [1, "zoop", false], "null": null }
```

This will hold true for all JSON values regardless of whatever data was provided, becuase MistQL expressions are a strict superset of JSON.

Similarly to JavaScript, you can omit the quotes around key names that are valid identifiers in object literals:

```
{"type": "cat"} == {type: "cat"}
```

JSON literals can contain complicated subexpressions, for example: 

```
[2 * 3, 5 * 10, { food: "hot" +"dog"}] == [6, 50, {food: "hotdog"}]
```

## Accessing Fields on Objects
There are 2 main ways of accessing a field on an object in MistQL, namely *Bracket Notation* and *Dot Notation*.

### Bracket Notation
Accessing fields on objects in MistQL is similar to JavaScript. To access fields on objects in MistQL, you use index notation with square brackets using strings:

| Query | Data | Result |
|---|---|---|
| `@["hello"]` | `{"hello": "world"}` | `"world"` |
| `foo["bar"]["baz"]` | `{"foo": {"bar": { "baz": true}}}` | `true` |

### Dot Notation
If the name of the key on an object is alphanumeric, you can use dot notation to access object keys as well. For example, given data of `{"foo": "bar"}`, you can validly rewrite `@["foo"]` as `@.foo`.

| Query | Data | Result |
|---|---|---|
| `@["foo"]` | `{"foo": "bar"}` | `"bar"` |
| `@.foo` | `{"foo": "bar"}` | `"bar"` |

Nested object can be accessed as a series of dots:

| Query | Data | Result |
|---|---|---|
| `@.foo.bar.baz` | `{"foo": {"bar": {"baz": 5}}}` | `5` |

Accessing fields on objects is _null coalescing by default_: if you access missing keys on an object `@.these.keys.dont.exist`, the expression will evaluate to `null` without erroring out.

## Accessing strings and arrays

Strings and arrays are accessed exclusively by *bracket notation* using zero-indexed numbers:

```
"hello"[0] == "h"
"hello"[4] == "o"
[100, 200, 300, 400][0] == 100
[100, 200, 300, 400][3] == 400
```

Using negative numbers allows indexing from the end of a string or array:

```
"world"[-1] == "d"
"world"[-2] == "l"
[100, 200, 300, 400][0] == 100
[100, 200, 300, 400][3] == 400
```

Additionally, you can use the `:` separator in a pythonic style to allow for selecting of ranges:

```
"china"[1:] == "hina"
"spain"[:-2] == "spa"
"portugal"[1:4] == "ort"
```

Similarly, we get the following for arrays:

```
[10, 20, 30, 40, 50][3:] == [40, 50]
[10, 20, 30, 40, 50][:2] == [10, 20]
[10, 20, 30, 40, 50][1:-1] == [20, 30, 40]
```

## Calling Functions
Functions in MistQL are called in a lisp-like syntax, with function name and arguments separated by whitespace:

```
functionname argument1 argument2 argument3
```

For example, the following query sorts the provided array using the single-argument function `sort`:

`sort [1, 5, 3]` results in the array `[1, 3, 5]`

Multi-argument functions follow the same pattern. For example, the following calls the `index` function using 3 arguments, namely `1`, `3`, and `"foobar"`:

```
index 1 3 "foobar"
```

#### Caveat: Function arguments can look quite complicated
Function arguments can be bare expressions. For example, take the following (fairly hard to read) expression:

```
index 54 * 2 1 - 3 / 5 @
```

In the above expression, `index` is taking 3 arguments:
* `54 * 2`
* `1 - 3 / 5`
* `@`
We can make this fact more clear by putting parentheses around each of the arguments. Putting explicit parentheses yields the following:

```
index (54 * 2) (1 - 3 / 5) (@)
```

## Piping
Piping provides an easy way to pass the result from one function along to another function in a clean, readable manner. When an expression is piped, the result of the expression on the left-hand side of the pipe is passed as the last argument to the function on the right-hand side of the pipe.

For example, these two queries behave identically:

```
split "," "dog,cat,walrus"
"dog,cat,walrus" | split ","
```

This allows users to chain together a sequence of functions in an easy-to-read manner:

```
events | filter type == "purchase" | groupby email | keys
```

MistQL's standard library is built around enabling piping as often as possible. If something can be expressed with pipes, it’s generally clearer to do so.

#### The `apply` idiom

You may be tempted to write the following to multiply 10 and 2:

```
10 | @ * 2         WRONG
```
This will error out, saying that the result of @ + 2 is not a function -- and it's right! Since piping relies on being able to pass functions in without arguments, allowing the above syntax would form an ambiguity:



```
10 | @ * 2         WRONG
10 | apply @ * 2   RIGHT
```

## The Root Variable `$`

The root variable `$` is an object containing all of the following:
* A list of all builtin functions in MistQL
* A reference to the root context variable (denoted `@`)

### Method overwriting
In many cases, there can be naming conflicts 

Example: `{ "map": [] }`. What does query `map` resolve to?

Answer: it resolves to [], as the `map` function is overwritten.

In order to get back to the root, we have to reference an un-overwritable variable, namely the $ variable. 
Root Context
In several cases, context can be overwritten in such a way that you need a convenient way 
