# MistQL Agent Documentation

This document provides a comprehensive overview of MistQL for AI agents and developers working with the project.

## Project Overview

MistQL is a miniature query language for performing computations on JSON-like structures, designed for embedding across multiple domains. It serves as a powerful common expression language with strong cross-platform behavior semantics.

### Key Features
- **Cross-platform**: Same behavior across JavaScript and Python implementations
- **Lightweight**: JavaScript implementation is only 5.3kB gzipped
- **Embeddable**: Designed for embedding in applications
- **JSON-like**: Works with JSON-like data structures
- **Functional**: Built around functional programming concepts with piping

## Installation & Usage

### JavaScript
```bash
npm install mistql
```
```js
import mistql from 'mistql';
const result = mistql.query('events | filter type == "purchase" | count', data);
```

### Python
```bash
uv venv
uv pip install mistql
```
```py
import mistql
result = mistql.query('events | filter type == "purchase" | count', data)
```

### Command Line
```bash
npm install -g mistql
mq "count @" < data.json
```

## Core Concepts

### Context and Variables
- `@` refers to the current context (initially the input data)
- Object keys are automatically populated as variables in the namespace
- `$` provides access to builtin functions and root context

### Data Types
MistQL supports 8 core types:

| Type | External | Description |
|------|----------|-------------|
| string | ✓ | Unicode codepoints |
| number | ✓ | Doubles (no NaN/infinity) |
| boolean | ✓ | true/false |
| null | ✓ | null value |
| object | ✓ | String-keyed objects |
| array | ✓ | Can be non-homogeneous |
| function | | Internal functions |
| regex | | Regular expressions |

### Type Casting
- **To number**: Strings parsed as JSON numbers, booleans (1/0), null (0)
- **To string**: Numbers as base-10, booleans as "true"/"false", objects as JSON
- **Truthiness**: Empty strings/arrays/objects are falsy, everything else is truthy

## Syntax Overview

### Basic Operations
```mistql
# Arithmetic
1 + 5 * 2  # 11

# Logical
true && false  # false
!false  # true

# Comparisons
10 > 5  # true
"abc" == "abc"  # true
```

### Literals
```mistql
# JSON literals work directly
{"name": "John", "age": 30}
[1, 2, 3, {"nested": true}]
```

### Object Access
```mistql
# Dot notation
@.name
@.user.profile.email

# Bracket notation
@["name"]
@["user"]["profile"]["email"]
```

### Array/String Indexing
```mistql
# Zero-indexed
"hello"[0]  # "h"
[1, 2, 3][1]  # 2

# Negative indexing
"world"[-1]  # "d"

# Slicing
"hello"[1:4]  # "ell"
[1, 2, 3, 4][1:3]  # [2, 3]
```

### Function Calls
```mistql
# Lisp-like syntax
functionname arg1 arg2 arg3

# Examples
sort [3, 1, 2]  # [1, 2, 3]
index 1 "hello"  # "e"
```

### Piping
```mistql
# Chain operations
data | filter type == "purchase" | map amount | sum

# Equivalent to:
sum (map amount (filter (type == "purchase") data))
```

## Built-in Functions

### Array Operations
- `count`: Get array length
- `filter`: Filter array elements
- `map`: Transform array elements
- `find`: Find first matching element
- `reduce`: Reduce array to single value
- `sort`/`sortby`: Sort arrays
- `reverse`: Reverse array
- `flatten`: Flatten nested arrays
- `withindices`: Add indices to elements

### Object Operations
- `keys`: Get object keys
- `values`: Get object values
- `entries`: Get key-value pairs
- `fromentries`: Create object from entries
- `mapkeys`/`mapvalues`: Transform keys/values
- `filterkeys`/`filtervalues`: Filter by keys/values

### String Operations
- `split`: Split string by delimiter
- `stringjoin`: Join array of strings
- `replace`: Replace substrings
- `match`: Test regex patterns
- `regex`: Create regex objects

### Mathematical
- `sum`: Sum array of numbers
- `summarize`: Statistical summary (mean, median, stddev, etc.)
- `range`: Generate number ranges

### Utility
- `if`: Conditional expressions
- `log`: Debug logging
- `apply`: Apply function to value
- `string`/`float`: Type casting

## Contextualized Expressions

MistQL's core feature is contextualized expressions - expressions that execute with different contexts:

```mistql
# @ changes context in each iteration
[{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]
| filter age > 26  # @ is each object
| map name  # @ is each filtered object
```

Functions that use contextualized expressions:
- `filter`: @ becomes each array element
- `map`: @ becomes each array element
- `groupby`: @ becomes each array element
- `sortby`: @ becomes each array element

## Operators

### Unary Operators
- `!`: Logical NOT
- `-`: Unary minus

### Binary Operators
- `+`: Addition (numbers), concatenation (strings/arrays)
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division
- `%`: Modulo
- `<`, `>`, `<=`, `>=`: Comparisons
- `==`, `!=`: Equality
- `=~`: Regex matching
- `&&`: Logical AND (short-circuiting)
- `||`: Logical OR (short-circuiting)

### Precedence (highest to lowest)
1. `.` (dot access)
2. Unary `!`, `-`
3. `*`, `/`, `%`
4. `+`, `-`
5. `<`, `>`, `<=`, `>=`
6. `==`, `!=`, `=~`
7. `&&`
8. `||`
9. Function application
10. `|` (piping)

## Common Gotchas

1. **Unary minus ambiguity**: Use parentheses: `map (-cost)` not `map -cost`
2. **Indexing spacing**: No spaces between object and brackets: `arr[0]` not `arr [0]`
3. **Missing object keys**: Use `@.key` instead of bare `key` for safety
4. **Type conversion**: MistQL may convert types when crossing language boundaries

## Implementation Details

### JavaScript Implementation
- TypeScript-based
- 5.3kB gzipped target
- Supports custom functions via `MistQLInstance`
- Type mapping: `undefined` → `null`, `Date` → ISO string

### Python Implementation
- Python 3.8+ support
- Uses Poetry for dependency management
- Type mapping: `None` → `null`, `datetime` → ISO string

### Custom Functions (JS only)
```js
const mq = new MistQLInstance({
  extras: {
    customfunc: (a, b) => a + b
  }
});
```

## Development

### Project Structure
- `/docs`: Docusaurus documentation site
- `/js`: JavaScript/TypeScript implementation
- `/py`: Python implementation
- `/shared`: Language-independent test suite

### Testing
- Shared test suite in `/shared` directory
- Language-specific tests alongside implementations
- Extensive test coverage for cross-platform compatibility

### Contributing
- Fork and submit PRs to `main` branch
- No strict formatting requirements
- Coordinate new implementations via Discord
- Example PR: [Adding functions](https://github.com/evinism/mistql/pull/175)

## Resources

- **Documentation**: https://www.mistql.com/
- **Try it out**: https://www.mistql.com/tryitout
- **Discord**: https://discord.gg/YupxqvE5Jk
- **Grammar**: [Lark grammar file](https://github.com/evinism/mistql/blob/main/py/mistql/grammar.lark)
