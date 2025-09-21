<img width="1022" alt="Screen Shot 2022-05-28 at 11 46 17 AM" src="https://user-images.githubusercontent.com/1979887/170838934-0553c383-d517-4158-8d29-589cd089ec28.png">

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/evinism/mistql/blob/main/LICENSE)
![Python](https://github.com/evinism/mistql/actions/workflows/python.yml/badge.svg)
![Node.js](https://github.com/evinism/mistql/actions/workflows/node.js.yml/badge.svg)
[![npm version](https://img.shields.io/npm/v/mistql.svg?style=flat)](https://www.npmjs.com/package/mistql)
[![npm version](https://img.shields.io/pypi/v/mistql.svg?style=flat)](https://pypi.org/project/mistql/)

MistQL is a query language for JSON-like structures, built for embedding across multiple domains. It supports
logic for querying and manipulating JSON-like data in a simple, readable manner. MistQL serves as a powerful
common expression language across multiple implementations.

For more detailed usage information, please visit MistQL's docs site.

[Join the Discord!!](https://discord.gg/YupxqvE5Jk)

### Links

- [MistQL's doc site](https://www.mistql.com/)
- [Getting Started](https://www.mistql.com/docs/intro)
- [Try it out!](https://www.mistql.com/tryitout)
- [Functions](https://www.mistql.com/docs/reference/functions)
- [Reference](https://www.mistql.com/docs/reference/overview)

# At A Glance

MistQL is an embedded query language.

```js
import mistql from 'mistql';

const animals = [
  {name: 'charlie', species: 'dog'},
  {name: 'mulberry', species: 'cat'},
]

const cats = mistql.query('@ | filter species == "dog" | count')
```

The primary power of MistQL comes from its strong cross-platform behavior semantics. For example, the following have the same behavior:

```py
# Python
import mistql
import json

query = 'events | groupby type | keys'
print(mistql.query(query, json.loads(data)))
```

```js
// JavaScript
import mistql from 'mistql'

const query = 'events | groupby type | keys';
console.log(mistql.query(query, JSON.parse(data)))
```

```rust
// Rust
use mistql::query;
let query = "events | groupby type | keys";
let result = query(query, data);
```

# Developing MistQL

Contributions to MistQL are very welcome!

As MistQL is a small project, there are no formatting requirements for either issues or pull requests.

If you're planning on making a new implementation, [ping the discord](https://discord.gg/YupxqvE5Jk) and we'll coordinate!

For an example PR that adds a function to MistQL, refer to [this PR](https://github.com/evinism/mistql/pull/175) as an example of a new function with moderate complexity.

### Code workflow

Code contributions to MistQL should roughly follow standard open source workflows:

1. Fork the project
2. Make code changes on your fork of the project.
3. (if necessary) Pull upstream to bring in new changes
4. Submit a pull request to MistQL's `main` branch.
5. (if necessary) Implement changes requested by maintainers.
6. Wait for the branch to be accepted and merged by maintainers!

### MistQL standard

No MistQL standard yet exists, but we're aiming for the `0.5.0` release of `mistql` as a standardizable language. After the `0.5.0` release, we will create a language specification, separate from any implementation.

In the meantime, we're actually pretty close.

We have a [Lark grammar](https://github.com/evinism/mistql/blob/main/py/mistql/grammar.lark)
which defines the language's syntax. This is likely the final grammar that will be formalized
into ABNF, although it is possible that we may need to fix minor issues before `0.5.0`. I
expect this to barely change, if at all.

Additionally, our language-independent test suite is rather extensive and forms the de-facto
standard of behaviors, as shared by both Python and JavaScript. While not strictly formalized,
the tests and the docs together form a cohesive body of behaviors, that, except for a few
minuitae, is of sufficient detail to be standardized.

## Directory Structure

MistQL's directory structure is a monorepo, currently consisting of these main directories:

1. `/docs`: Documentation Site (hosted at [mistql.com](https://www.mistql.com/))
2. `/js`: MistQL's browser implementation (e.g. `mistql` on npm).
3. `/py`: MistQL's python implementation (e.g. `mistql` on pypi).
4. `/rust`: MistQL's rust implementation (e.g. `mistql` on crates.io (not yet published)).
5. `/shared`: Shared assets between all implementation. Contains the language-independent test suite.

## Developing for the docs site

Docs are built via a fairly standard [Docusaurus 2](https://docusaurus.io/) implementation. Please follow Docusaurus's docs for developing for the Docs site.

## Developing for `mistql` on npm

`mistql` is written exclusively using typescript. Additionally, `mistql` uses `yarn` for dependency management, versioning, and uploading. JS-specific tests are stored alongside their implementation, using the suffix `.spec.ts`. Tests that describe the language itself are written in a language agnostic JSON format in the `/shared` directory. Writing tests for all feature additions and bug fixes is strongly encouraged.

For all major improvements, it is strongly encouraged to run `yarn bundlesize` to estimate gzipped impact of MistQL on a browser. MistQL for the browser should, in general, remain relatively close to 5kb.

The directory structure is relatively flat, except for the single `src/builtins` folder, which contains the implementation of all of MistQL's internal functions.

## Developing for `mistql` on pypi

`mistql` is a fairly standard python package managed with [poetry](https://python-poetry.org/).

Tests can be run using pytest, e.g. `poetry run pytest` from within the `/py` directory.

## Developing for `mistql` on crates.io

TBD.
