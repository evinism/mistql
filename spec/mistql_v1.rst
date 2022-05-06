===============
MistQL Standard
===============

MistQL is a language for performing computations on JSON-like structures in a variety of programming languages.

Scope
=====
This spec defines the MistQL language.

Values
======

This section describes all values representable in MistQL. The set of all values in MistQL is denoted `Value`.

Null value
----------
A primitive value representing the absence.

Boolean value
-------------
There are two boolean values, named `true` and `false`.

Number value
------------
A primitive value corresponding to a double-precision IEEE 754-2019 value, with the values NaN, positive infinity, and negative infinity removed.

String value
------------
A primitive value containing a sequence of UTF-8 codepoints.

Array value
-----------
A finite ordered sequence of `Value`s of length zero or greater. It is not required that all values in the sequence are of the same type.

Object value
------------
An unordered set of pairs called entries. The first element of each entry is a string `Value` denoted `key`, and the second element of each entry is a `Value` denoted `value`. Entries in an object are unique on the `key` value: no two entries in an object share the same `key`.

Function Value
--------------
A function value is a value that can be executed as part of a function application. The set of function values is equivalent to the set of builtin functions.

Regex Value
-----------
A Regex value is a string value `pattern` equipped with 4 boolean-valued flags.

The flags are:

* The `global` flag (abbreviated as `g`)
* The `invariant` flag (abbreviated as `i`)
* The `multiline` flag (abbreviated as `m`)
* The `dotall` flag (abbreviated as `s``)

Lexical Context
===============

