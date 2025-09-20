## Python Implementation Details

### Architecture Overview

The Python implementation follows a clean, modular architecture with the following key components:

```
py/
├── query.py            # Main entry point, exports the primary `query(query: str, data: Any) -> Any` function
├── instance.py         # MistQL instance management
├── parse.py            # Parser implementation using Lark
├── execute.py          # Expression execution engine with context management
├── runtime_value.py    # `RuntimeValue` class representing MistQL values
├── builtins.py         # MistQL built-in functions (40+ functions)
├── expression.py       # AST node definitions
├── stack.py            # Execution stack management
├── gardenwall.py       # Type boundary management
├── cli.py              # Command line interface, pretty printing, file management, JSON/JSONL support
├── exceptions.py       # Error handling, exception hierarchy, and user-friendly error messages
```

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
