# MistQL: Query language for JSON-like structures
![mistql logo](https://www.mistql.com/assets/images/icon128-020f567a30894a6c26227dc6773d3406.png)

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/evinism/mistql/blob/main/LICENSE) 
![Python](https://github.com/evinism/mistql/actions/workflows/python.yml/badge.svg) 
![Node.js](https://github.com/evinism/mistql/actions/workflows/node.js.yml/badge.svg) 
[![npm version](https://img.shields.io/npm/v/mistql.svg?style=flat)](https://www.npmjs.com/package/mistql)
[![npm version](https://img.shields.io/pypi/v/mistql.svg?style=flat)](https://pypi.org/project/mistql/)



MistQL is a miniature embeddable query language for JSON-like structures, built for embedding within applications. It supports
logic for querying and manipulating JSON-like data in a simple, readable manner.

For more detailed usage information, please visit MistQL's docs site.

### Links

- [MistQL's doc site](https://www.mistql.com/)
- [Getting Started](https://www.mistql.com/docs/intro)
- [Try it out!](https://www.mistql.com/tryitout)
- [Functions](https://www.mistql.com/docs/reference/functions)
- [Reference](https://www.mistql.com/docs/reference/overview)

# Developing MistQL

Contributions to MistQL are very welcome!

As MistQL is still a small project, there are no formatting requirements for either issues or pull requests.

### Code workflow

Code contributions to MistQL should roughly follow standard open source workflows:

1. Fork the project
2. Make code changes on your fork of the project.
3. (if necessary) Pull upstream to bring in new changes
4. Submit a pull request to MistQL's `main` branch.
5. (if necessary) Implement changes requested by maintainers.
6. Wait for the branch to be accepted and merged by maintainers!

### MistQL standard

No MistQL standard yet exists, but we're aiming for the `0.5.0` release of `mistql` on npm as a standardizable language. After the `0.5.0` release, we will create a language specification, separate from any implementation. 

In the meantime, our language-independent test suite is rather extensive and forms the de-facto
standard, as shared by both Python and JavaScript. While not formalized, the tests and the docs
together form a cohesive body of behaviors, that, except for a few minute details, is of sufficient
detail to be standardized.

## Directory Structure

MistQL's directory struture is a monorepo, currently consisting of these main directories:

1. `/docs`: Documentation Site (hosted at [mistql.com](https://www.mistql.com/))
2. `/js`: MistQL's browser implementation (e.g. `mistql` on npm).
3. `/py`: MistQL's python implementation (e.g. `mistql` on pypi).
4. `/shared`: Shared assets between all implementation. Contains the language-independent test suite.

## Developing for the docs site

Docs are built via a fairly standard [Docusaurus 2](https://docusaurus.io/) implementation. Please follow Docusaurus's docs for developing for the Docs site.

## Developing for `mistql` on npm

`mistql` is written exclusively using typescript. Additionally, `mistql` uses `yarn` for dependency management, versioning, and uploading. JS-specific tests are stored alongside their implementation, using the suffix `.spec.ts`. Tests that describe the language itself are written in a language agnostic JSON format in the `/shared` directory. Writing tests for all feature additions and bug fixes is strongly encouraged.

For all major improvements, it is strongly encouraged to run `yarn bundlesize` to estimate gzipped impact of MistQL on a browser. MistQL for the browser should, in general, remain relatively close to 5kb.

The directory structure is relatively flat, except for the single `src/builtins` folder, which contains the implementation of all of MistQL's internal functions.

## Developing for `mistql` on pypi

`mistql` is a fairly standard python package managed with [poetry](https://python-poetry.org/).

Tests can be run using pytest, e.g. `poetry run pytest` from within the `/py` directory.
