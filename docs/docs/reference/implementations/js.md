---
sidebar_position: 1
---

# JS Implementation

The JS implementation of MistQL can be installed via `npm install mistql`.

### Example Usage:

```js
import { query } from 'mistql';

const len = query('count @', [1, 2, 3]);
console.log(len);
```

### `mistql` package exports

| Export | type | Description |
|---|---|---|
| `query` | `(query: string, data: any) => any` | The default query function for MistQL. Most uses of MistQL can rely solely on this function |
| `defaultInstance` | `MistQLInstance` | The default instance of MistQL. The exported `query` function is an alias to the `query` method on the default instance |
| `MistQLInstance` | `class` | The class for constructing parameterized MistQL instances. If you're adding custom functions to MistQL, you'll use this interface. |
| `jsFunctionToMistQLFunction` | `(fn) => FunctionValue` | Helper function for constructing MistQL functions from JS functions |
| `default` | `{query, defaultInstance, }` | An object consisting of the  | 

MistQL also exposes a number of TS types through the default interface, although they're not listed here.

### Type correspondence between JS and MistQL
Separate from the specification of the language, the JS implementation maps from
JS types to MistQL types in a specific manner, as documented by the table below. As
the following is NOT part of the language, rather the library, the correspondence
may change with further releases of the `mistql` package, following semver.

If this correspondence doesn't work for you for some reason, please open an issue and
describe the situation carefully. This correspondence is easier to change than the language
itself.

| Value | MistQL Value | Notes |
|---|---|---|
| `number` | `number` | Infinity and NaN casted to `null` |
| `boolean` | `boolean` | |
| `string` | `string` | |
| `object` | `object` | |
| `null` | `null` | |
| `array` | `array` | |
| `undefined` | `null` | |
| `function` | `object` with enumerated properties | |
| `Number` object | `number` | |
| `Boolean` object | `boolean` | |
| `String` object | `string` | |
| `Date` | `date.toISOString()` | |
| `Symbol` | `symbol.toString()` | |
| Anything else | `object` with enumerated properties | |

