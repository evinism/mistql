---
sidebar_position: 1
---

# Syntax Overview

MistQL's syntax takes inspiration from bash, SQL, JavaScript, Haskell, `jq`, and others.

## Inputs

Keys of the data input are resolved as variables.

```js
mistql.query("sandwich", {sandwich: "blt"}) === "blt";
mistql.query("skate.num_of_letters + 1", {skate: {num_of_letters: 3}}) === 4;
```

## Calling functions

Functions are called in a bash-like syntax: 

```
function argument1 argument2 argument3
```

For example, getting the length of an array might look like:

```
count [1, 2, 3]
```

## Piping

Piping allows functions to easily pass the result of one expression as the last parameter of another.

For example, the following are equivalent:
```
animals | filter type=="cat"
filter type=="cat" animals
```

This allows users to chain together a sequence of functions in an easy-to-read manner:

```
events | filter type=="purchase" | groupby email | keys
```

MistQL's standard library is built around enabling piping as often as possible.




## Literals

MistQL supports all JSON literals. JSON is valid MistQL.

```
{
  "name": "Jamie",
  "pronouns": ["they", "them"],
  "age": 42,
  "subscriptionDate": null, 
  "isRadAsHeck": true,
}
```

Arrays and structs can contain other expressions:

```
[
  -1 - 500,
  { isSpot: dog == "spot"},
  (events | filter type=="purchase" | count)
]
```

## Operators

Operators are functions that are called via a specialized syntax. 

For example, the following adds 3 numbers together via the `+` binary operator.

```
1 + 2 + 3 == 6
```

## The `@` symbol

The `@` symbol refers to the current context variable. At the top level, the context variable is set to the data input.

```js
mistql.query("@", [1, 2, 3]); // evaluates to [1, 2, 3]
```

Certain functions change the context variable, depending on usage.

```js
mistql.query("@ | map @ + 1 ", [1, 2, 3]); // evaluates to [2, 3, 4]
```

Context changes are explored more fully in the next portion of the tutorial.
