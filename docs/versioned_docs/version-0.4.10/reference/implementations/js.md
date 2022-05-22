---
sidebar_position: 1
---

# JS Implementation

The JS implementation of MistQL can be installed via `npm install mistql`.

### Example Usage:

```js
import {query} from 'mistql';

const len = query('count @', [1, 2, 3]);
console.log(len);
```

### `mistql` package exports

| Export | type | Description |
|---|---|---|
| `query` | `(query: string, data: any) => any` | The query interface for MistQL | 
| `default` | `{query: query}` | An object solely consisting of the query interface | 

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

