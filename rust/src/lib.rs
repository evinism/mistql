//! MistQL - A miniature query language for performing computations on JSON-like structures
//!
//! This is the Rust implementation of MistQL, designed for embedding across multiple domains.
//! It serves as a powerful common expression language with strong cross-platform behavior semantics.

use crate::executor::ExecutionError;
pub mod builtins;
pub mod executor;
pub mod lexer;
pub mod parser;
pub mod types;

// Re-export commonly used types and functions
pub use types::RuntimeValue;

// Shared test modules.
#[cfg(test)]
mod tests;

// MistQL error types.
#[derive(Debug, thiserror::Error)]
pub enum MistQLError {
    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Type error: {0}")]
    Type(String),

    #[error("Reference error: {0}")]
    Reference(String),
}

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
pub fn query<T>(query_str: &str, data: T) -> Result<RuntimeValue, MistQLError>
where
    T: TryInto<RuntimeValue, Error = ExecutionError>,
{
    let runtime_data = data.try_into().map_err(|e| MistQLError::Runtime(e.to_string()))?;
    query_runtime(query_str, &runtime_data)
}

// Query function that works directly with RuntimeValue - preferred for internal use
pub fn query_runtime(query_str: &str, data: &RuntimeValue) -> Result<RuntimeValue, MistQLError> {
    use crate::executor::{execute_expression, ExecutionContext};
    use crate::parser::Parser;

    // Parse the query string into an expression
    let expr = Parser::parse(query_str).map_err(|e| MistQLError::Parser(e))?;

    // Create execution context with builtins
    let mut context = ExecutionContext::with_builtins(data.clone());

    // Execute the expression
    let result = execute_expression(&expr, &mut context).map_err(|e| MistQLError::Runtime(e.to_string()))?;

    Ok(result)
}
