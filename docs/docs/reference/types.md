---
sidebar_position: 2
---

# Types

MistQL has 8 core datatypes, many of which correspond to their JSON counterparts.

External types are types that can be either provided or returned by a MistQL query. All other
types are considered internal and cannot be provided to or returned from MistQL. 

| Type     | Primitive | External | Comment                       |
| -------- | --------- | -------- | ----------------------------- |
| string   | ✔         | ✔        |                               |
| number   | ✔         | ✔        | Double-precision float        |
| boolean  | ✔         | ✔        |                               |
| null     | ✔         | ✔        |                               |
| object   |           | ✔        | Stringly typed keys           |
| array    |           | ✔        | Can be nonhomogenous          |
| function |           |          | Arity not part of type system |
| regex    |           |          |                               |

## Type Equality

Equality in MistQL is strict, meaning that if two variables have different 
data types, they are considered unequal. 

| Type     | Equality           | Truthiness                         |
| -------- | ------------------ | ---------------------------------- |
| string   | exact              | `false` if empty, `true` otherwise |
| number   | IEEE 754 compliant | IEEE 754 compliant                 |
| boolean  | exact              | Standard                           |
| null     | `true```           | `false`                            |
| object   | Deep equality      | `true`                             |
| array    | Deep equality      | `true`                             |
| function | Referential        | `true`                             |
| regex    | On source and flag | `true`                             |
