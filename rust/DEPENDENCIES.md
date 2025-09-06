# MistQL Rust Dependencies

This document outlines the selected dependencies for the MistQL Rust implementation, with rationale for each choice.

## Core Dependencies

### JSON and Serialization
- **`serde`** (1.0) - Framework for serializing and deserializing Rust data structures
- **`serde_json`** (1.0) - JSON serialization/deserialization library
- **Rationale**: Industry standard for JSON handling in Rust. Provides type-safe serialization and excellent performance.

### Error Handling
- **`thiserror`** (1.0) - Derive macro for custom error types
- **`anyhow`** (1.0) - Flexible error handling with context
- **Rationale**: `thiserror` provides clean error type definitions, while `anyhow` offers flexible error handling for application code.

### Parsing
- **`nom`** (7.0) - Parser combinator library
- **Rationale**: Replaces Python's Lark parser. Nom is the most mature and performant parser combinator library in Rust, with excellent documentation and community support.

### Regular Expressions
- **`regex`** (1.0) - Fast, safe regular expression library
- **Rationale**: Required for MistQL's regex type and string operations. The `regex` crate is the standard choice in Rust.

### CLI Interface
- **`clap`** (4.0) - Command-line argument parser
- **Rationale**: Replaces Python's argparse. Clap v4 with derive features provides excellent ergonomics and performance.

### Logging
- **`log`** (0.4) - Logging facade
- **`env_logger`** (0.10) - Environment-based logger
- **Rationale**: Required for MistQL's `log` function. Standard logging solution in Rust.

### Data Format Support
- **`jsonl`** (0.1) - JSON Lines support
- **Rationale**: Replaces Python's `json-lines` dependency for CLI JSONL file support.

### Date/Time Handling
- **`chrono`** (0.4) - Date and time library
- **Rationale**: Required for DateTime → ISO string conversion, matching Python's datetime handling.

### Mathematical Operations
- **`num-traits`** (0.2) - Numeric traits
- **`statistical`** (0.1) - Statistical functions
- **Rationale**: Required for MistQL's mathematical functions and statistical operations.

### String Manipulation
- **`unicode-segmentation`** (1.0) - Unicode text segmentation
- **Rationale**: Required for proper string handling in MistQL's string operations.

## Development Dependencies

### Testing
- **`rstest`** (0.19) - Testing framework with fixtures
- **`proptest`** (1.0) - Property-based testing
- **`tempfile`** (3.0) - Temporary file utilities
- **Rationale**: `rstest` provides excellent testing ergonomics, `proptest` enables comprehensive property-based testing, and `tempfile` is useful for file-based tests.

### Benchmarking
- **`criterion`** (0.5) - Statistical benchmarking
- **Rationale**: Required for performance comparison with JavaScript and Python implementations.

## Comparison with Existing Implementations

### Python Dependencies → Rust Equivalents
| Python | Rust | Purpose |
|--------|------|---------|
| `lark` | `nom` | Parser generation |
| `typeguard` | Rust type system | Runtime type checking |
| `json-lines` | `jsonl` | JSONL file support |
| `pytest` | `rstest` + `proptest` | Testing framework |
| `ruff` | `cargo fmt` + `clippy` | Code formatting/linting |
| `mypy` | Rust compiler | Static type checking |

### JavaScript Dependencies → Rust Equivalents
| JavaScript | Rust | Purpose |
|------------|------|---------|
| `rollup` | `cargo build` | Bundling |
| `typescript` | Rust compiler | Type checking |
| `mocha` | `rstest` | Testing framework |

## Performance Considerations

1. **Zero-copy JSON parsing**: `serde_json` supports zero-copy parsing where possible
2. **Efficient string handling**: `regex` and `unicode-segmentation` are optimized for performance
3. **Minimal allocations**: Nom parser combinators minimize memory allocations
4. **Fast CLI**: Clap v4 provides excellent performance for command-line parsing

## Security Considerations

1. **Safe regex**: The `regex` crate prevents ReDoS attacks
2. **Memory safety**: All dependencies are memory-safe by Rust's design
3. **Input validation**: Clap provides built-in input validation for CLI arguments

## Maintenance and Support

All selected dependencies are:
- Actively maintained with regular updates
- Widely used in the Rust ecosystem
- Well-documented with comprehensive examples
- Compatible with Rust's stability guarantees

## Future Considerations

- **SIMD optimizations**: May consider `simd-json` for even faster JSON parsing if needed
- **Async support**: Current dependencies support async/await if needed for future features
- **WebAssembly**: All dependencies are WASM-compatible for potential web deployment
