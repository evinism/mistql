---
sidebar_position: 3
---

# Gotchas

These are the nasty parts of MistQL, documented here for posterity's sake.

## Gotcha 1: Unary Minus

You may be tempted to write:

```
items | map -cost
```

Unfortunately, the expression is parsed as `map - cost` rather than `map (-cost)`. This is due to the ambiguity
between the minus sign acting as a unary operator vs. a binary operator.

To get around this, simply surround the unary minus operator and operand with parentheses:

```
items | map (-cost)
```

This only affects the minus sign because it's the only token that serves as both a unary and a binary operator.

## Gotcha 2: Indexing expressions and whitespace

Indexing expressions must follow whatever they're indexing directly, with no spaces. This is due to the potential ambiguity between an indexing expression and an array literal. For example, `count [1] [2]` is parsed as 2 separate arguments to the count function rather than a single argument `[1][2]`.

#### Valid
```[1, 2, 3][0]```

#### Invalid
```[1, 2, 3] [0]```

To get around this if necessary, you can always use the `index` method for which indexing expressions are syntactic sugar.

## Gotcha 3: Using named variables with non-homogenous data structures

When mapping or filtering over non-homogenous data structures, certain variables may not 
be defined for every item. For example, consider the key "bar" in the following invalid expression:

#### Invalid
```
[{foo: 1, bar: 1}, {foo: 2}] | filter bar == null
                                       ^
                          bar is sometimes missing!
```

When iterating over the second object, the variable `bar` is not defined as the object contains
no such key.  To get around this, simply specify where the variable comes from by using the `@` symbol.

#### Valid
```
[{foo: 1, bar: 1}, {foo: 2}] | filter @.bar == null
                                        ^
                            If bar is missing, this
                            evaluates to null
```


## Gotcha 4: Type Correspondence with the host language.

We say in the docs that MistQL performs queries over _JSON-like data structures_, but what does that practically mean?

What _JSON-Like data structure_ means differs between languages, but in general, it should be approximately whatever the most common method of JSON parsing in a language returns. Each language in itself will have a slightly different mapping from data types to MistQL types.

In many cases, since MistQL is roughly a lowest common denominator between a lot of programming languages, some fidelity may be lost over the MistQL boundary. For example, since MistQL doesn't make a distinction between ints and floats, it may lead to unintuitive languages that have that distinction. More concretely, providing a variable of type `int` to the query `@` may be output as a `float`, simply because when passing through the identity query, we have to convert down to MistQL types in order to execute the query.
