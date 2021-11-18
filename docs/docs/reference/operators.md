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
|`+`| `number` or `string` or `array` | `number` or `string` or `array` | Adds two numbers, concatanates two strings, or concatanates two arrays, depending on argument type |
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
|`=~`| `string` or `regex` | `boolean` | Whether the left hand value matches the right hand value |
|`&&`| `any` | `boolean` | Returns the first if the first is falsy, the second otherwiseotherwise. |
|`\|\|`| `any` | `boolean` | Returns the first if the first is truthy, the second otherwise. NOTE: The backslashes aren't necessary. I just can't figure out how to format it properly for Docusaurus. |


## Operator precedence and associativity
Below are in order from highest to lowest, where all operators on the same level are equal precedence. 

| Operator | Associativity |
|---|---|
| `.` | ltr |
| unary `!`, unary `-` | rtl |
| `*`, `/`, `%` | ltr |
| `+`, `-` | ltr |
| `<`, `>`, `<=`, `>=` | ltr |
| `==`, `!=`, `=~` | ltr |
| `&&` | ltr |
| `\|\|` | ltr |
| `[function application]` | ltr |
| `\|` | ltr |
