---
sidebar_position: 3
---

# Operators

Operators are functions with specialized syntax for ease of use. They come in two forms: unary and binary.

## Unary Operators

There are only two unary operators in MistQL


| Operator | Parameter Type | Return Type | Description |
|---|---|---|---|
| `!` | `Any` | `Boolean` | `false` if the argument is truthy, `true` otherwise |
| `-` | `Number` | `Number` | Returns the negative of the number |

## Binary Operators

Binary operators make up the vast majority of MistQL's operators.

| Operator | Parameter Types | Return Type | Description |
|---|---|---|---|
|`+`| `number` or `string` | `number` or `string` | Adds two numbers or concatenates two strings, depending on argument type |
|`-`| `number` | `number` | Subtracts one number from another |
|`*`| `number` | `number` | Multiplies 2 numbers |
|`/`| `number` | `number` | Divides one number by another |
|`%`| `number` | `number` | Computes `a mod b`|
|`<`| `number` or `string` | `number` or `string` | Less Than |
|`>`| `number` or `string` | `number` or `string` | Greater Than |
|`<=`| `number` or `string` | `number` or `string` | Less Than or Equal |
|`>=`| `number` or `string` | `number` or `string` | Greater Than or Equal |
|`==`| `any` | `boolean` | Whether two values are equivalent |
|`!=`| `any` | `boolean` | Whether two values are not equivalent |
|`&&`| `any` | `boolean` | `true` if the two values are both truthy, `false` otherwise |
|`\|\|`| `any` | `boolean` | `true` if either of the values is truthy, `false` otherwise |
