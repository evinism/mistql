# MistQL Rust Implementation

This is the Rust implementation of MistQL, a miniature query language for
performing computations on JSON-like structures. It's designed for embedding
across multiple domains and serves as a powerful common expression language with
strong cross-platform behavior semantics.

## Project Status

**Current Status**:

- [x] **Core Architecture**: Lexer, Parser, Executor, Type System.
- [x] **Basic Query Function**: Main API working.
- [x] **Builtin Functions**: All functions are implemented.
- [x] **Shared Test Framework**: 511/514 assertions in the shared test suite are passing.
- [x] **Refactor**: Carefully read the LLM-assisted port and refactor it.
- [ ] **Performance Optimization** - Optimize for performance and memory efficiency, benchmark against JS/Python implementations
- [ ] **Documentation** - Create Rust-specific documentation and usage examples

## Quick Start

### Running the CLI

TODO.

```bash
cargo run --bin mq
```

### Example API Usage

```rust
use mistql::query;

// Basic usage
let data = serde_json::json!([{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]);
let result = query("filter age > 26 | map name", &data)?;

// Custom functions
use mistql::MistQLInstance;
let instance = MistQLInstance::new()
    .with_custom_function("custom_sum", |a, b| a + b);
let result = instance.query("custom_sum 1 2", &serde_json::Value::Null)?;
```

## Contributing

Feel free to open up a PR! This project follows the same patterns as the existing JavaScript and Python implementations.

### Running Tests

```bash
# Run the full shared test suite
cargo test test_shared_suite -- --nocapture

# Run all tests
cargo test
```

### Project Structure

```
rust/
├── Cargo.toml          # Package configuration
├── README.md           # This file
└── src/
    ├── lib.rs          # Library entry point
    ├── bin/
    │   └── mq.rs       # CLI binary
    ├── types.rs        # Runtime value types
    ├── parser.rs       # Syntax parser
    ├── executor.rs     # Expression execution
    ├── builtins.rs     # Built-in functions
    ├── instance.rs     # Instance management
    └── errors.rs       # Error handling
```

### Resources

- Overview files (relative to monorepo root):
  - README.md
  - AGENTS.md
  - py/AGENTS.python.md
  - rust/README.md
- **Main Documentation**: https://www.mistql.com/
- **Try it out**: https://www.mistql.com/tryitout
- **Discord**: https://discord.gg/YupxqvE5Jk
- **Grammar**: [Lark grammar file](https://github.com/evinism/mistql/blob/main/py/mistql/grammar.lark)
- **Python Implementation**: `/py` directory for reference
- **JavaScript Implementation**: `/js` directory for reference
