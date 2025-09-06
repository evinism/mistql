//! Built-in functions for MistQL
//!
//! This module implements all the built-in functions that are available in MistQL,
//! including array operations, object operations, string operations, mathematical
//! functions, and utility functions.

use crate::types::{RuntimeValue, RuntimeValueType};
use crate::parser::Expression;
use crate::executor::{ExecutionContext, ExecutionError, execute_expression};
use std::collections::HashMap;

/// Type alias for built-in function implementations
pub type BuiltinFunction = fn(&[Expression], &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError>;

/// Validate the number of arguments for a builtin function
fn validate_args(name: &str, args: &[Expression], min_args: usize, max_args: Option<usize>) -> Result<(), ExecutionError> {
    if args.len() < min_args {
        return Err(ExecutionError::Custom(format!("{} takes at least {} arguments", name, min_args)));
    }
    if let Some(max) = max_args {
        if args.len() > max {
            return Err(ExecutionError::Custom(format!("{} takes at most {} arguments", name, max)));
        }
    }
    Ok(())
}

/// Assert that a value is of a specific type
fn assert_type(value: RuntimeValue, expected_type: RuntimeValueType) -> Result<RuntimeValue, ExecutionError> {
    if value.get_type() == expected_type {
        Ok(value)
    } else {
        Err(ExecutionError::TypeMismatch(format!("Expected {}, got {}", expected_type, value.get_type())))
    }
}

/// Assert that a value is a number and return it
fn assert_number(value: RuntimeValue) -> Result<f64, ExecutionError> {
    match value {
        RuntimeValue::Number(n) => Ok(n),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected number, got {}", value.get_type())))
    }
}

/// Assert that a value is an array and return it
fn assert_array(value: RuntimeValue) -> Result<Vec<RuntimeValue>, ExecutionError> {
    match value {
        RuntimeValue::Array(arr) => Ok(arr),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected array, got {}", value.get_type())))
    }
}

/// Assert that a value is an object and return it
fn assert_object(value: RuntimeValue) -> Result<HashMap<String, RuntimeValue>, ExecutionError> {
    match value {
        RuntimeValue::Object(obj) => Ok(obj),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected object, got {}", value.get_type())))
    }
}

/// Assert that a value is a string and return it
fn assert_string(value: RuntimeValue) -> Result<String, ExecutionError> {
    match value {
        RuntimeValue::String(s) => Ok(s),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected string, got {}", value.get_type())))
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Log function - prints the value and returns it
pub fn log(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("log", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;
    println!("MistQL: {}", value.to_string());
    Ok(value)
}

/// If function - conditional execution
pub fn if_function(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("if", args, 3, Some(3))?;

    let condition = execute_expression(&args[0], context)?;
    if condition.truthy() {
        execute_expression(&args[1], context)
    } else {
        execute_expression(&args[2], context)
    }
}

/// Apply function - applies a function to a value
pub fn apply(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("apply", args, 2, Some(2))?;

    let func = execute_expression(&args[0], context)?;
    let value = execute_expression(&args[1], context)?;

    match func {
        RuntimeValue::Function(func_name) => {
            // Create a function call expression with the value as argument
            let call_expr = Expression::FnExpression {
                function: Box::new(Expression::RefExpression {
                    name: func_name,
                    absolute: false
                }),
                arguments: vec![Expression::ValueExpression { value }]
            };
            execute_expression(&call_expr, context)
        }
        _ => Err(ExecutionError::NotCallable(func.get_type().to_string()))
    }
}

// ============================================================================
// ARRAY OPERATIONS
// ============================================================================

/// Count function - returns the length of an array
pub fn count(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("count", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;
    Ok(RuntimeValue::Number(array.len() as f64))
}

/// Filter function - filters array elements based on a condition
pub fn filter(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("filter", args, 2, Some(2))?;

    let condition = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut result = Vec::new();
    for item in array {
        // Push item as new context
        context.push_context(item.clone());

        // Execute condition with item as @ context
        let condition_result = execute_expression(condition, context)?;

        // Pop context
        context.pop_context()?;

        // If condition is truthy, include the item
        if condition_result.truthy() {
            result.push(item);
        }
    }

    Ok(RuntimeValue::Array(result))
}

/// Map function - transforms array elements
pub fn map(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("map", args, 2, Some(2))?;

    let transformation = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut result = Vec::new();
    for item in array {
        // Push item as new context
        context.push_context(item.clone());

        // Execute transformation with item as @ context
        let transformed = execute_expression(transformation, context)?;

        // Pop context
        context.pop_context()?;

        result.push(transformed);
    }

    Ok(RuntimeValue::Array(result))
}

/// Find function - finds the first element that matches a condition
pub fn find(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("find", args, 2, Some(2))?;

    let condition = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    for item in array {
        // Push item as new context
        context.push_context(item.clone());

        // Execute condition with item as @ context
        let condition_result = execute_expression(condition, context)?;

        // Pop context
        context.pop_context()?;

        // If condition is truthy, return the item
        if condition_result.truthy() {
            return Ok(item);
        }
    }

    Ok(RuntimeValue::Null)
}

/// Reverse function - reverses an array
pub fn reverse(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("reverse", args, 1, Some(1))?;
    let mut array = assert_array(execute_expression(&args[0], context)?)?;
    array.reverse();
    Ok(RuntimeValue::Array(array))
}

/// Flatten function - flattens nested arrays
pub fn flatten(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("flatten", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = Vec::new();
    for item in array {
        match item {
            RuntimeValue::Array(nested) => result.extend(nested),
            other => result.push(other),
        }
    }

    Ok(RuntimeValue::Array(result))
}

/// Sum function - sums all numbers in an array
pub fn sum(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("sum", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut total = 0.0;
    for item in array {
        total += assert_number(item)?;
    }

    Ok(RuntimeValue::Number(total))
}

// ============================================================================
// OBJECT OPERATIONS
// ============================================================================

/// Keys function - returns the keys of an object
pub fn keys(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("keys", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    let keys: Vec<RuntimeValue> = object.keys()
        .map(|k| RuntimeValue::String(k.clone()))
        .collect();

    Ok(RuntimeValue::Array(keys))
}

/// Values function - returns the values of an object
pub fn values(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("values", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    let values: Vec<RuntimeValue> = object.values().cloned().collect();
    Ok(RuntimeValue::Array(values))
}

/// Entries function - returns key-value pairs as an array of objects
pub fn entries(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("entries", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    let mut result = Vec::new();
    for (key, value) in object {
        let mut entry = HashMap::new();
        entry.insert("key".to_string(), RuntimeValue::String(key));
        entry.insert("value".to_string(), value);
        result.push(RuntimeValue::Object(entry));
    }

    Ok(RuntimeValue::Array(result))
}

// ============================================================================
// STRING OPERATIONS
// ============================================================================

/// Split function - splits a string by a delimiter
pub fn split(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("split", args, 2, Some(2))?;

    let delimiter = assert_string(execute_expression(&args[0], context)?)?;
    let string = assert_string(execute_expression(&args[1], context)?)?;

    let parts: Vec<RuntimeValue> = string
        .split(&delimiter)
        .map(|s| RuntimeValue::String(s.to_string()))
        .collect();

    Ok(RuntimeValue::Array(parts))
}

/// String join function - joins an array of strings
pub fn stringjoin(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("stringjoin", args, 2, Some(2))?;

    let delimiter = assert_string(execute_expression(&args[0], context)?)?;
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let strings: Result<Vec<String>, ExecutionError> = array
        .into_iter()
        .map(|item| assert_string(item))
        .collect();

    let strings = strings?;
    let result = strings.join(&delimiter);

    Ok(RuntimeValue::String(result))
}

// ============================================================================
// TYPE CONVERSION
// ============================================================================

/// String function - converts a value to a string
pub fn string(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("string", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;
    Ok(RuntimeValue::String(value.to_string()))
}

/// Float function - converts a value to a number
pub fn float(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("float", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;
    let num = value.to_float().map_err(|e| ExecutionError::TypeMismatch(e))?;
    Ok(RuntimeValue::Number(num))
}

// ============================================================================
// BUILTIN REGISTRATION
// ============================================================================

/// Get all built-in functions as a HashMap
pub fn get_builtins() -> HashMap<String, RuntimeValue> {
    let mut builtins = HashMap::new();

    // Utility functions
    builtins.insert("log".to_string(), RuntimeValue::Function("log".to_string()));
    builtins.insert("if".to_string(), RuntimeValue::Function("if".to_string()));
    builtins.insert("apply".to_string(), RuntimeValue::Function("apply".to_string()));

    // Array operations
    builtins.insert("count".to_string(), RuntimeValue::Function("count".to_string()));
    builtins.insert("filter".to_string(), RuntimeValue::Function("filter".to_string()));
    builtins.insert("map".to_string(), RuntimeValue::Function("map".to_string()));
    builtins.insert("find".to_string(), RuntimeValue::Function("find".to_string()));
    builtins.insert("reverse".to_string(), RuntimeValue::Function("reverse".to_string()));
    builtins.insert("flatten".to_string(), RuntimeValue::Function("flatten".to_string()));
    builtins.insert("sum".to_string(), RuntimeValue::Function("sum".to_string()));

    // Object operations
    builtins.insert("keys".to_string(), RuntimeValue::Function("keys".to_string()));
    builtins.insert("values".to_string(), RuntimeValue::Function("values".to_string()));
    builtins.insert("entries".to_string(), RuntimeValue::Function("entries".to_string()));

    // String operations
    builtins.insert("split".to_string(), RuntimeValue::Function("split".to_string()));
    builtins.insert("stringjoin".to_string(), RuntimeValue::Function("stringjoin".to_string()));

    // Type conversion
    builtins.insert("string".to_string(), RuntimeValue::Function("string".to_string()));
    builtins.insert("float".to_string(), RuntimeValue::Function("float".to_string()));

    builtins
}

/// Execute a built-in function by name
pub fn execute_builtin(name: &str, args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    match name {
        // Utility functions
        "log" => log(args, context),
        "if" => if_function(args, context),
        "apply" => apply(args, context),

        // Array operations
        "count" => count(args, context),
        "filter" => filter(args, context),
        "map" => map(args, context),
        "find" => find(args, context),
        "reverse" => reverse(args, context),
        "flatten" => flatten(args, context),
        "sum" => sum(args, context),

        // Object operations
        "keys" => keys(args, context),
        "values" => values(args, context),
        "entries" => entries(args, context),

        // String operations
        "split" => split(args, context),
        "stringjoin" => stringjoin(args, context),

        // Type conversion
        "string" => string(args, context),
        "float" => float(args, context),

        _ => Err(ExecutionError::Custom(format!("Unknown builtin function: {}", name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RuntimeValue;
    use crate::parser::Expression;
    use std::collections::HashMap;

    fn create_test_context() -> ExecutionContext {
        let data = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), RuntimeValue::String("John".to_string()));
            map.insert("age".to_string(), RuntimeValue::Number(30.0));
            map
        });

        let builtins = get_builtins();
        ExecutionContext::new(data, builtins)
    }

    #[test]
    fn test_count_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Array(vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::Number(3.0),
            ])
        }];

        let result = count(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(3.0));
    }

    #[test]
    fn test_filter_function() {
        let mut context = create_test_context();

        // Filter even numbers: filter (@ % 2 == 0) [1, 2, 3, 4, 5]
        let condition = Expression::BinaryExpression {
            operator: crate::parser::BinaryOperator::Eq,
            left: Box::new(Expression::BinaryExpression {
                operator: crate::parser::BinaryOperator::Mod,
                left: Box::new(Expression::RefExpression { name: "@".to_string(), absolute: false }),
                right: Box::new(Expression::ValueExpression { value: RuntimeValue::Number(2.0) })
            }),
            right: Box::new(Expression::ValueExpression { value: RuntimeValue::Number(0.0) })
        };

        let args = vec![
            condition,
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                    RuntimeValue::Number(4.0),
                    RuntimeValue::Number(5.0),
                ])
            }
        ];

        let result = filter(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], RuntimeValue::Number(2.0));
            assert_eq!(arr[1], RuntimeValue::Number(4.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_map_function() {
        let mut context = create_test_context();

        // Map to double: map (@ * 2) [1, 2, 3]
        let transformation = Expression::BinaryExpression {
            operator: crate::parser::BinaryOperator::Mul,
            left: Box::new(Expression::RefExpression { name: "@".to_string(), absolute: false }),
            right: Box::new(Expression::ValueExpression { value: RuntimeValue::Number(2.0) })
        };

        let args = vec![
            transformation,
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                ])
            }
        ];

        let result = map(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], RuntimeValue::Number(2.0));
            assert_eq!(arr[1], RuntimeValue::Number(4.0));
            assert_eq!(arr[2], RuntimeValue::Number(6.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_sum_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Array(vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::Number(3.0),
            ])
        }];

        let result = sum(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(6.0));
    }

    #[test]
    fn test_keys_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Object({
                let mut map = HashMap::new();
                map.insert("a".to_string(), RuntimeValue::Number(1.0));
                map.insert("b".to_string(), RuntimeValue::Number(2.0));
                map
            })
        }];

        let result = keys(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            // Keys order is not guaranteed, so we check that both keys are present
            let keys: Vec<String> = arr.into_iter()
                .map(|v| match v {
                    RuntimeValue::String(s) => s,
                    _ => panic!("Expected string key")
                })
                .collect();
            assert!(keys.contains(&"a".to_string()));
            assert!(keys.contains(&"b".to_string()));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_string_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Number(42.0)
        }];

        let result = string(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("42".to_string()));
    }

    #[test]
    fn test_if_function() {
        let mut context = create_test_context();

        let args = vec![
            Expression::ValueExpression { value: RuntimeValue::Boolean(true) },
            Expression::ValueExpression { value: RuntimeValue::String("yes".to_string()) },
            Expression::ValueExpression { value: RuntimeValue::String("no".to_string()) },
        ];

        let result = if_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("yes".to_string()));

        // Test false condition
        let args = vec![
            Expression::ValueExpression { value: RuntimeValue::Boolean(false) },
            Expression::ValueExpression { value: RuntimeValue::String("yes".to_string()) },
            Expression::ValueExpression { value: RuntimeValue::String("no".to_string()) },
        ];

        let result = if_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("no".to_string()));
    }
}
