---
sidebar_position: 2
---

# Python Implementation

The Python implementation of MistQL can be installed via `pip install mistql`.

### Example Usage:

```py
import mistql

length = mistql.query('count @', [1, 2, 3])
print(length)
```

### `mistql` package exports

| Export | type | Description |
|---|---|---|
| `query` | `(query: string, data: any) => any` | The query interface for MistQL | 
| `__version__` | `str` | The current version of MistQL installed | 


### Type correspondence between Python and MistQL
Separate from the specification of the language, the python implementation maps from
Python types to MistQL types in a specific manner, as documented by the table below. As
the following is NOT part of the language, rather the library, the correspondence
may change with further releases of the `mistql` package, following semver.

Note that this is generally very restrictive in what it allows, but 
can be easily expanded as necessary. For examples, iterables as a whole may, outside of 
specific cases, be casted directly to MistQL array types.

If this correspondence doesn't work for you for some reason, please open an issue and
describe the situation carefully. This correspondence is easier to change than the language
itself.

| Value | MistQL Value |
|---|---|
| `number` | `double` |
| `boolean` | `boolean` |
| `string` | `string` |
| `dict` | `object` |
| `None` | `null` |
| `list` | `array` |
| `tuple` | `array` |
| `Date` | `date.toISOString()` |
| Anything else | Error |

