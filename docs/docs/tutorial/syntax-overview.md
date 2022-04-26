---
sidebar_position: 1
---
# MistQL Syntax Overview

MistQL’s syntax is simple by design. You should be able to fully understand MistQL’s syntax within a single day. 

## Initial Context
The simplest query in MistQL is `@`, which simply returns data back. `@` refers to the current context, which initially is set to _data_, making `@` the identity query in MistQL.

### Variables are populated from the context.
The keys of the current context `@` are populated in the namespace as variables. For example, if the data is `{ "foo": "bar" }`, the query `foo` returns `"bar"`, as the variable `foo` has been pushed to the namespace.

## Basic Types
MistQL has 8 data types total. 6 of these data types stem directly from their JSON counterparts:
* Object
* Array
* String
* Number
* Boolean
* Null

MistQL additionally has two types that aren’t representable in JSON, but are first-class members of the MistQL ecosystem regardless:
* Function
* Regex

## Basic Operators
MistQL supports basic arithmetic expressions. For example, the query `1 + x` operating over the data `{ "x": 5 }` returns the number `6`. Arithmetic expressions can get fairly complicated, involving parentheticals, modulo, etc. The expression `10.5 * (30 % 7) - 5 / 8`, for example, evaluates to `20.375`

MistQL also allows basic logical operations. The expression `true && false` outputs `false`, and `!false` outputs `true`. Finally, MistQL supports standard comparisons and equivalences: `50 > 10` outputs `false`, `10 > 50` outputs `false`, and `10 == 10` outputs `true`. For precise equality semantics, please refer to the Types section of the docs site. 

## Literals
JSON literals are all valid MistQL. For example the expression `{ "foo": 1, "bar": "baz" }` would return `{ "foo": 1, "bar": "baz" }`, regardless of whatever data was provided.

JSON literals can contain complicated subexpressions, for example the expression `[2 * 3, 5 * 10, { "food": "hot" +"dog"}]` evaluates to `[6, 50, {"food": "hotdog"}]`.

## Accessing Fields on Objects
There are 2 main ways of accessing a field on an object in MistQL, namely *Bracket Notation* and *Dot Notation*.

### Bracket Notation
Accessing fields on objects in MistQL is similar to JavaScript. To access fields on objects in MistQL, you use index notation with square brackets using strings. For example, if `@` is `{"hello": "world"}`, then the query `@["hello"]` returns the string `"world"`.

### Dot Notation
If the name of the key on an object is alphanumeric, you can use dot notation to access object keys as well. For example, given data of `{"foo": "bar"}`, you can validly rewrite `@["foo"]` as `@.foo`

Accessing fields on objects is _null coalescing by default_: if you access missing keys on an object `@.these.keys.dont.exist`, the expression will evaluate to `null` without erroring out. The
same is true for accessing fields on objects as well,

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

Identically, for arrays, we get the following:

```
[10, 20, 30, 40, 50][3:] == [40, 50]
[10, 20, 30, 40, 50][:2] == [10, 20]
[10, 20, 30, 40, 50][1:-1] == [10, 20]
```

## Calling Functions
Functions in MistQL are called in a bash-like syntax:

`functionname argument1 argument2 argument3`

For example, the following query sorts the provided array using the single-argument function `sort`:

`sort [1, 5, 3]` results in the array `[1, 3, 5]`

### Multi-argument functions 
Fucntions 

### Function Arguments can look quite complicated
Function arguments can be bare expressions. For example, take the following (fairly hard to read) expression:

`index 54 * 2 1 - 3 / 5 @`

In the above expression, `index` is taking 3 arguments:
* `54 * 2`
* `1 - 3 / 5`
* `@`
We can make this fact more clear by putting parentheses around each of the arguments. Putting explicit parentheses yields the following:

`index (54 * 2) (1 - 3 / 5) (@)`

## Piping
Piping provides an easy way to pass the result from one function along to another function in a clean, readable manner. When an expression is piped, the result of the expression on the left-hand side of the pipe is passed as the last argument to the function on the right-hand side of the pipe.

For example, these two queries behave identically:
` split ", " "dog, cat, walrus"`
`"dog, cat, walrus" | split ", "`

This allows users to chain together a sequence of functions in an easy-to-read manner:

`events | filter type == "purchase" | groupby email | keys`

MistQL's standard library is built around enabling piping as often as possible. If something can be expressed with pipes, it’s generally clearer to do so. 

### Contextualized Expressions
Contextualized expressions form the core of what differentiate MistQL from many other languages, and so should not be ignored.

When calculating the result of a function, MistQL often executes expressions multiple times under different contexts. For example, in the following code, the expression `@ % 2 == 0` is run multiple times: For each element in the array, filter appends the @ variable (along with any properties of that variable) to the stack, and executes the first expression.

`filter (@%2 == 0) [1, 2, 3, 4, 5, 6, 7]` results in `[2, 4, 6]`

The function calling the expression dictates how context is supplied to that expression, and how to handle the result of the expression. Functions that push to the context stack in this way are notated as `@: SomeContextType -> SomeReturnType` in the documentation.

You can think of contextualized expressions as a rough syntactic standin for where lambdas might arise in a similar language. 

Having contextualized expressions in this manner allows for some very clean-looking syntax. For example, filtering for all events of a given type and mapping to their email can be done with the following:

`events | filter type == "page_shown" | map email`

In the above query, we first filter for all events where event.type is page_shown, then map to the email field on each event in turn.

## The Root Variable `$`
[ todo: This section is sketched out a little bit, but not much. ]

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
