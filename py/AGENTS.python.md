## Python Implementation Details

### Architecture Overview

The Python implementation follows a clean, modular architecture with the following key components:

#### Core Modules

1. **`query.py`** - Main entry point
   - Exports the primary `query(query: str, data: Any) -> Any` function
   - Uses the default instance for query execution

2. **`instance.py`** - MistQL Instance Management
   - `MistQLInstance` class for parameterized instances
   - Supports custom functions via `extras` parameter
   - `default_instance` for simple usage

3. **`parse.py`** - Parser Implementation
   - Uses Lark parser with `grammar.lark` for syntax parsing
   - Converts Lark AST to MistQL expression tree
   - Handles operators, function calls, indexing, and piping

4. **`execute.py`** - Expression Execution Engine
   - `execute()` function for evaluating expressions
   - Handles function calls, piping, and contextualized expressions
   - Manages execution stack and context

5. **`runtime_value.py`** - Type System and Value Management
   - `RuntimeValue` class representing MistQL values
   - Type conversion between Python and MistQL types
   - Equality, comparison, and truthiness operations
   - JSON serialization and string formatting

6. **`builtins.py`** - Built-in Functions
   - All MistQL built-in functions (40+ functions)
   - Decorator-based function registration
   - Type checking and argument validation

7. **`expression.py`** - AST Node Definitions
   - Expression types: `FnExpression`, `RefExpression`, `ValueExpression`
   - Array, Object, and Pipe expressions
   - Type-safe expression tree structure

8. **`stack.py`** - Execution Stack Management
   - Context stack for variable scoping
   - Variable resolution with absolute/relative references
   - Stack frame creation from runtime values

9. **`gardenwall.py`** - Type Boundary Management
   - Input/output conversion between Python and MistQL types
   - Ensures type safety at language boundaries

10. **`cli.py`** - Command Line Interface
    - Full-featured CLI with JSON/JSONL support
    - Pretty printing and file I/O options
    - Argument parsing and error handling

11. **`exceptions.py`** - Error Handling
    - Custom exception hierarchy
    - Type errors, runtime errors, and reference errors
    - User-friendly error messages

### Key Features

#### Type System
- **8 Core Types**: null, boolean, number, string, object, array, function, regex
- **Type Safety**: Runtime type checking with `typeguard`
- **Automatic Conversion**: Python types → MistQL types → Python types
- **Special Handling**: `datetime` → ISO strings, `None` → `null`

#### Expression Evaluation
- **Contextualized Expressions**: `@` variable changes context in iterations
- **Lazy Evaluation**: Functions receive unevaluated AST nodes
- **Stack-based Execution**: Proper variable scoping and resolution
- **Piping Support**: Clean function chaining syntax

#### Built-in Functions
- **40+ Functions**: Array, object, string, mathematical operations
- **Decorator Pattern**: `@builtin(name, min_args, max_args)` registration
- **Type Validation**: Automatic argument count and type checking
- **Contextualized**: Functions like `filter`, `map`, `groupby` use context

#### Parser Architecture
- **Lark Grammar**: Language syntax defined in `grammar.lark`
- **AST Transformation**: Lark tree → MistQL expression tree
- **Operator Handling**: Infix operators converted to function calls
- **Indexing Support**: Bracket notation and dot access

### Dependencies

```toml
dependencies = [
    "lark>=1.0.0,<2.0.0",           # Parser generator
    "typeguard>=2.13.3,<5.0.0",     # Runtime type checking
    "json-lines>=0.5.0,<1.0.0",     # JSONL file support
]
```

### Development Tools

```toml
dev = [
    "pytest>=7.4.4,<8.0.0",         # Testing framework
    "ruff>=0.11.12,<0.12.0",        # Linting and formatting
    "toml>=0.10.2,<1.0.0",          # Configuration parsing
    "mypy>=1.4.1,<2.0.0",           # Static type checking
]
```

### Usage Patterns

#### Basic Usage
```python
import mistql
result = mistql.query('events | filter type == "purchase" | count', data)
```

#### Custom Functions
```python
from mistql import MistQLInstance

def custom_sum(a, b):
    return a + b

instance = MistQLInstance(extras={'custom_sum': custom_sum})
result = instance.query('custom_sum 1 2', None)  # Returns 3
```

#### CLI Usage
```bash
# Basic query
echo '{"items": [1,2,3]}' | python -m mistql "count items"

# JSONL processing
python -m mistql "filter active" --file_jsonl data.jsonl

# Pretty output
python -m mistql "groupby category" --file data.json --pretty
```

### Performance Characteristics

- **Memory Efficient**: Immutable runtime values with reference sharing
- **Lazy Evaluation**: Expressions evaluated only when needed
- **Type Safety**: Runtime checks with minimal overhead
- **Cross-platform**: Identical behavior with JavaScript implementation

### Error Handling

- **Graceful Degradation**: Missing object keys return `null`
- **Type Safety**: Clear error messages for type mismatches
- **Reference Errors**: Undefined variables properly handled
- **Parser Errors**: Syntax errors with helpful messages

## Version Information

- Current version: 0.4.12
- Target for 0.5.0: Language standardization
- Cross-platform compatibility maintained across versions
