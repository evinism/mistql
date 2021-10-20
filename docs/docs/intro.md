---
sidebar_position: 1
---

# Getting Started

MistQL is a miniature language for performing computations on JSON-like structures.

MistQL's JavaScript implementation is built from the ground up to be included in clientside browser applications. It has no dependencies and has a total footprint of 5.2kB gzipped, making it suitable for size-sensitive frontends. 

## Installation

The JavaScript implementation of MistQL is installed via the following:

```shell
npm install --save mistql
```

## Code usage

MistQL can be interacted with programatically:

```js
import mistql from 'mistql';

const query = 'events | filter type == "purchase" | groupby email | keys';
const purchaserEmails = mistql.query(query, data);
```

## Command line usage

MistQL exposes a command line interface under the name `mq`. `mq` can be installed globally via `npm install -g mistql`.

The CLI can be used via `mq <query> [file]`

If file is not provided, `mq` defaults to `stdin`. An example usage might be the following:

```sh
$ echo "[]" | mq "count @"
> 0
````
