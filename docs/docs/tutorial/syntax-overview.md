---
sidebar_position: 1
---

# Syntax Overview

MistQL's syntax takes inspiration from bash, SQL, JavaScript, Haskell, `jq`, and others.

## Calling functions

Functions are called in a bash-like syntax: 

`function argument1 argument2 argument3`

For example, counting 

## Operators

Operators are functions that are called via a specialized syntax. 

For example, the following adds 3 numbers together via the `+` binary operator.

```
1 + 2 + 3 == 6
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


