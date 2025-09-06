//! MistQL - A miniature query language for performing computations on JSON-like structures
//!
//! This is the Rust implementation of MistQL, designed for embedding across multiple domains.
//! It serves as a powerful common expression language with strong cross-platform behavior semantics.

pub mod types;
pub mod lexer;
pub mod parser;
pub mod executor;
pub mod builtins;
pub mod instance;
pub mod errors;

// Test modules
#[cfg(test)]
mod tests;

// Test modules are integrated into their respective source files

/// Main query function - the primary entry point for MistQL queries
///
/// # Examples
///
/// ```rust,no_run
/// use mistql::query;
///
/// let data = serde_json::json!([{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]);
/// let result = query("filter age > 26 | map name", &data).unwrap();
/// ```
pub fn query(query_str: &str, data: &serde_json::Value) -> Result<serde_json::Value, errors::MistQLError> {
    use crate::parser::{Expression, Parser};
    use crate::executor::{execute_expression, ExecutionContext};
    use crate::types::RuntimeValue;

    // Parse the query string into an expression
    let expr = Parser::parse(query_str)
        .map_err(|e| errors::MistQLError::Parser(e))?;

    // Convert serde_json::Value to RuntimeValue
    let runtime_data = RuntimeValue::from_serde_value(data);

    // Create execution context with builtins
    let mut context = ExecutionContext::with_builtins(runtime_data);

    // Execute the expression
    let result = execute_expression(&expr, &mut context)
        .map_err(|e| errors::MistQLError::Runtime(e.to_string()))?;

    // Convert RuntimeValue back to serde_json::Value
    Ok(result.to_serde_value())
}
