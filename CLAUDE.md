# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MistQL is a miniature query language for computations on JSON-like structures, with implementations in JavaScript/TypeScript, Python, Rust, and Kotlin. All implementations must produce identical behavior — cross-platform correctness is critical.

## Build & Test Commands

The project uses `talc` (a task CLI defined in `talc.yaml`) for orchestration:

```bash
talc install          # Install all dependencies
talc test all         # Test all implementations
talc test js          # JS tests (cd js && npm test)
talc test py          # Python tests (cd py && uv run pytest && uv run mypy mistql)
talc build all        # Build all
talc build js         # JS build via rollup (cd js && npm run build)
talc build py         # Python build (cd py && uv build)
```

### Running tests directly

```bash
# JavaScript - ts-mocha, tests are colocated as *.spec.ts
cd js && npm test
cd js && npx ts-mocha -p tsconfig.tests.json 'src/builtins/count.spec.ts'  # single test file

# Python - pytest
cd py && uv run pytest
cd py && uv run pytest tests/test_shared.py -k "test_name"  # single test
cd py && uv run mypy mistql                                  # type checking

# Rust
cd rust && cargo test

# Kotlin - Gradle + JUnit 5
cd kt && ./gradlew test
```

## Architecture

### Monorepo layout
- `js/` — TypeScript implementation (lexer → parser → executor pipeline)
- `py/` — Python implementation (Lark grammar-based parser)
- `rust/` — Rust implementation (nom parser combinators)
- `kt/` — Kotlin/JVM implementation (lexer → parser → executor pipeline)
- `shared/` — Language-independent test suite (`testdata.json`) and sample data
- `docs/` — Docusaurus 3 site (mistql.com)

### Shared test suite
`shared/testdata.json` defines the canonical test cases that all implementations must pass. When adding or modifying language features, update this file and ensure all implementations pass.

### JS implementation (`js/src/`)
Pipeline: `lexer.ts` → `parser.ts` → `executor.ts`. Builtin functions are individual files in `builtins/`, each exporting a function registered in `builtins/index.ts`. Tests live alongside source as `*.spec.ts`.

### Python implementation (`py/mistql/`)
Uses a Lark grammar (`grammar.lark`) for parsing. All builtins are in a single `builtins.py`. The runtime type system is in `runtime_value.py`. Tests are in `py/tests/`.

### Rust implementation (`rust/src/`)
Uses nom parser combinators in `parser.rs`. Builtins in `builtins.rs`, type system in `types.rs`.

### Kotlin implementation (`kt/src/main/kotlin/com/mistql/`)
Follows the JS architecture: `parser/Parser.kt` (lexer + token-based parser with precedence resolution) → `runtime/Executor.kt`. Builtins are split across files in `builtins/` (ArrayFunctions, StringFunctions, ObjectFunctions, MathFunctions, UtilityFunctions, Operators). Type system in `runtime/RuntimeValue.kt`. Uses Gradle with kotlinx-serialization-json. Tests in `kt/src/test/kotlin/com/mistql/`.

## Key Conventions

- The `@` symbol refers to the current context value; `$` accesses builtins/root
- Functions use contextualized expressions (e.g., `filter`, `map`, `sortby` change what `@` refers to)
- JS bundle target is ~5.3kB gzipped — monitor with `cd js && npm run bundlesize`
- Python uses `uv` as the package manager
- Version metadata is in `meta.json` at the repo root
