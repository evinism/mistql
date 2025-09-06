# MistQL Rust Implementation

This is the Rust implementation of MistQL, a miniature query language for performing computations on JSON-like structures. It's designed for embedding across multiple domains and serves as a powerful common expression language with strong cross-platform behavior semantics.

## Project Status

🚧 **Work in Progress** - This is an active port of MistQL to Rust, maintaining compatibility with the existing JavaScript and Python implementations.

## Development Roadmap

### Phase 1: Core Architecture (Tasks 1-7)

- [x] **Project Setup** - Set up Rust project structure with Cargo.toml, workspace configuration, and basic directory layout
- [ ] **Dependencies Research** - Research and select Rust dependencies for JSON parsing, regex, CLI, and testing frameworks
- [ ] **Type System** - Implement RuntimeValue enum and type system for 8 core MistQL types (null, boolean, number, string, object, array, function, regex)
- [ ] **Parser Implementation** - Implement parser using nom or pest to replace Lark grammar, supporting all MistQL syntax features
- [ ] **AST Definition** - Define AST node types (FnExpression, RefExpression, ValueExpression, Array, Object, Pipe expressions)
- [ ] **Execution Engine** - Implement expression execution engine with contextualized expressions and lazy evaluation
- [ ] **Stack Management** - Implement execution stack for variable scoping and context management

### Phase 2: Feature Implementation (Tasks 8-12)

- [ ] **Built-in Functions** - Implement all 40+ built-in functions (array, object, string, mathematical, utility operations)
- [ ] **Instance Management** - Implement MistQLInstance for custom functions and parameterized instances
- [ ] **Type Boundaries** - Implement type conversion between Rust and MistQL types (serde integration, special handling for Option<T>, DateTime, etc.)
- [ ] **Error Handling** - Implement custom error types and user-friendly error messages
- [ ] **CLI Interface** - Implement command-line interface with JSON/JSONL support and pretty printing

### Phase 3: Quality & Compatibility (Tasks 13-16)

- [ ] **Testing Integration** - Integrate with shared test suite and implement Rust-specific tests
- [ ] **Performance Optimization** - Optimize for performance and memory efficiency, benchmark against JS/Python implementations
- [ ] **Documentation** - Create Rust-specific documentation and usage examples
- [ ] **Cross-platform Validation** - Validate cross-platform compatibility with JavaScript and Python implementations

## Key Technical Considerations

### Type Safety
- Rust's type system should provide better compile-time guarantees than Python's runtime checking
- Zero-copy JSON parsing where possible
- Efficient string handling with minimal allocations

### Performance
- Significant performance improvements over Python expected
- Memory-efficient immutable runtime values with reference sharing
- Lazy evaluation for expressions

### Cross-platform Compatibility
- Must pass all shared tests from `/shared` directory
- Identical behavior semantics with JavaScript and Python implementations
- Same API surface as Python implementation

## Target API

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

## MistQL Language Features

### Core Types
- **8 Core Types**: null, boolean, number, string, object, array, function, regex
- **Type Conversion**: Automatic conversion between Rust and MistQL types
- **Special Handling**: `Option<T>` → `null`, `DateTime` → ISO strings

### Syntax Features
- **Contextualized Expressions**: `@` variable changes context in iterations
- **Piping**: Clean function chaining syntax (`data | filter condition | map field`)
- **Operators**: Arithmetic, logical, comparison, and regex matching
- **Indexing**: Dot notation, bracket notation, array/string indexing with negative indices and slicing

### Built-in Functions (40+)
- **Array Operations**: count, filter, map, find, reduce, sort, reverse, flatten, withindices
- **Object Operations**: keys, values, entries, fromentries, mapkeys, mapvalues, filterkeys, filtervalues
- **String Operations**: split, stringjoin, replace, match, regex
- **Mathematical**: sum, summarize, range
- **Utility**: if, log, apply, string, float

## Dependencies (Planned)

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
regex = "1.0"
clap = { version = "4.0", features = ["derive"] }
thiserror = "1.0"
nom = "7.0"  # or pest = "2.0" for parsing
```

## Development Tools

```toml
[dev-dependencies]
criterion = "0.5"  # Benchmarking
proptest = "1.0"   # Property-based testing
```

## Project Structure

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

## Contributing

This implementation follows the same patterns as the existing JavaScript and Python implementations. Key principles:

1. **Cross-platform Compatibility**: All implementations must behave identically
2. **Shared Test Suite**: Must pass all tests in `/shared` directory
3. **Performance**: Rust implementation should be significantly faster than Python
4. **Type Safety**: Leverage Rust's type system for compile-time guarantees

## Resources

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
