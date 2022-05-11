---
sidebar_position: 5
---

# Indexing

Indexing is done via 3 different methods:

1. The `index` function.
2. Subscripting
3. Dot Access

While indexing is primarily done via subscripting and dot access, both these operations are best defined via the `index function`

## The `index` function
The index function has two arities.

### Arity of 2
Index behavior is defined as follows when operating in the two arity mode:

| Value Type | Allowed Indexes    | Returns             |
|------------|--------------------|---------------------|
| `object`   | `string`           | Value at key        |
| `array`    | `number`           | Value at offset     |
| `string`   | `number`           | Character at offset |
| `null`     | `string`, `number` | `null`              |

All other signatures should raise RuntimeErrors.

### Arity of 3
Index behavior is defined as follows when operating in the two arity mode:

| Value Type | Allowed Indexes    | Returns                              |
|------------|--------------------|--------------------------------------|
| `array`    | `number` or `null` | Array of values between offsets      |
| `string`   | `number` or `null` | String of characters between offsets |

All other signatures should raise RuntimeErrors.

## Subscript Access
Subscript access is equivalent to calling the index function, using the following table of syntactic sugar

| Subscript Syntax | Index Syntax          |
|------------------|-----------------------|
| `foo[bar]`       | `index bar foo`       |
| `foo[bar:]`      | `index bar null foo`  |
| `foo[:bar]`      | `index null bar foo`  |
| `foo[bar:baz]`   | `index null bar foo`  |
| `foo[:]`         | `index null null foo` |

## Dot Access
Dot access is equivalent to the 2-arity index function with the right hand side interpereted as
a string.

For example, `foo.bar` is syntactic sugar for `index "bar" foo`

## Indexing strings
Strings are indexed via full unicode codepoints rather than with surrogate pairs. For example, `"😊a"[0] == "😊"` and `"😊a"[1] == "a"`. MistQL treats the two codepoints in modifiers as separate characters, as in `"👋🏽"[0] == "👋"`.
