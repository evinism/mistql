# MistQL Rust Implementation

This is the Rust implementation of MistQL, a miniature query language for performing computations on JSON-like structures. It's designed for embedding across multiple domains and serves as a powerful common expression language with strong cross-platform behavior semantics.

## Project Status

🚧 **MVP Complete - Core Infrastructure Ready** - The basic MistQL infrastructure is working with a comprehensive test suite. Ready for systematic implementation of builtin functions.

**Current Status**:
- ✅ **Core Architecture**: Lexer, Parser, Executor, Type System
- ✅ **Test Framework**: 325+ tests running from shared test suite
- ✅ **Basic Query Function**: Main API working
- ⚠️ **Builtin Functions**: Most functions need implementation (see TODO list below)
- ⚠️ **Test Pass Rate**: ~0% (expected during development)

## Development Roadmap

### Phase 1: Core Architecture

- [x] **Project Setup** - Set up Rust project structure with Cargo.toml, workspace configuration, and basic directory layout
- [x] **Dependencies Research** - Research and select Rust dependencies for JSON parsing, regex, CLI, and testing frameworks
- [x] **Bare minimal testing framework** - Implement a bare minimal testing framework in Rust
- [x] **Type System** - Implement RuntimeValue enum and type system for 8 core MistQL types (null, boolean, number, string, object, array, function, regex)
- [x] **Test type system** - Test the type system (match tests in Python implementation)
- [x] **Lexer Implementation** - Implement lexer from scratch in Rust
- [x] **Test lexer** - Test the lexer (match tests in Python implementation)
- [x] **Parser Implementation** - Implement parser from scratch in Rust
- [x] **Test parser** - Test the parser (match tests in Python implementation)
- [x] **AST Definition** - Define AST node types (FnExpression, RefExpression, ValueExpression, Array, Object, Pipe expressions)
- [x] **Test AST** - Test the AST (match tests in Python implementation)
- [x] **Execution Engine** - Implement expression execution engine with contextualized expressions
  - [x] **Execution Stack** - Implement execution stack and context management system
  - [x] **Core Execution Engine** - Implement core expression execution engine with pattern matching
  - [x] **Binary and Unary Operators** - Implement binary and unary operator evaluation
  - [x] **Function Call Execution** - Implement function call execution and argument handling
  - [x] **Pipeline Execution** - Implement pipeline execution with context passing
  - [x] **Dot Access and Indexing** - Implement dot access and indexing operations
  - [x] **Comprehensive Tests** - Create comprehensive tests for executor functionality
  - [x] **Contextualized Expressions** - Implement contextualized expressions (@ variable handling)
- [ ] **Test execution engine** - Test the execution engine (match tests in Python implementation)

### Phase 2: Feature Implementation (Tasks 8-12)

- [x] **Built-in Functions** - Implement all 40+ built-in functions (array, object, string, mathematical, utility operations)
- [ ] **Instance Management** - Implement MistQLInstance for custom functions and parameterized instances
- [ ] **Type Boundaries** - Implement type conversion between Rust and MistQL types (serde integration, special handling for Option<T>, DateTime, etc.)
- [ ] **Error Handling** - Implement custom error types and user-friendly error messages
- [ ] **CLI Interface** - Implement command-line interface with JSON/JSONL support and pretty printing

### Phase 3: Quality & Compatibility (Tasks 13-16)

- [x] **Testing Integration** - Integrate with shared test suite and implement Rust-specific tests
- [ ] **Performance Optimization** - Optimize for performance and memory efficiency, benchmark against JS/Python implementations
- [ ] **Documentation** - Create Rust-specific documentation and usage examples
- [ ] **Cross-platform Validation** - Validate cross-platform compatibility with JavaScript and Python implementations

## Current Test Status

**Test Suite Status**: ✅ **Working** - 135 failed assertions from 192 test cases
- **Total Test Cases**: 192 test cases loaded from `/shared/testdata.json`
- **Total Assertions**: 508 individual test assertions
- **Current Pass Rate**: 73.4% (373 passed, 135 failed) - Excellent progress made!
- **Test Framework**: ✅ Fully functional with detailed failure reporting and metrics

## Quick Start

### Running Tests
```bash
# Run the full shared test suite
cargo test test_shared_suite -- --nocapture

# Run basic functionality tests
cargo test test_basic_functionality -- --nocapture

# Run all tests
cargo test
```

### Current Test Results
- **Test Cases**: 192 test cases from shared test suite
- **Assertions**: 508 individual test assertions
- **Pass Rate**: 45.7% (232 passed, 276 failed)
- **Main Issues**: Missing builtin functions, parser improvements needed

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

## Dependencies (Selected)

```toml
[dependencies]
# Core JSON and serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Parsing - using nom for parser combinators (replaces Lark)
nom = "7.0"

# Regular expressions (for regex type and string operations)
regex = "1.0"

# CLI interface (replaces Python's argparse)
clap = { version = "4.0", features = ["derive", "color", "suggestions"] }

# Logging (for debug logging function)
log = "0.4"
env_logger = "0.10"

# JSONL support (replaces Python's json-lines)
jsonl = "0.1"

# Date/time handling (for DateTime -> ISO string conversion)
chrono = { version = "0.4", features = ["serde"] }

# Mathematical operations and statistics
num-traits = "0.2"
statistical = "0.1"

# String manipulation utilities
unicode-segmentation = "1.0"
```

## Development Tools

```toml
[dev-dependencies]
# Testing framework (replaces Python's pytest)
rstest = "0.19"

# Property-based testing (for comprehensive test coverage)
proptest = "1.0"

# Benchmarking (for performance comparison with JS/Python)
criterion = { version = "0.5", features = ["html_reports"] }

# Additional testing utilities
tempfile = "3.0"
```

See [DEPENDENCIES.md](DEPENDENCIES.md) for detailed rationale and comparisons with existing implementations.

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
