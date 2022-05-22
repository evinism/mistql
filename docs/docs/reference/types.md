---
sidebar_position: 2
---

# Types

MistQL has 8 core datatypes, many of which correspond to their JSON counterparts.

External types are types that can be either provided or returned by a MistQL query. All other
types are considered internal and cannot be provided to or returned from MistQL. 

| Type     | Primitive | External | Comment                          |
| -------- | --------- | -------- | -------------------------------- |
| string   | ✔         | ✔        | Series of unicode codepoints     |
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

| Type     | Equality           |
| -------- | ------------------ |
| string   | exact              |
| number   | IEEE 754 compliant |
| boolean  | exact              |
| null     | `true`             |
| object   | Deep equality      |
| array    | Deep equality      |
| function | Referential        |
| regex    | On source and flag |

## Casting Tables

MistQL defines casting from some types to other types

| Type     | Cast to Float               | Cast To String                                     | Truthiness                         |
| -------- | --------------------------- | -------------------------------------------------- | ---------------------------------- |
| string   | Parsed as float, as per JSON standard. | noop      | `false` if empty, `true` otherwise |
| number   | noop | As base 10 float. If number is an integer, no trailing digits or decimal. Exponential notation when not within non-inclusive range `1e-7` to `1e21` | IEEE 754 compliant                 |
| boolean  | 1 for `true`, 0 for `false` | `"true"` for `true`, `"false"` for `false`         | Standard                           |
| null     | 0                           | `"null"`                                           | `false`                            |
| object   | Invalid Operation           | Concise JSON, recursively converting items         | `false` if empty, `true` otherwise |
| array    | Invalid Operation           | Concise JSON, recursively converting items         | `false` if empty, `true` otherwise |
| function | Invalid Operation           | Invalid Operation                                  | `true`                             |
| regex    | Invalid Operation           | Invalid Operation                                  | `true`                             |

## Type Properties
There are 3 properties that any given type may or may not exhibit. The properties are as follows:

* `Comparable`: Whether or not a type can be compared to another of the same type.
* `NumberCastable`: Whether or not a type can be cast to the `number` type.
* `StringCastable`: Whether or not a type can be cast to the `string` type.

The table for which types exhibit which properties can be seen below:

| Type     | Comparable | NumberCastable | StringCastable |
| -------- | ---------- | -------------- | -------------- |
| string   | ✔          | ✔              | ✔              |
| number   | ✔          | ✔              | ✔              |
| boolean  | ✔          | ✔              | ✔              |
| null     | ✔          | ✔              | ✔              |
| object   |            |                | ✔              |
| array    |            |                | ✔              |
| function |            |                |                |
| regex    |            |                |                |
