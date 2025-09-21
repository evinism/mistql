//! Built-in functions for MistQL
//!
//! This module implements all the built-in functions that are available in MistQL,
//! including array operations, object operations, string operations, mathematical
//! functions, and utility functions.

use crate::executor::{execute_expression, ExecutionContext, ExecutionError};
use crate::parser::Expression;
use crate::types::RuntimeValue;
use std::collections::HashMap;

// Validate the number of arguments for a builtin function.
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

// Assert that a value is a number and return it.
fn assert_number(value: RuntimeValue) -> Result<f64, ExecutionError> {
    match value {
        RuntimeValue::Number(n) => Ok(n),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected number, got {}", value.get_type()))),
    }
}

// Assert that a value is an array and return it.
fn assert_array(value: RuntimeValue) -> Result<Vec<RuntimeValue>, ExecutionError> {
    match value {
        RuntimeValue::Array(arr) => Ok(arr),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected array, got {}", value.get_type()))),
    }
}

// Assert that a value is an object and return it.
fn assert_object(value: RuntimeValue) -> Result<HashMap<String, RuntimeValue>, ExecutionError> {
    match value {
        RuntimeValue::Object(obj) => Ok(obj),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected object, got {}", value.get_type()))),
    }
}

// Assert that a value is a string and return it.
fn assert_string(value: RuntimeValue) -> Result<String, ExecutionError> {
    match value {
        RuntimeValue::String(s) => Ok(s),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected string, got {}", value.get_type()))),
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

// Log function - prints the value and returns it.
pub fn log(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("log", args, 1, Some(1))?;

    let value = execute_expression(&args[0], context)?;
    println!("MistQL: {}", value.to_string_serialize());
    Ok(value)
}

// If function - conditional execution.
pub fn if_function(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("if", args, 3, Some(3))?;

    let condition = execute_expression(&args[0], context)?;
    if condition.truthy() {
        execute_expression(&args[1], context)
    } else {
        execute_expression(&args[2], context)
    }
}

// Apply function - applies an expression to a value (sets @ context).
pub fn apply(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("apply", args, 2, Some(2))?;

    let expression = &args[0];
    let value = execute_expression(&args[1], context)?;
    context.push_context(value);

    let result = execute_expression(expression, context);
    let _ = context.pop_context();
    result
}

// ============================================================================
// ARRAY OPERATIONS
// ============================================================================

// Return the length of an array.
pub fn count(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("count", args, 1, Some(1))?;

    let array = assert_array(execute_expression(&args[0], context)?)?;
    Ok(RuntimeValue::Number(array.len() as f64))
}

// Filter array elements based on a condition.
pub fn filter(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("filter", args, 2, Some(2))?;

    let condition = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut result = Vec::new();
    for item in array {
        context.push_context(item.clone());
        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            result.push(item);
        }
    }

    Ok(RuntimeValue::Array(result))
}

// Transform array elements.
pub fn map(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("map", args, 2, Some(2))?;

    let transformation = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut result = Vec::new();
    for item in array {
        context.push_context(item.clone());
        let transformed = execute_expression(transformation, context)?;
        context.pop_context()?;
        result.push(transformed);
    }

    Ok(RuntimeValue::Array(result))
}

// Find the first element that matches a condition.
pub fn find(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("find", args, 2, Some(2))?;

    let condition = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    for item in array {
        context.push_context(item.clone());
        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            return Ok(item);
        }
    }

    Ok(RuntimeValue::Null)
}

// Reverse an array.
pub fn reverse(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("reverse", args, 1, Some(1))?;

    let mut array = assert_array(execute_expression(&args[0], context)?)?;
    array.reverse();
    Ok(RuntimeValue::Array(array))
}

// Flatten nested arrays.
pub fn flatten(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("flatten", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = Vec::new();
    for item in array {
        match item {
            RuntimeValue::Array(nested) => result.extend(nested),
            _ => {
                return Err(ExecutionError::TypeMismatch(format!(
                    "flatten: all elements must be arrays, got {}",
                    item.get_type()
                )))
            }
        }
    }

    Ok(RuntimeValue::Array(result))
}

// Sum all numbers in an array.
pub fn sum(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("sum", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;

    let array = match value {
        RuntimeValue::Array(arr) => arr,
        _ => return Err(ExecutionError::TypeMismatch(format!("Expected array, got {}", value.get_type()))),
    };

    let mut total = 0.0;
    for item in array {
        let num = match item {
            RuntimeValue::Number(n) => n,
            _ => {
                return Err(ExecutionError::TypeMismatch(format!(
                    "sum: all elements must be numbers, got {}",
                    item.get_type()
                )))
            }
        };
        total += num;
    }

    Ok(RuntimeValue::Number(total))
}

// Sort an array in ascending order.
pub fn sort(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("sort", args, 1, Some(1))?;
    let mut array = assert_array(execute_expression(&args[0], context)?)?;

    if array.is_empty() {
        return Ok(RuntimeValue::Array(array));
    }

    let first_type = array[0].get_type();
    for item in &array {
        if item.get_type() != first_type {
            return Err(ExecutionError::TypeMismatch(format!(
                "sort: cannot sort non-homogeneous arrays (found {} and {})",
                first_type,
                item.get_type()
            )));
        }
        if !item.comparable() {
            return Err(ExecutionError::Custom("sort: Cannot sort non-comparable values".to_string()));
        }
    }

    array.sort_by(|a, b| a.compare(b).unwrap_or(std::cmp::Ordering::Equal));

    Ok(RuntimeValue::Array(array))
}

// Sort an array by a key expression.
pub fn sortby(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("sortby", args, 2, Some(2))?;

    let key_expr = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut with_key: Vec<(RuntimeValue, RuntimeValue)> = Vec::new();
    for item in array {
        context.push_context(item.clone());
        let key = execute_expression(key_expr, context)?;
        context.pop_context()?;

        if !key.comparable() {
            return Err(ExecutionError::Custom("sortby: Cannot sort non-comparable values".to_string()));
        }

        with_key.push((key, item));
    }

    with_key.sort_by(|a, b| a.0.compare(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let result: Vec<RuntimeValue> = with_key.into_iter().map(|(_, value)| value).collect();

    Ok(RuntimeValue::Array(result))
}

// Reduce an array to a single value.
pub fn reduce(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("reduce", args, 3, Some(3))?;

    let reducer_expr = &args[0];
    let initial = execute_expression(&args[1], context)?;
    let array = assert_array(execute_expression(&args[2], context)?)?;

    let mut accumulator = initial;
    for item in array {
        let acc_cur = RuntimeValue::Array(vec![accumulator.clone(), item]);
        context.push_context(acc_cur);
        accumulator = execute_expression(reducer_expr, context)?;
        context.pop_context()?;
    }

    Ok(accumulator)
}

// Group array elements by a key expression.
pub fn groupby(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("groupby", args, 2, Some(2))?;

    let key_expr = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut groups: HashMap<String, Vec<RuntimeValue>> = HashMap::new();

    for item in array {
        context.push_context(item.clone());
        let key = execute_expression(key_expr, context)?;
        context.pop_context()?;
        // Dispatch to string_display to handle nested objects and arrays.
        // Otherwise use string_display to handle strings.
        match key {
            RuntimeValue::Object(_) => {
                groups.entry(key.to_string_display()).or_insert_with(Vec::new).push(item);
            }
            RuntimeValue::Array(_) => {
                groups.entry(key.to_string_display()).or_insert_with(Vec::new).push(item);
            }
            _ => {
                groups.entry(key.to_string_display()).or_insert_with(Vec::new).push(item);
            }
        }
    }

    // Convert HashMap to RuntimeValue::Object
    let mut result_obj = HashMap::new();
    for (key, values) in groups {
        result_obj.insert(key, RuntimeValue::Array(values));
    }

    Ok(RuntimeValue::Object(result_obj))
}

// Add indices to array elements.
pub fn withindices(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("withindices", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = Vec::new();
    for (index, item) in array.into_iter().enumerate() {
        let index_value = RuntimeValue::Number(index as f64);
        let pair = RuntimeValue::Array(vec![index_value, item]);
        result.push(pair);
    }

    Ok(RuntimeValue::Array(result))
}

// Find subsequences satisfying conditions.
pub fn sequence(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    if args.len() < 2 {
        return Err(ExecutionError::Custom("sequence takes at least 2 arguments".to_string()));
    }

    let predicates = &args[..args.len() - 1];
    let array = assert_array(execute_expression(&args[args.len() - 1], context)?)?;

    // Create bitmasks for each predicate.
    let mut bitmasks: Vec<Vec<bool>> = Vec::new();
    for predicate in predicates {
        let mut bitmask = Vec::new();
        for item in &array {
            context.push_context(item.clone());
            let result = execute_expression(predicate, context)?;
            context.pop_context()?;
            bitmask.push(result.truthy());
        }
        bitmasks.push(bitmask);
    }

    // Find all valid subsequences.
    let indices_map = sequence_helper(&bitmasks, 0);

    // Convert indices to actual values.
    let mut result = Vec::new();
    for indices in indices_map {
        let mut subsequence = Vec::new();
        for idx in indices {
            if idx < array.len() {
                subsequence.push(array[idx].clone());
            }
        }
        result.push(RuntimeValue::Array(subsequence));
    }

    Ok(RuntimeValue::Array(result))
}

// Recursively find valid subsequences.
fn sequence_helper(bitmasks: &[Vec<bool>], start: usize) -> Vec<Vec<usize>> {
    if bitmasks.is_empty() {
        return vec![];
    }

    let first_array = &bitmasks[0];
    let mut result = Vec::new();

    for idx in start..first_array.len() {
        if first_array[idx] {
            if bitmasks.len() == 1 {
                result.push(vec![idx]);
            } else {
                let sub_result = sequence_helper(&bitmasks[1..], idx + 1);
                for mut sub_indices in sub_result {
                    sub_indices.insert(0, idx);
                    result.push(sub_indices);
                }
            }
        }
    }

    result
}

// ============================================================================
// OBJECT OPERATIONS
// ============================================================================

// Return the keys of an object.
pub fn keys(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("keys", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    // Sort keys to match JavaScript behavior.
    let mut sorted_keys: Vec<String> = object.keys().cloned().collect();
    sorted_keys.sort();

    let keys: Vec<RuntimeValue> = sorted_keys.iter().map(|k| RuntimeValue::String(k.clone())).collect();

    Ok(RuntimeValue::Array(keys))
}

// Return the values of an object.
pub fn values(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("values", args, 1, Some(1))?;
    let target = execute_expression(&args[0], context)?;

    match target {
        RuntimeValue::Object(object) => {
            // Sort keys to match JavaScript behavior.
            let mut sorted_keys: Vec<String> = object.keys().cloned().collect();
            sorted_keys.sort();

            let values: Vec<RuntimeValue> = sorted_keys.iter().map(|key| object[key].clone()).collect();
            Ok(RuntimeValue::Array(values))
        }
        _ => {
            // For non-objects, return empty array (matches Python behavior)
            Ok(RuntimeValue::Array(vec![]))
        }
    }
}

// Return key-value pairs as an array of [key, value] arrays.
pub fn entries(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("entries", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    // Sort keys to match JavaScript behavior.
    let mut sorted_keys: Vec<String> = object.keys().cloned().collect();
    sorted_keys.sort();

    let mut result = Vec::new();
    for key in sorted_keys {
        let value = object[&key].clone();
        let entry = RuntimeValue::Array(vec![RuntimeValue::String(key), value]);
        result.push(entry);
    }

    Ok(RuntimeValue::Array(result))
}

// Create an object from an array of [key, value] pairs.
pub fn fromentries(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("fromentries", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = HashMap::new();
    for entry in array {
        let entry_array = assert_array(entry)?;

        // TODO: Is this correct?
        let key = if entry_array.len() > 0 {
            entry_array[0].to_string_display()
        } else {
            "null".to_string()
        };

        // TODO: Is this correct?
        let value = if entry_array.len() > 1 {
            entry_array[1].clone()
        } else {
            RuntimeValue::Null
        };

        result.insert(key, value);
    }

    Ok(RuntimeValue::Object(result))
}

// Map object keys.
pub fn mapkeys(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("mapkeys", args, 2, Some(2))?;

    let transformation = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object {
        context.push_context(RuntimeValue::String(key.clone()));
        let new_key = execute_expression(transformation, context)?;
        context.pop_context()?;
        // Dispatch to string_serialize to handle nested objects and arrays.
        // Otherwise use string_display to handle strings.
        match new_key {
            RuntimeValue::Object(_) => {
                result.insert(new_key.to_string_serialize(), value);
            }
            RuntimeValue::Array(_) => {
                result.insert(new_key.to_string_serialize(), value);
            }
            _ => {
                result.insert(new_key.to_string_display(), value);
            }
        }
    }

    Ok(RuntimeValue::Object(result))
}

// Map object values.
pub fn mapvalues(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("mapvalues", args, 2, Some(2))?;

    let transformation = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object {
        context.push_context(value.clone());
        let new_value = execute_expression(transformation, context)?;
        context.pop_context()?;
        result.insert(key, new_value);
    }

    Ok(RuntimeValue::Object(result))
}

// Filter object keys based on a condition.
pub fn filterkeys(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("filterkeys", args, 2, Some(2))?;

    let condition = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object {
        context.push_context(RuntimeValue::String(key.clone()));
        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            result.insert(key, value);
        }
    }

    Ok(RuntimeValue::Object(result))
}

// Filter object values based on a condition.
pub fn filtervalues(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("filtervalues", args, 2, Some(2))?;

    let condition = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object {
        context.push_context(value.clone());
        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            result.insert(key, value);
        }
    }

    Ok(RuntimeValue::Object(result))
}

// ============================================================================
// STRING OPERATIONS
// ============================================================================

// Split a string by a delimiter.
pub fn split(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("split", args, 2, Some(2))?;

    let delimiter_value = execute_expression(&args[0], context)?;
    let string = assert_string(execute_expression(&args[1], context)?)?;

    let parts: Vec<RuntimeValue> = match delimiter_value {
        RuntimeValue::String(delimiter) => {
            if delimiter.is_empty() {
                // Special case: empty delimiter splits into individual characters.
                string.chars().map(|c| RuntimeValue::String(c.to_string())).collect()
            } else {
                string.split(&delimiter).map(|s| RuntimeValue::String(s.to_string())).collect()
            }
        }
        RuntimeValue::Regex(regex) => {
            let compiled_regex = regex.as_regex();
            compiled_regex.split(&string).map(|s| RuntimeValue::String(s.to_string())).collect()
        }
        _ => {
            return Err(ExecutionError::TypeMismatch(format!(
                "Expected string or regex for split delimiter, got {}",
                delimiter_value.get_type()
            )));
        }
    };

    Ok(RuntimeValue::Array(parts))
}

// Join an array of strings.
pub fn stringjoin(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("stringjoin", args, 2, Some(2))?;

    let delimiter = assert_string(execute_expression(&args[0], context)?)?;
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let strings: Result<Vec<String>, ExecutionError> = array.into_iter().map(|item| assert_string(item)).collect();

    let strings = strings?;
    let result = strings.join(&delimiter);

    Ok(RuntimeValue::String(result))
}

// Replace substrings in a string.
pub fn replace(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("replace", args, 3, Some(3))?;

    let pattern = execute_expression(&args[0], context)?;
    let replacement = assert_string(execute_expression(&args[1], context)?)?;
    let target = assert_string(execute_expression(&args[2], context)?)?;

    match pattern {
        RuntimeValue::String(pattern_str) => {
            // Replace first occurrence only.
            let result = if let Some(pos) = target.find(&pattern_str) {
                let mut result = target.clone();
                result.replace_range(pos..pos + pattern_str.len(), &replacement);
                result
            } else {
                target.clone()
            };
            Ok(RuntimeValue::String(result))
        }
        RuntimeValue::Regex(regex_obj) => {
            let compiled_regex = regex_obj.as_regex();
            let result = if regex_obj.flags().contains('g') {
                // Global.
                compiled_regex.replace_all(&target, &replacement).to_string()
            } else {
                // First occurrence only.
                compiled_regex.replace(&target, &replacement).to_string()
            };
            Ok(RuntimeValue::String(result))
        }
        _ => Err(ExecutionError::TypeMismatch("replace: pattern must be string or regex".to_string())),
    }
}

// Test if a string matches a pattern.
pub fn match_function(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("match", args, 2, Some(2))?;

    let pattern = execute_expression(&args[0], context)?;
    let target = assert_string(execute_expression(&args[1], context)?)?;

    let matches = match pattern {
        RuntimeValue::String(pattern_str) => {
            // String arguments should be treated as regexes.
            match regex::Regex::new(&pattern_str) {
                Ok(regex) => regex.is_match(&target),
                Err(_) => return Err(ExecutionError::Custom(format!("Invalid regex pattern: {}", pattern_str))),
            }
        }
        RuntimeValue::Regex(regex_obj) => {
            // Regex matching.
            regex_obj.as_regex().is_match(&target)
        }
        _ => return Err(ExecutionError::TypeMismatch("match: pattern must be string or regex".to_string())),
    };

    Ok(RuntimeValue::Boolean(matches))
}

// Create a regex object.
pub fn regex(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("regex", args, 1, Some(2))?;

    let pattern = assert_string(execute_expression(&args[0], context)?)?;
    let flags = if args.len() == 2 {
        assert_string(execute_expression(&args[1], context)?)?
    } else {
        String::new()
    };

    let regex_obj =
        crate::types::MistQLRegex::new(&pattern, &flags).map_err(|e| ExecutionError::Custom(format!("Invalid regex: {}", e)))?;

    Ok(RuntimeValue::Regex(regex_obj))
}

// ============================================================================
// TYPE CONVERSION
// ============================================================================

// Convert a value to a string.
pub fn string(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("string", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;

    match value {
        RuntimeValue::String(s) => Ok(RuntimeValue::String(s)),
        RuntimeValue::Function(_) => Err(ExecutionError::TypeMismatch("Cannot cast function to string".to_string())),
        RuntimeValue::Regex(_) => Err(ExecutionError::TypeMismatch("Cannot cast regex to string".to_string())),
        _ => Ok(RuntimeValue::String(value.to_string_serialize())),
    }
}

// Convert a value to a number.
pub fn float(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("float", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;

    let num = match value {
        RuntimeValue::Number(n) => Ok(n),
        RuntimeValue::String(s) => {
            // Trim whitespace before parsing
            let trimmed = s.trim();
            trimmed.parse::<f64>().map_err(|_| format!("Cannot convert string to float: {}", s))
        }
        RuntimeValue::Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
        RuntimeValue::Null => Ok(0.0),
        _ => Err(format!("Cannot convert {} to float", value.get_type())),
    };

    let num = num.map_err(|e| ExecutionError::TypeMismatch(e))?;
    Ok(RuntimeValue::Number(num))
}

// ============================================================================
// MATHEMATICAL FUNCTIONS
// ============================================================================

// Generate a range of numbers.
pub fn range(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    if args.len() < 1 || args.len() > 3 {
        return Err(ExecutionError::Custom("range takes 1-3 arguments".to_string()));
    }

    let validate_integer = |value: RuntimeValue, arg_name: &str| -> Result<i64, ExecutionError> {
        match value {
            RuntimeValue::Number(n) => {
                if n.fract() == 0.0 {
                    Ok(n as i64)
                } else {
                    Err(ExecutionError::TypeMismatch(format!(
                        "range: {} must be an integer, got {}",
                        arg_name, n
                    )))
                }
            }
            _ => Err(ExecutionError::TypeMismatch(format!(
                "range: {} must be a number, got {}",
                arg_name,
                value.get_type()
            ))),
        }
    };

    let start: i64;
    let stop: i64;
    let step: i64;

    if args.len() == 1 {
        // range(stop) -> start=0, step=1
        start = 0;
        stop = validate_integer(execute_expression(&args[0], context)?, "stop")?;
        step = 1;
    } else if args.len() == 2 {
        // range(start, stop) -> step=1
        start = validate_integer(execute_expression(&args[0], context)?, "start")?;
        stop = validate_integer(execute_expression(&args[1], context)?, "stop")?;
        step = 1;
    } else {
        // range(start, stop, step)
        start = validate_integer(execute_expression(&args[0], context)?, "start")?;
        stop = validate_integer(execute_expression(&args[1], context)?, "stop")?;
        step = validate_integer(execute_expression(&args[2], context)?, "step")?;
    }

    // Handle negative step sizes (backward ranges).
    if step < 0 {
        if start <= stop {
            return Ok(RuntimeValue::Array(vec![]));
        }
        let mut result = Vec::new();
        let mut current = start;
        while current > stop {
            result.push(RuntimeValue::Number(current as f64));
            current += step;
        }
        return Ok(RuntimeValue::Array(result));
    }

    // Validate positive step.
    if step <= 0 {
        return Err(ExecutionError::Custom("range: step must be greater than 0".to_string()));
    }

    // Check if step direction matches stop - start direction.
    if (stop - start).signum() != step.signum() && stop != start {
        return Ok(RuntimeValue::Array(vec![]));
    }

    let mut result = Vec::new();
    let mut current = start;
    while current < stop {
        result.push(RuntimeValue::Number(current as f64));
        current += step;
    }

    Ok(RuntimeValue::Array(result))
}

// Provide a statistical summary of numbers.
pub fn summarize(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    validate_args("summarize", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    if array.is_empty() {
        return Err(ExecutionError::Custom("summarize: cannot summarize empty array".to_string()));
    }

    let numbers: Result<Vec<f64>, ExecutionError> = array.iter().map(|item| assert_number(item.clone())).collect();
    let numbers = numbers?;

    if numbers.len() < 2 {
        return Err(ExecutionError::Custom(
            "summarize: requires at least 2 numbers for variance calculation".to_string(),
        ));
    }

    // Calculate statistics.
    let max = numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;

    // Calculate median.
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted_numbers.len() % 2 == 0 {
        let mid = sorted_numbers.len() / 2;
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2.0
    } else {
        sorted_numbers[sorted_numbers.len() / 2]
    };

    // Calculate variance and standard deviation.
    let variance = numbers.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (numbers.len() - 1) as f64;
    let stddev = variance.sqrt();

    // Create result object.
    let mut result = HashMap::new();
    result.insert("max".to_string(), RuntimeValue::Number(max));
    result.insert("min".to_string(), RuntimeValue::Number(min));
    result.insert("mean".to_string(), RuntimeValue::Number(mean));
    result.insert("median".to_string(), RuntimeValue::Number(median));
    result.insert("variance".to_string(), RuntimeValue::Number(variance));
    result.insert("stddev".to_string(), RuntimeValue::Number(stddev));

    Ok(RuntimeValue::Object(result))
}

// ============================================================================
// INDEXING FUNCTION
// ============================================================================

pub fn index(args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    if args.len() == 2 {
        // Single index: index(index, array/object/string)
        let index_val = execute_expression(&args[0], context)?;
        let operand = execute_expression(&args[1], context)?;
        index_single(index_val, operand)
    } else if args.len() == 3 {
        // Slice: index(start, end, array/string)
        let start_val = execute_expression(&args[0], context)?;
        let end_val = execute_expression(&args[1], context)?;
        let operand = execute_expression(&args[2], context)?;
        index_double(start_val, end_val, operand)
    } else {
        Err(ExecutionError::Custom("index: takes 2 or 3 arguments".to_string()))
    }
}

// Single index operation: index(index, array/object/string)
fn index_single(index: RuntimeValue, operand: RuntimeValue) -> Result<RuntimeValue, ExecutionError> {
    match operand {
        RuntimeValue::Array(ref arr) => {
            let index_num = assert_number(index)?;
            if index_num.fract() != 0.0 {
                return Err(ExecutionError::Custom("index: Non-integers cannot be used on arrays".to_string()));
            };

            let len = arr.len() as isize;
            let actual_index: isize;
            // Handle negative indices.
            if index_num < 0.0 {
                actual_index = len + index_num as isize;
            } else {
                actual_index = index_num as isize;
            }

            // Handle regular and out of bounds.
            Ok(arr.get(actual_index as usize).cloned().unwrap_or(RuntimeValue::Null))
        }
        RuntimeValue::String(ref s) => {
            let index_num = assert_number(index)?;
            if index_num.fract() != 0.0 {
                return Err(ExecutionError::Custom("index: Non-integers cannot be used on strings".to_string()));
            }

            let len = s.len() as isize;
            let actual_index: isize;
            if index_num < 0.0 {
                actual_index = len + index_num as isize;
            } else {
                actual_index = index_num as isize;
            }

            Ok(s.chars()
                .nth(actual_index as usize)
                .map(|c| RuntimeValue::String(c.to_string()))
                .unwrap_or(RuntimeValue::Null))
        }
        RuntimeValue::Object(obj) => {
            let key = assert_string(index)?;
            Ok(obj.get(&key).cloned().unwrap_or(RuntimeValue::Null))
        }
        RuntimeValue::Null => {
            // Indexing null returns null, but only if index is number or string.
            match index {
                RuntimeValue::Number(_) | RuntimeValue::String(_) => Ok(RuntimeValue::Null),
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Expected number or string, got {}",
                    index.get_type()
                ))),
            }
        }
        _ => Err(ExecutionError::Custom(format!(
            "index: Cannot index {:?} with {:?}",
            operand.get_type(),
            index.get_type()
        ))),
    }
}

// Double index operation (slicing): index(start, end, array/string)
fn index_double(start: RuntimeValue, end: RuntimeValue, operand: RuntimeValue) -> Result<RuntimeValue, ExecutionError> {
    match operand {
        RuntimeValue::Array(ref arr) => {
            let start_idx = match start {
                RuntimeValue::Number(n) => n as isize,
                RuntimeValue::Null => 0,
                _ => {
                    return Err(ExecutionError::TypeMismatch(format!(
                        "Slice start must be number or null, got {:?}",
                        start.get_type()
                    )))
                }
            };

            let end_idx = match end {
                RuntimeValue::Number(n) => n as isize,
                RuntimeValue::Null => arr.len() as isize,
                _ => {
                    return Err(ExecutionError::TypeMismatch(format!(
                        "Slice end must be number or null, got {:?}",
                        end.get_type()
                    )))
                }
            };

            let len = arr.len() as isize;
            let start_idx = if start_idx < 0 { len + start_idx } else { start_idx };
            let end_idx = if end_idx < 0 { len + end_idx } else { end_idx };

            let start_idx = start_idx.clamp(0, len) as usize;
            let end_idx = end_idx.clamp(0, len) as usize;

            if start_idx >= end_idx {
                Ok(RuntimeValue::Array(vec![]))
            } else {
                Ok(RuntimeValue::Array(arr[start_idx..end_idx].to_vec()))
            }
        }
        RuntimeValue::String(ref s) => {
            let start_idx = match start {
                RuntimeValue::Number(n) => n as isize,
                RuntimeValue::Null => 0,
                _ => {
                    return Err(ExecutionError::TypeMismatch(format!(
                        "Slice start must be number or null, got {:?}",
                        start.get_type()
                    )))
                }
            };

            let end_idx = match end {
                RuntimeValue::Number(n) => n as isize,
                RuntimeValue::Null => s.len() as isize,
                _ => {
                    return Err(ExecutionError::TypeMismatch(format!(
                        "Slice end must be number or null, got {:?}",
                        end.get_type()
                    )))
                }
            };

            let len = s.len() as isize;
            let start_idx = if start_idx < 0 { len + start_idx } else { start_idx };
            let end_idx = if end_idx < 0 { len + end_idx } else { end_idx };

            let start_idx = start_idx.clamp(0, len) as usize;
            let end_idx = end_idx.clamp(0, len) as usize;

            if start_idx >= end_idx {
                Ok(RuntimeValue::String(String::new()))
            } else {
                // Handle Unicode characters properly
                let chars: Vec<char> = s.chars().collect();
                if start_idx >= chars.len() || end_idx > chars.len() {
                    Ok(RuntimeValue::String(String::new()))
                } else {
                    Ok(RuntimeValue::String(chars[start_idx..end_idx].iter().collect()))
                }
            }
        }
        _ => Err(ExecutionError::Custom(format!(
            "index: Cannot slice {:?} with start {:?} and end {:?}",
            operand.get_type(),
            start.get_type(),
            end.get_type()
        ))),
    }
}

// ============================================================================
// BUILTIN REGISTRATION
// ============================================================================

// Get all built-in functions as a HashMap.
pub fn get_builtins() -> HashMap<String, RuntimeValue> {
    let mut builtins = HashMap::new();

    // Unary operators.
    builtins.insert("!/unary".to_string(), RuntimeValue::Function("!/unary".to_string()));
    builtins.insert("-/unary".to_string(), RuntimeValue::Function("-/unary".to_string()));

    // Binary operators.
    builtins.insert("||".to_string(), RuntimeValue::Function("||".to_string()));
    builtins.insert("&&".to_string(), RuntimeValue::Function("&&".to_string()));
    builtins.insert("==".to_string(), RuntimeValue::Function("==".to_string()));
    builtins.insert("!=".to_string(), RuntimeValue::Function("!=".to_string()));
    builtins.insert(">".to_string(), RuntimeValue::Function(">".to_string()));
    builtins.insert("<".to_string(), RuntimeValue::Function("<".to_string()));
    builtins.insert(">=".to_string(), RuntimeValue::Function(">=".to_string()));
    builtins.insert("<=".to_string(), RuntimeValue::Function("<=".to_string()));
    builtins.insert("=~".to_string(), RuntimeValue::Function("=~".to_string()));
    builtins.insert("+".to_string(), RuntimeValue::Function("+".to_string()));
    builtins.insert("-".to_string(), RuntimeValue::Function("-".to_string()));
    builtins.insert("*".to_string(), RuntimeValue::Function("*".to_string()));
    builtins.insert("/".to_string(), RuntimeValue::Function("/".to_string()));
    builtins.insert("%".to_string(), RuntimeValue::Function("%".to_string()));

    // Utility functions.
    builtins.insert("log".to_string(), RuntimeValue::Function("log".to_string()));
    builtins.insert("if".to_string(), RuntimeValue::Function("if".to_string()));
    builtins.insert("apply".to_string(), RuntimeValue::Function("apply".to_string()));

    // Array operations.
    builtins.insert("count".to_string(), RuntimeValue::Function("count".to_string()));
    builtins.insert("filter".to_string(), RuntimeValue::Function("filter".to_string()));
    builtins.insert("map".to_string(), RuntimeValue::Function("map".to_string()));
    builtins.insert("find".to_string(), RuntimeValue::Function("find".to_string()));
    builtins.insert("reverse".to_string(), RuntimeValue::Function("reverse".to_string()));
    builtins.insert("flatten".to_string(), RuntimeValue::Function("flatten".to_string()));
    builtins.insert("sum".to_string(), RuntimeValue::Function("sum".to_string()));
    builtins.insert("sort".to_string(), RuntimeValue::Function("sort".to_string()));
    builtins.insert("sortby".to_string(), RuntimeValue::Function("sortby".to_string()));
    builtins.insert("reduce".to_string(), RuntimeValue::Function("reduce".to_string()));
    builtins.insert("groupby".to_string(), RuntimeValue::Function("groupby".to_string()));
    builtins.insert("withindices".to_string(), RuntimeValue::Function("withindices".to_string()));
    builtins.insert("sequence".to_string(), RuntimeValue::Function("sequence".to_string()));

    // Object operations.
    builtins.insert("keys".to_string(), RuntimeValue::Function("keys".to_string()));
    builtins.insert("values".to_string(), RuntimeValue::Function("values".to_string()));
    builtins.insert("entries".to_string(), RuntimeValue::Function("entries".to_string()));
    builtins.insert("fromentries".to_string(), RuntimeValue::Function("fromentries".to_string()));
    builtins.insert("mapkeys".to_string(), RuntimeValue::Function("mapkeys".to_string()));
    builtins.insert("mapvalues".to_string(), RuntimeValue::Function("mapvalues".to_string()));
    builtins.insert("filterkeys".to_string(), RuntimeValue::Function("filterkeys".to_string()));
    builtins.insert("filtervalues".to_string(), RuntimeValue::Function("filtervalues".to_string()));

    // String operations.
    builtins.insert("split".to_string(), RuntimeValue::Function("split".to_string()));
    builtins.insert("stringjoin".to_string(), RuntimeValue::Function("stringjoin".to_string()));
    builtins.insert("replace".to_string(), RuntimeValue::Function("replace".to_string()));
    builtins.insert("match".to_string(), RuntimeValue::Function("match".to_string()));
    builtins.insert("regex".to_string(), RuntimeValue::Function("regex".to_string()));

    // Mathematical functions.
    builtins.insert("range".to_string(), RuntimeValue::Function("range".to_string()));
    builtins.insert("summarize".to_string(), RuntimeValue::Function("summarize".to_string()));

    // Indexing.
    builtins.insert("index".to_string(), RuntimeValue::Function("index".to_string()));

    // Type conversion.
    builtins.insert("string".to_string(), RuntimeValue::Function("string".to_string()));
    builtins.insert("float".to_string(), RuntimeValue::Function("float".to_string()));

    builtins
}

// Execute a built-in function by name.
pub fn execute_builtin(name: &str, args: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    match name {
        // Unary operators.
        "!/unary" => {
            validate_args("!/unary", args, 1, Some(1))?;
            let value = execute_expression(&args[0], context)?;
            Ok(RuntimeValue::Boolean(!value.truthy()))
        }
        "-/unary" => {
            validate_args("-/unary", args, 1, Some(1))?;
            let value = execute_expression(&args[0], context)?;
            match value {
                RuntimeValue::Number(n) => Ok(RuntimeValue::Number(-n)),
                _ => Err(ExecutionError::TypeMismatch(format!("Cannot negate {}", value.get_type()))),
            }
        }

        // Binary operators.
        "||" => {
            validate_args("||", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            if left.truthy() {
                Ok(left.clone())
            } else {
                Ok(right.clone())
            }
        }
        "&&" => {
            validate_args("&&", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            if left.truthy() {
                Ok(right.clone())
            } else {
                Ok(left.clone())
            }
        }
        "==" => {
            validate_args("==", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            Ok(RuntimeValue::Boolean(left == right))
        }
        "!=" => {
            validate_args("!=", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            Ok(RuntimeValue::Boolean(left != right))
        }
        "=~" => {
            // TODO: Join with match_function?
            validate_args("=~", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                (RuntimeValue::String(s), RuntimeValue::Regex(regex)) => Ok(RuntimeValue::Boolean(regex.as_regex().is_match(s))),
                (RuntimeValue::String(s), RuntimeValue::String(pattern)) => {
                    // Treat string as regex pattern
                    match regex::Regex::new(pattern) {
                        Ok(regex) => Ok(RuntimeValue::Boolean(regex.is_match(s))),
                        Err(_) => Err(ExecutionError::TypeMismatch(format!("Invalid regex pattern: {}", pattern))),
                    }
                }
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Cannot match {:?} with {:?}",
                    left.get_type(),
                    right.get_type()
                ))),
            }
        }
        ">" => {
            validate_args(">", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match left.compare(&right) {
                Ok(std::cmp::Ordering::Greater) => Ok(RuntimeValue::Boolean(true)),
                Ok(_) => Ok(RuntimeValue::Boolean(false)),
                Err(e) => Err(ExecutionError::TypeMismatch(e)),
            }
        }
        "<" => {
            validate_args("<", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match left.compare(&right) {
                Ok(std::cmp::Ordering::Less) => Ok(RuntimeValue::Boolean(true)),
                Ok(_) => Ok(RuntimeValue::Boolean(false)),
                Err(e) => Err(ExecutionError::TypeMismatch(e)),
            }
        }
        ">=" => {
            validate_args(">=", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match left.compare(&right) {
                Ok(std::cmp::Ordering::Greater) | Ok(std::cmp::Ordering::Equal) => Ok(RuntimeValue::Boolean(true)),
                Ok(_) => Ok(RuntimeValue::Boolean(false)),
                Err(e) => Err(ExecutionError::TypeMismatch(e)),
            }
        }
        "<=" => {
            validate_args("<=", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match left.compare(&right) {
                Ok(std::cmp::Ordering::Less) | Ok(std::cmp::Ordering::Equal) => Ok(RuntimeValue::Boolean(true)),
                Ok(_) => Ok(RuntimeValue::Boolean(false)),
                Err(e) => Err(ExecutionError::TypeMismatch(e)),
            }
        }
        "+" => {
            validate_args("+", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a + b)),
                (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::String(format!("{}{}", a, b))),
                (RuntimeValue::Array(a), RuntimeValue::Array(b)) => {
                    let mut result = a.clone();
                    result.extend(b.clone());
                    Ok(RuntimeValue::Array(result))
                }
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Cannot add {:?} and {:?}",
                    left.get_type(),
                    right.get_type()
                ))),
            }
        }
        "-" => {
            validate_args("-", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a - b)),
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Cannot subtract {:?} from {:?}",
                    right.get_type(),
                    left.get_type()
                ))),
            }
        }
        "*" => {
            validate_args("*", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a * b)),
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Cannot multiply {:?} and {:?}",
                    left.get_type(),
                    right.get_type()
                ))),
            }
        }
        "/" => {
            validate_args("/", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                    if *b == 0.0 {
                        Err(ExecutionError::DivisionByZero)
                    } else {
                        Ok(RuntimeValue::Number(a / b))
                    }
                }
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Cannot divide {:?} by {:?}",
                    left.get_type(),
                    right.get_type()
                ))),
            }
        }
        "%" => {
            validate_args("%", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a % b)),
                _ => Err(ExecutionError::TypeMismatch(format!(
                    "Cannot modulo {:?} by {:?}",
                    left.get_type(),
                    right.get_type()
                ))),
            }
        }
        "." => {
            validate_args(".", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;
            match (&left, &right) {
                _ => Err(ExecutionError::Custom("Not implemented".to_string())),
            }
        }

        // Utility functions.
        "log" => log(args, context),
        "if" => if_function(args, context),
        "apply" => apply(args, context),

        // Array operations.
        "count" => count(args, context),
        "filter" => filter(args, context),
        "map" => map(args, context),
        "find" => find(args, context),
        "reverse" => reverse(args, context),
        "flatten" => flatten(args, context),
        "sum" => sum(args, context),
        "sort" => sort(args, context),
        "sortby" => sortby(args, context),
        "reduce" => reduce(args, context),
        "groupby" => groupby(args, context),
        "withindices" => withindices(args, context),
        "sequence" => sequence(args, context),

        // Object operations.
        "keys" => keys(args, context),
        "values" => values(args, context),
        "entries" => entries(args, context),
        "fromentries" => fromentries(args, context),
        "mapkeys" => mapkeys(args, context),
        "mapvalues" => mapvalues(args, context),
        "filterkeys" => filterkeys(args, context),
        "filtervalues" => filtervalues(args, context),

        // String operations.
        "split" => split(args, context),
        "stringjoin" => stringjoin(args, context),
        "replace" => replace(args, context),
        "match" => match_function(args, context),
        "regex" => regex(args, context),

        // Mathematical functions.
        "range" => range(args, context),
        "summarize" => summarize(args, context),

        // Indexing.
        "index" => index(args, context),

        // Type conversion.
        "string" => string(args, context),
        "float" => float(args, context),

        _ => Err(ExecutionError::Custom(format!("Unknown builtin function: {}", name))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Expression;
    use crate::types::{RuntimeValue, RuntimeValueType};
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
            ]),
        }];

        let result = count(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(3.0));
    }

    #[test]
    fn test_filter_function() {
        let mut context = create_test_context();

        // Filter even numbers: filter (@ % 2 == 0) [1, 2, 3, 4, 5]
        let condition = Expression::function_call(
            Expression::reference("==", false),
            vec![
                Expression::function_call(
                    Expression::reference("%", false),
                    vec![
                        Expression::RefExpression {
                            name: "@".to_string(),
                            absolute: false,
                        },
                        Expression::ValueExpression {
                            value: RuntimeValue::Number(2.0),
                        },
                    ],
                ),
                Expression::ValueExpression {
                    value: RuntimeValue::Number(0.0),
                },
            ],
        );

        let args = vec![
            condition,
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                    RuntimeValue::Number(4.0),
                    RuntimeValue::Number(5.0),
                ]),
            },
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
        let transformation = Expression::function_call(
            Expression::reference("*", false),
            vec![
                Expression::RefExpression {
                    name: "@".to_string(),
                    absolute: false,
                },
                Expression::ValueExpression {
                    value: RuntimeValue::Number(2.0),
                },
            ],
        );

        let args = vec![
            transformation,
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                ]),
            },
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
            ]),
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
            }),
        }];

        let result = keys(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            // Keys order is not guaranteed, so we check that both keys are present
            let keys: Vec<String> = arr
                .into_iter()
                .map(|v| match v {
                    RuntimeValue::String(s) => s,
                    _ => panic!("Expected string key"),
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
            value: RuntimeValue::Number(42.0),
        }];

        let result = string(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("42".to_string()));
    }

    #[test]
    fn test_if_function() {
        let mut context = create_test_context();

        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::Boolean(true),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("yes".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("no".to_string()),
            },
        ];

        let result = if_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("yes".to_string()));

        // Test false condition
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::Boolean(false),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("yes".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("no".to_string()),
            },
        ];

        let result = if_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("no".to_string()));
    }

    #[test]
    fn test_sort_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Array(vec![
                RuntimeValue::Number(3.0),
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
            ]),
        }];

        let result = sort(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], RuntimeValue::Number(1.0));
            assert_eq!(arr[1], RuntimeValue::Number(2.0));
            assert_eq!(arr[2], RuntimeValue::Number(3.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_sortby_function() {
        let mut context = create_test_context();

        // Sort by modulo 4: [3, 1, 2, 8] -> [8, 1, 2, 3] (sorted by @ % 4)
        let transformation = Expression::function_call(
            Expression::reference("%", false),
            vec![
                Expression::RefExpression {
                    name: "@".to_string(),
                    absolute: false,
                },
                Expression::ValueExpression {
                    value: RuntimeValue::Number(4.0),
                },
            ],
        );

        let args = vec![
            transformation,
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(3.0),
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(8.0),
                ]),
            },
        ];

        let result = sortby(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 4);
            assert_eq!(arr[0], RuntimeValue::Number(8.0)); // 8 % 4 = 0
            assert_eq!(arr[1], RuntimeValue::Number(1.0)); // 1 % 4 = 1
            assert_eq!(arr[2], RuntimeValue::Number(2.0)); // 2 % 4 = 2
            assert_eq!(arr[3], RuntimeValue::Number(3.0)); // 3 % 4 = 3
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_reduce_function() {
        let mut context = create_test_context();

        // Reduce to sum: reduce (@[0] + @[1]) 0 [1, 2, 3]
        let reducer = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::function_call(
                    Expression::reference("index", false),
                    vec![
                        Expression::ValueExpression {
                            value: RuntimeValue::Number(0.0),
                        },
                        Expression::RefExpression {
                            name: "@".to_string(),
                            absolute: false,
                        },
                    ],
                ),
                Expression::function_call(
                    Expression::reference("index", false),
                    vec![
                        Expression::ValueExpression {
                            value: RuntimeValue::Number(1.0),
                        },
                        Expression::RefExpression {
                            name: "@".to_string(),
                            absolute: false,
                        },
                    ],
                ),
            ],
        );

        let args = vec![
            reducer,
            Expression::ValueExpression {
                value: RuntimeValue::Number(0.0),
            }, // initial
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                ]),
            },
        ];

        let result = reduce(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(6.0));
    }

    #[test]
    fn test_groupby_function() {
        let mut context = create_test_context();

        // Group by gender
        let key_expr = Expression::RefExpression {
            name: "gender".to_string(),
            absolute: false,
        };

        let args = vec![
            key_expr,
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Object({
                        let mut map = HashMap::new();
                        map.insert("gender".to_string(), RuntimeValue::String("female".to_string()));
                        map.insert("name".to_string(), RuntimeValue::String("hayley".to_string()));
                        map
                    }),
                    RuntimeValue::Object({
                        let mut map = HashMap::new();
                        map.insert("gender".to_string(), RuntimeValue::String("male".to_string()));
                        map.insert("name".to_string(), RuntimeValue::String("abhik".to_string()));
                        map
                    }),
                    RuntimeValue::Object({
                        let mut map = HashMap::new();
                        map.insert("gender".to_string(), RuntimeValue::String("female".to_string()));
                        map.insert("name".to_string(), RuntimeValue::String("emily".to_string()));
                        map
                    }),
                ]),
            },
        ];

        let result = groupby(&args, &mut context).unwrap();
        if let RuntimeValue::Object(groups) = result {
            assert_eq!(groups.len(), 2);
            assert!(groups.contains_key("female"));
            assert!(groups.contains_key("male"));

            if let RuntimeValue::Array(female_group) = &groups["female"] {
                assert_eq!(female_group.len(), 2);
            }

            if let RuntimeValue::Array(male_group) = &groups["male"] {
                assert_eq!(male_group.len(), 1);
            }
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_withindices_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Array(vec![
                RuntimeValue::String("a".to_string()),
                RuntimeValue::String("b".to_string()),
                RuntimeValue::String("c".to_string()),
            ]),
        }];

        let result = withindices(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);

            // Check first pair
            if let RuntimeValue::Array(pair) = &arr[0] {
                assert_eq!(pair[0], RuntimeValue::Number(0.0));
                assert_eq!(pair[1], RuntimeValue::String("a".to_string()));
            }

            // Check second pair
            if let RuntimeValue::Array(pair) = &arr[1] {
                assert_eq!(pair[0], RuntimeValue::Number(1.0));
                assert_eq!(pair[1], RuntimeValue::String("b".to_string()));
            }

            // Check third pair
            if let RuntimeValue::Array(pair) = &arr[2] {
                assert_eq!(pair[0], RuntimeValue::Number(2.0));
                assert_eq!(pair[1], RuntimeValue::String("c".to_string()));
            }
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_entries_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Object({
                let mut map = HashMap::new();
                map.insert("foo".to_string(), RuntimeValue::String("bar".to_string()));
                map.insert("baz".to_string(), RuntimeValue::Number(42.0));
                map
            }),
        }];

        let result = entries(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);

            // Check that entries are [key, value] pairs
            for entry in arr {
                if let RuntimeValue::Array(pair) = entry {
                    assert_eq!(pair.len(), 2);
                    assert_eq!(pair[0].get_type(), RuntimeValueType::String);
                } else {
                    panic!("Expected array pair");
                }
            }
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_fromentries_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Array(vec![
                RuntimeValue::Array(vec![
                    RuntimeValue::String("foo".to_string()),
                    RuntimeValue::String("bar".to_string()),
                ]),
                RuntimeValue::Array(vec![RuntimeValue::String("baz".to_string()), RuntimeValue::Number(42.0)]),
            ]),
        }];

        let result = fromentries(&args, &mut context).unwrap();
        if let RuntimeValue::Object(obj) = result {
            assert_eq!(obj.len(), 2);
            assert_eq!(obj["foo"], RuntimeValue::String("bar".to_string()));
            assert_eq!(obj["baz"], RuntimeValue::Number(42.0));
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_range_function() {
        let mut context = create_test_context();

        // Test range(5) -> [0, 1, 2, 3, 4]
        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Number(5.0),
        }];

        let result = range(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 5);
            assert_eq!(arr[0], RuntimeValue::Number(0.0));
            assert_eq!(arr[1], RuntimeValue::Number(1.0));
            assert_eq!(arr[2], RuntimeValue::Number(2.0));
            assert_eq!(arr[3], RuntimeValue::Number(3.0));
            assert_eq!(arr[4], RuntimeValue::Number(4.0));
        } else {
            panic!("Expected array result");
        }

        // Test range(3, 8) -> [3, 4, 5, 6, 7]
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::Number(3.0),
            },
            Expression::ValueExpression {
                value: RuntimeValue::Number(8.0),
            },
        ];

        let result = range(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 5);
            assert_eq!(arr[0], RuntimeValue::Number(3.0));
            assert_eq!(arr[4], RuntimeValue::Number(7.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_summarize_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ValueExpression {
            value: RuntimeValue::Array(vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::Number(3.0),
                RuntimeValue::Number(4.0),
                RuntimeValue::Number(5.0),
                RuntimeValue::Number(6.0),
            ]),
        }];

        let result = summarize(&args, &mut context).unwrap();
        if let RuntimeValue::Object(stats) = result {
            assert_eq!(stats["max"], RuntimeValue::Number(6.0));
            assert_eq!(stats["min"], RuntimeValue::Number(1.0));
            assert_eq!(stats["mean"], RuntimeValue::Number(3.5));
            assert_eq!(stats["median"], RuntimeValue::Number(3.5));
            // Variance and stddev should be present
            assert!(stats.contains_key("variance"));
            assert!(stats.contains_key("stddev"));
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_index_function() {
        let mut context = create_test_context();

        // Test array indexing: index 1 [1, 2, 3] -> 2
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::Number(1.0),
            },
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                ]),
            },
        ];

        let result = index(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(2.0));

        // Test object indexing: index "key" {key: "value"} -> "value"
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::String("key".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::Object({
                    let mut map = HashMap::new();
                    map.insert("key".to_string(), RuntimeValue::String("value".to_string()));
                    map
                }),
            },
        ];

        let result = index(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("value".to_string()));

        // Test slicing: index 1 3 [1, 2, 3, 4, 5] -> [2, 3]
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::Number(1.0),
            },
            Expression::ValueExpression {
                value: RuntimeValue::Number(3.0),
            },
            Expression::ValueExpression {
                value: RuntimeValue::Array(vec![
                    RuntimeValue::Number(1.0),
                    RuntimeValue::Number(2.0),
                    RuntimeValue::Number(3.0),
                    RuntimeValue::Number(4.0),
                    RuntimeValue::Number(5.0),
                ]),
            },
        ];

        let result = index(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], RuntimeValue::Number(2.0));
            assert_eq!(arr[1], RuntimeValue::Number(3.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_replace_function() {
        let mut context = create_test_context();

        // Test string replacement
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::String("haha".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("z".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("haha".to_string()),
            },
        ];

        let result = replace(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("z".to_string()));
    }

    #[test]
    fn test_match_function() {
        let mut context = create_test_context();

        // Test string matching
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::String("test".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("this is a test".to_string()),
            },
        ];

        let result = match_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(true));

        // Test non-matching
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::String("xyz".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("this is a test".to_string()),
            },
        ];

        let result = match_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(false));
    }

    #[test]
    fn test_regex_function() {
        let mut context = create_test_context();

        // Test regex creation
        let args = vec![
            Expression::ValueExpression {
                value: RuntimeValue::String("test".to_string()),
            },
            Expression::ValueExpression {
                value: RuntimeValue::String("i".to_string()),
            },
        ];

        let result = regex(&args, &mut context).unwrap();
        assert_eq!(result.get_type(), RuntimeValueType::Regex);
    }
}
