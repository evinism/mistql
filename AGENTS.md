# MistQL: A Comprehensive Overview

## What is MistQL?

MistQL is a miniature, embedded query language designed for JSON-like data structures. It's built from the ground up to be lightweight, cross-platform, and easily embeddable across multiple domains. The language provides a powerful yet simple way to query and manipulate JSON-like data with a consistent, readable syntax.

## Key Differentiators from Other Query Languages

### 1. **Embedded Design Philosophy**
Unlike traditional query languages that are standalone systems, MistQL is designed to be embedded directly into applications. It has:
- **Minimal footprint**: JavaScript implementation is only 5.3kB gzipped
- **Zero dependencies**: No external runtime requirements
- **Cross-platform consistency**: Identical behavior across JavaScript and Python implementations

### 2. **Contextualized Expressions: The Core Innovation**
MistQL's most distinctive feature is its **contextualized expressions** system, which replaces traditional lambda functions with a more intuitive approach:

```javascript
// Traditional lambda approach (other languages)
data.filter(item => item.type === "purchase")
     .map(item => item.email)

// MistQL contextualized expressions
data | filter type == "purchase" | map email
```

**How it works:**
- The `@` variable represents the current context
- Functions that operate on collections push each item to the context stack
- Object keys are automatically populated as variables in scope
- This eliminates the need for explicit parameter declarations

### 3. **Context Population System**
MistQL automatically populates object keys as variables, making queries incredibly concise:

```javascript
// Data: {"name": "Alice", "age": 30, "city": "NYC"}

// Access via automatic context population
name + " is " + age + " years old from " + city

// Equivalent to traditional approach
@.name + " is " + @.age + " years old from " + @.city
```

### 4. **Pipeline-First Architecture**
MistQL is built around a powerful pipeline system that makes data transformations intuitive:

```javascript
// Complex data transformation in a single pipeline
events 
  | filter type == "purchase" 
  | groupby email 
  | mapvalues count 
  | filter @ > 5
  | keys
```

**Pipeline mechanics:**
- Each stage processes the output of the previous stage
- The `|` operator chains functions together
- Functions are designed to work seamlessly in pipelines
- The `apply` function allows custom expressions in pipelines

## Architecture & Implementation

### JavaScript Implementation (`/js`)
- **TypeScript-based**: Full type safety with comprehensive type definitions
- **Rollup bundling**: Optimized for both UMD and ESM formats
- **Zero dependencies**: Self-contained implementation
- **Browser-optimized**: Designed for client-side applications

### Python Implementation (`/py`)
- **Lark grammar**: Uses Lark parser for robust syntax parsing
- **Type checking**: Full typeguard integration for runtime safety
- **Poetry management**: Modern Python packaging with uv.lock
- **Cross-version compatibility**: Supports Python 3.8.1+

### Shared Architecture
Both implementations share:
- **Identical behavior**: Language-independent test suite ensures consistency
- **Common grammar**: Lark grammar file defines syntax
- **Function parity**: All built-in functions behave identically
- **Error handling**: Consistent error messages and behavior

## Core Language Features

### Type System
MistQL has 8 core data types:
- **Primitive types**: `string`, `number`, `boolean`, `null`
- **Complex types**: `object`, `array`
- **Internal types**: `function`, `regex`

**Type safety features:**
- Strict equality (different types always compare as false)
- Automatic type casting where safe
- Comprehensive type checking and validation

### Built-in Functions
MistQL provides a rich set of built-in functions optimized for data manipulation:

**Collection operations:**
- `filter`, `map`, `reduce`, `groupby`
- `count`, `sort`, `unique`, `flatten`

**Object operations:**
- `keys`, `values`, `entries`, `mapkeys`, `mapvalues`

**String operations:**
- `split`, `join`, `match`, `replace`

**Utility functions:**
- `if`, `apply`, `index`, `slice`

### Operator System
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `&&`, `||`, `!`
- **Pattern matching**: `=~` for regex matching
- **Pipeline**: `|` for function chaining

## Advanced Features

### 1. **Root Variable (`$`)**
Provides access to built-in functions and original data:
```javascript
// Access built-ins even when context conflicts
$.map  // Always refers to built-in map function
$.@    // Always refers to original input data
```

### 2. **Garden Wall System**
Automatic input/output sanitization:
- **Input sanitization**: Converts platform-specific types to MistQL types
- **Output sanitization**: Ensures only safe types are returned
- **Error prevention**: Handles edge cases gracefully

### 3. **Custom Function Extensions**
Both implementations support custom function registration:
```javascript
// JavaScript
const instance = new MistQLInstance({
  extras: {
    customFn: (data) => data.length * 2
  }
});

// Python
instance = MistQLInstance(extras={
    "custom_fn": lambda data: len(data) * 2
})
```

## Use Cases & Applications

### 1. **Client-Side Data Processing**
- Real-time data filtering and transformation
- User interface data manipulation
- Form validation and processing

### 2. **API Response Processing**
- Data transformation before display
- Response filtering and aggregation
- Dynamic data restructuring

### 3. **Configuration Processing**
- Dynamic configuration evaluation
- Template-based data generation
- Conditional logic in configurations

### 4. **Embedded Analytics**
- Simple data analysis queries
- Real-time metrics calculation
- Dynamic report generation

## Performance Characteristics

### JavaScript Implementation
- **Bundle size**: 5.3kB gzipped
- **Runtime performance**: Optimized for browser environments
- **Memory usage**: Minimal memory footprint
- **Startup time**: Instant initialization

### Python Implementation
- **Import time**: Fast module loading
- **Memory efficiency**: Optimized for server environments
- **Parsing speed**: Efficient Lark-based parsing
- **Type checking**: Runtime type validation with minimal overhead

## Comparison with Alternatives

### vs. JSONPath
- **More expressive**: Full programming language capabilities
- **Better performance**: Optimized for common use cases
- **Cleaner syntax**: Pipeline-based approach vs. path expressions

### vs. JQ
- **Lighter weight**: Smaller footprint and faster startup
- **Better embedding**: Designed for application integration
- **Consistent behavior**: Identical across platforms

### vs. Custom Solutions
- **Standardized**: Well-tested, documented language
- **Maintainable**: Clear syntax and semantics
- **Extensible**: Built-in extension mechanisms

## Development & Ecosystem

### Current Status
- **Version**: 0.4.12 (stable)
- **License**: MIT
- **Community**: Active Discord community
- **Documentation**: Comprehensive docs at mistql.com

### Roadmap
- **0.5.0**: Planned standardization release
- **Language specification**: Formal ABNF specification
- **Additional implementations**: Community-driven ports
- **Performance optimizations**: Ongoing improvements

### Contributing
- **Open source**: MIT licensed with active development
- **Test-driven**: Comprehensive test suite ensures quality
- **Cross-platform**: Contributions benefit all implementations
- **Community-driven**: Active maintainer and contributor base

## Conclusion

MistQL represents a unique approach to embedded data querying that prioritizes:
1. **Simplicity**: Clean, readable syntax that's easy to learn
2. **Performance**: Minimal footprint with maximum capability
3. **Consistency**: Identical behavior across all platforms
4. **Embeddability**: Designed to integrate seamlessly into applications

The language's contextualized expressions and pipeline-first architecture make it particularly well-suited for scenarios where traditional query languages would be overkill, while still providing the power and flexibility needed for complex data transformations.

For developers looking to add data querying capabilities to their applications without the overhead of full database systems or complex query engines, MistQL offers an elegant, performant solution that scales from simple lookups to complex data processing pipelines. 