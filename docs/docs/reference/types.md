---
sidebar_position: 2
---

# Types

MistQL has 8 core datatypes, many of which correspond to their JSON counterparts.

External types are types that can be either provided or returned by a MistQL query. All other
types are considered internal and cannot be provided to or returned from MistQL. 

| Type     | Primitive | External | Comment                          |
| -------- | --------- | -------- | -------------------------------- |
| string   | ✔         | ✔        |                                  |
| number   | ✔         | ✔        | Doubles, without NaN or infinity |
| boolean  | ✔         | ✔        |                                  |
| null     | ✔         | ✔        |                                  |
| object   |           | ✔        | Stringly typed keys              |
| array    |           | ✔        | Can be nonhomogenous             |
| function |           |          | Arity not part of type system    |
| regex    |           |          |                                  |

## Type Equality

Equality in MistQL is strict, meaning that if two variables have different 
data types, they are considered unequal. 

| Type     | Equality           | Truthiness                         |
| -------- | ------------------ | ---------------------------------- |
| string   | exact              | `false` if empty, `true` otherwise |
| number   | IEEE 754 compliant | IEEE 754 compliant                 |
| boolean  | exact              | Standard                           |
| null     | `true`             | `false`                            |
| object   | Deep equality      | `false` if empty, `true` otherwise |
| array    | Deep equality      | `false` if empty, `true` otherwise |
| function | Referential        | `true`                             |
| regex    | On source and flag | `true`                             |


## Casting Tables

MistQL defines casting from some types to other types

| Type     | Cast to Float               | Cast To String                                     |
| -------- | --------------------------- | -------------------------------------------------- |
| string   | Parsed as float, as per JSON standard. | noop      |
| number   | noop | As base 10 float. If number is an integer, no trailing digits or decimal. Exponential notation when not within non-inclusive range `1e-7` to `1e21` |
| boolean  | 1 for `true`, 0 for `false` | `"true"` for `true`, `"false"` for `false`         |
| null     | 0                           | `"null"`                                           |
| object   | Throws error                | Concise JSON, recursively converting items         |
| array    | Throws error                | Concise JSON, recursively converting items         |
| function | Throws error                | Throws error                                       |
| regex    | Throws error                | Throws error                                       |
