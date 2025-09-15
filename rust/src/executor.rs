//! Expression execution engine
//!
//! This module implements the core execution engine for MistQL expressions,
//! including contextualized expressions, function calls, and pipeline processing.

use crate::builtins::execute_builtin;
use crate::parser::Expression;
use crate::types::RuntimeValue;
use std::collections::HashMap;

// A single frame in the execution stack containing variable bindings.
pub type StackFrame = HashMap<String, RuntimeValue>;

// The execution stack containing nested variable scopes.
pub type ExecutionStack = Vec<StackFrame>;

// Execution context containing the stack, builtins, and root data.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    stack: ExecutionStack,
    builtins: HashMap<String, RuntimeValue>,
    root_data: RuntimeValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    VariableNotFound(String),
    NotCallable(String),
    TypeMismatch(String),
    DivisionByZero,
    InvalidOperation(String),
    CannotConvertToJSON(String),
    CannotConvertToRuntimeValue(String),
    Custom(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::VariableNotFound(name) => write!(f, "Could not find referenced variable: {}", name),
            ExecutionError::NotCallable(actual_type) => write!(
                f,
                "Attempted to call a variable of type \"{}\". Only functions are callable",
                actual_type
            ),
            ExecutionError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            ExecutionError::DivisionByZero => write!(f, "Division by zero"),
            ExecutionError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            ExecutionError::CannotConvertToJSON(msg) => write!(f, "Cannot convert to JSON: {}", msg),
            ExecutionError::CannotConvertToRuntimeValue(msg) => write!(f, "Cannot convert to RuntimeValue: {}", msg),
            ExecutionError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl ExecutionContext {
    pub fn new(data: RuntimeValue, builtins: HashMap<String, RuntimeValue>) -> Self {
        let mut context = Self {
            stack: Vec::new(),
            builtins,
            root_data: data.clone(),
        };

        context.build_initial_stack(data);
        context
    }

    pub fn with_builtins(data: RuntimeValue) -> Self {
        use crate::builtins::get_builtins;
        Self::new(data, get_builtins())
    }

    fn build_initial_stack(&mut self, data: RuntimeValue) {
        // Frame 0: Built-in functions and $ variable.
        let mut functions_frame = HashMap::new();
        for (key, value) in &self.builtins {
            functions_frame.insert(key.clone(), value.clone());
        }

        let mut dollar_frame = HashMap::new();
        dollar_frame.insert("@".to_string(), data.clone());
        for (key, value) in &self.builtins {
            dollar_frame.insert(key.clone(), value.clone());
        }
        functions_frame.insert("$".to_string(), RuntimeValue::Object(dollar_frame));

        self.stack.push(functions_frame);

        // Frame 1: Data context (object keys become variables).
        self.push_context(data);
    }

    pub fn push_context(&mut self, value: RuntimeValue) {
        let mut new_frame = HashMap::new();

        // Always add @ variable.
        new_frame.insert("@".to_string(), value.clone());

        // If the value is an object, populate keys as variables.
        if let RuntimeValue::Object(obj) = &value {
            for (key, val) in obj {
                if is_valid_identifier(key) {
                    new_frame.insert(key.clone(), val.clone());
                }
            }
        }

        self.stack.push(new_frame);
    }

    pub fn pop_context(&mut self) -> Result<(), ExecutionError> {
        if self.stack.len() <= 2 {
            return Err(ExecutionError::Custom("Cannot pop initial stack frames".to_string()));
        }
        self.stack.pop();
        Ok(())
    }

    // Find a variable in the execution stack.
    pub fn find_variable(&self, name: &str, absolute: bool) -> Result<RuntimeValue, ExecutionError> {
        if absolute {
            // For absolute references (like $), only search in the first frame (builtins).
            if let Some(frame) = self.stack.first() {
                if let Some(value) = frame.get(name) {
                    return Ok(value.clone());
                }
            }
        } else {
            // Search from top to bottom of stack.
            for frame in self.stack.iter().rev() {
                if let Some(value) = frame.get(name) {
                    return Ok(value.clone());
                }
            }
        }

        Err(ExecutionError::VariableNotFound(name.to_string()))
    }

    pub fn get_builtin(&self, name: &str) -> Result<&RuntimeValue, ExecutionError> {
        self.builtins
            .get(name)
            .ok_or_else(|| ExecutionError::VariableNotFound(name.to_string()))
    }

    // Get the current @ context value.
    pub fn get_current_context(&self) -> Result<&RuntimeValue, ExecutionError> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.get("@") {
                return Ok(value);
            }
        }
        Err(ExecutionError::VariableNotFound("@".to_string()))
    }

    pub fn get_root_data(&self) -> &RuntimeValue {
        &self.root_data
    }

    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }
}

// Check if a string is a valid identifier for variable binding.
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // First character must be alphabetic or underscore.
    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    // Remaining characters must be alphanumeric or underscore.
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

pub fn execute_expression(expr: &Expression, context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    match expr {
        Expression::ValueExpression { value } => Ok(value.clone()),
        Expression::RefExpression { name, absolute } => context.find_variable(name, *absolute),
        Expression::FnExpression { function, arguments } => execute_function_call(function, arguments, context),
        Expression::ArrayExpression { items } => execute_array(items, context),
        Expression::ObjectExpression { entries } => execute_object(entries, context),
        Expression::PipeExpression { stages } => execute_pipeline(stages, context),
        Expression::ParentheticalExpression { expression } => execute_expression(expression, context),
        Expression::DotAccessExpression { object, field } => execute_dot_access(object, field, context),
    }
}

fn execute_array(items: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    let mut result = Vec::new();
    for item in items {
        let value = execute_expression(item, context)?;
        result.push(value);
    }
    Ok(RuntimeValue::Array(result))
}

fn execute_object(
    entries: &std::collections::HashMap<String, Expression>,
    context: &mut ExecutionContext,
) -> Result<RuntimeValue, ExecutionError> {
    let mut result = std::collections::HashMap::new();
    for (key, expr) in entries {
        let value = execute_expression(expr, context)?;
        result.insert(key.clone(), value);
    }
    Ok(RuntimeValue::Object(result))
}

fn execute_function_call(
    function: &Expression,
    arguments: &[Expression],
    context: &mut ExecutionContext,
) -> Result<RuntimeValue, ExecutionError> {
    let func_value = execute_expression(function, context)?;

    match func_value {
        RuntimeValue::Function(func_name) => execute_builtin(&func_name, arguments, context),
        _ => Err(ExecutionError::NotCallable(func_value.get_type().to_string())),
    }
}

fn execute_pipeline(stages: &[Expression], context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    if stages.is_empty() {
        return Err(ExecutionError::Custom("Empty pipeline".to_string()));
    }

    // Execute first stage separately.
    let mut data = execute_expression(&stages[0], context)?;

    for stage in &stages[1..] {
        context.push_context(data.clone());

        let result = match stage {
            Expression::FnExpression { function, arguments } => {
                // For function calls, append data as the last argument.
                let mut new_args = arguments.clone();
                new_args.push(Expression::ValueExpression { value: data.clone() });

                let new_call = Expression::FnExpression {
                    function: function.clone(),
                    arguments: new_args,
                };
                execute_expression(&new_call, context)?
            }
            Expression::RefExpression { name, absolute } => {
                // For function references, create a function call with data as argument.
                let func_value = context.find_variable(name, *absolute)?;
                if matches!(func_value, RuntimeValue::Function(_)) {
                    let new_call = Expression::FnExpression {
                        function: Box::new(stage.clone()),
                        arguments: vec![Expression::ValueExpression { value: data.clone() }],
                    };
                    execute_expression(&new_call, context)?
                } else {
                    // For non-function references, execute normally.
                    execute_expression(stage, context)?
                }
            }
            Expression::DotAccessExpression { object: _, field: _ } => {
                // For dot access, check if it resolves to a function.
                let func_value = execute_expression(stage, context)?;
                if matches!(func_value, RuntimeValue::Function(_)) {
                    let new_call = Expression::FnExpression {
                        function: Box::new(Expression::ValueExpression { value: func_value }),
                        arguments: vec![Expression::ValueExpression { value: data.clone() }],
                    };
                    execute_expression(&new_call, context)?
                } else {
                    // For non-function results, return as-is.
                    func_value
                }
            }
            _ => {
                // For other expressions, execute and check if result is a function.
                let stage_result = execute_expression(stage, context)?;
                if matches!(stage_result, RuntimeValue::Function(_)) {
                    // If the result is a function, execute it with data as argument.
                    let new_call = Expression::FnExpression {
                        function: Box::new(Expression::ValueExpression { value: stage_result }),
                        arguments: vec![Expression::ValueExpression { value: data.clone() }],
                    };
                    execute_expression(&new_call, context)?
                } else {
                    // For non-function results, return as-is.
                    stage_result
                }
            }
        };

        context.pop_context()?;
        data = result;
    }

    Ok(data)
}

fn execute_dot_access(object: &Expression, field: &str, context: &mut ExecutionContext) -> Result<RuntimeValue, ExecutionError> {
    let obj_value = execute_expression(object, context)?;

    match obj_value {
        RuntimeValue::Object(obj) => Ok(obj.get(field).cloned().unwrap_or(RuntimeValue::Null)),
        RuntimeValue::Null => {
            // Null coalescing: property access on null returns null.
            Ok(RuntimeValue::Null)
        }
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot access property '{}' on type {}",
            field,
            obj_value.get_type()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RuntimeValue, RuntimeValueType};
    use std::collections::HashMap;

    fn create_test_context() -> ExecutionContext {
        let data = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), RuntimeValue::String("John".to_string()));
            map.insert("age".to_string(), RuntimeValue::Number(30.0));
            map
        });

        let builtins = HashMap::new();
        ExecutionContext::new(data, builtins)
    }

    #[test]
    fn test_execution_context_creation() {
        let data = RuntimeValue::String("test".to_string());
        let builtins = HashMap::new();
        let context = ExecutionContext::new(data.clone(), builtins);

        assert_eq!(context.get_root_data(), &data);
        assert!(context.stack_depth() >= 2); // builtins+$ and data frames
    }

    #[test]
    fn test_variable_lookup() {
        let context = create_test_context();

        // Test @ variable
        let at_value = context.find_variable("@", false).unwrap();
        assert_eq!(at_value.get_type(), RuntimeValueType::Object);

        // Test object key variables
        let name_value = context.find_variable("name", false).unwrap();
        assert_eq!(name_value, RuntimeValue::String("John".to_string()));

        let age_value = context.find_variable("age", false).unwrap();
        assert_eq!(age_value, RuntimeValue::Number(30.0));
    }

    #[test]
    fn test_variable_lookup_missing() {
        let context = create_test_context();

        let result = context.find_variable("nonexistent", false);
        assert!(matches!(result, Err(ExecutionError::VariableNotFound(_))));
    }

    #[test]
    fn test_push_pop_context() {
        let mut context = create_test_context();
        let initial_depth = context.stack_depth();

        let new_data = RuntimeValue::String("new context".to_string());
        context.push_context(new_data.clone());

        assert_eq!(context.stack_depth(), initial_depth + 1);

        // Check that @ now refers to the new context
        let at_value = context.find_variable("@", false).unwrap();
        assert_eq!(at_value, new_data);

        // Pop context
        context.pop_context().unwrap();
        assert_eq!(context.stack_depth(), initial_depth);
    }

    #[test]
    fn test_context_population_from_object() {
        let mut context = create_test_context();

        let obj_data = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("foo".to_string(), RuntimeValue::String("bar".to_string()));
            map.insert("123invalid".to_string(), RuntimeValue::String("ignored".to_string())); // Invalid identifier
            map.insert("_valid".to_string(), RuntimeValue::Number(42.0));
            map
        });

        context.push_context(obj_data);

        // Valid identifiers should be available as variables
        let foo_value = context.find_variable("foo", false).unwrap();
        assert_eq!(foo_value, RuntimeValue::String("bar".to_string()));

        let valid_value = context.find_variable("_valid", false).unwrap();
        assert_eq!(valid_value, RuntimeValue::Number(42.0));

        // Invalid identifiers should not be available
        let result = context.find_variable("123invalid", false);
        assert!(matches!(result, Err(ExecutionError::VariableNotFound(_))));
    }

    #[test]
    fn test_absolute_variable_lookup() {
        let mut context = create_test_context();

        // Add a builtin function
        let mut builtins = HashMap::new();
        builtins.insert("count".to_string(), RuntimeValue::Function("count".to_string()));
        context.builtins = builtins;

        // Rebuild stack with new builtins
        let data = context.get_root_data().clone();
        context.stack.clear();
        context.build_initial_stack(data);

        // Absolute lookup should find builtin
        let count_value = context.find_variable("count", true).unwrap();
        assert_eq!(count_value, RuntimeValue::Function("count".to_string()));

        // Non-absolute lookup should also find it (searches all frames)
        let count_value2 = context.find_variable("count", false).unwrap();
        assert_eq!(count_value2, RuntimeValue::Function("count".to_string()));
    }

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("valid_name"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("name123"));
        assert!(is_valid_identifier("a"));
        assert!(is_valid_identifier("_"));

        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123invalid"));
        assert!(!is_valid_identifier("name-with-dash"));
        assert!(!is_valid_identifier("name with space"));
        assert!(!is_valid_identifier("name.with.dot"));
    }

    #[test]
    fn test_error_display() {
        let error = ExecutionError::VariableNotFound("test".to_string());
        assert_eq!(format!("{}", error), "Could not find referenced variable: test");

        let error = ExecutionError::NotCallable("string".to_string());
        assert_eq!(
            format!("{}", error),
            "Attempted to call a variable of type \"string\". Only functions are callable"
        );

        let error = ExecutionError::DivisionByZero;
        assert_eq!(format!("{}", error), "Division by zero");
    }
}

#[cfg(test)]
mod execution_tests {
    use super::*;
    use crate::parser::Expression;
    use crate::types::RuntimeValue;
    use std::collections::HashMap;

    fn create_test_context() -> ExecutionContext {
        let data = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), RuntimeValue::String("John".to_string()));
            map.insert("age".to_string(), RuntimeValue::Number(30.0));
            map.insert(
                "scores".to_string(),
                RuntimeValue::Array(vec![
                    RuntimeValue::Number(85.0),
                    RuntimeValue::Number(92.0),
                    RuntimeValue::Number(78.0),
                ]),
            );
            map
        });

        ExecutionContext::with_builtins(data)
    }

    #[test]
    fn test_execute_value_expression() {
        let mut context = create_test_context();

        let expr = Expression::value(RuntimeValue::String("hello".to_string()));

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("hello".to_string()));
    }

    #[test]
    fn test_execute_reference_expression() {
        let mut context = create_test_context();

        let expr = Expression::reference("name", false);

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("John".to_string()));
    }

    #[test]
    fn test_execute_array_expression() {
        let mut context = create_test_context();

        let expr = Expression::array(vec![
            Expression::value(RuntimeValue::Number(1.0)),
            Expression::value(RuntimeValue::Number(2.0)),
            Expression::value(RuntimeValue::Number(3.0)),
        ]);

        let result = execute_expression(&expr, &mut context).unwrap();
        let expected = RuntimeValue::Array(vec![
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(3.0),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_object_expression() {
        let mut context = create_test_context();

        let mut entries = HashMap::new();
        entries.insert("key1".to_string(), Expression::value(RuntimeValue::String("value1".to_string())));
        entries.insert("key2".to_string(), Expression::value(RuntimeValue::Number(42.0)));

        let expr = Expression::object(entries);

        let result = execute_expression(&expr, &mut context).unwrap();

        if let RuntimeValue::Object(obj) = result {
            assert_eq!(obj["key1"], RuntimeValue::String("value1".to_string()));
            assert_eq!(obj["key2"], RuntimeValue::Number(42.0));
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_execute_parenthetical_expression() {
        let mut context = create_test_context();

        let expr = Expression::parenthetical(Expression::value(RuntimeValue::Number(42.0)));

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(42.0));
    }

    #[test]
    fn test_execute_unary_not() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("!/unary", true),
            vec![Expression::value(RuntimeValue::Boolean(true))],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(false));
    }

    #[test]
    fn test_execute_unary_negate() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("-/unary", false),
            vec![Expression::value(RuntimeValue::Number(42.0))],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(-42.0));
    }

    #[test]
    fn test_execute_binary_arithmetic() {
        let mut context = create_test_context();

        // Test addition
        let expr = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::value(RuntimeValue::Number(10.0)),
                Expression::value(RuntimeValue::Number(5.0)),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(15.0));

        // Test multiplication
        let expr = Expression::function_call(
            Expression::reference("*", false),
            vec![
                Expression::value(RuntimeValue::Number(3.0)),
                Expression::value(RuntimeValue::Number(4.0)),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(12.0));
    }

    #[test]
    fn test_execute_binary_comparison() {
        let mut context = create_test_context();

        // Test equality
        let expr = Expression::function_call(
            Expression::reference("==", false),
            vec![
                Expression::value(RuntimeValue::Number(5.0)),
                Expression::value(RuntimeValue::Number(5.0)),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(true));

        // Test greater than
        let expr = Expression::function_call(
            Expression::reference(">", false),
            vec![
                Expression::value(RuntimeValue::Number(10.0)),
                Expression::value(RuntimeValue::Number(5.0)),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(true));
    }

    #[test]
    fn test_execute_binary_logical() {
        let mut context = create_test_context();

        // Test logical AND (short-circuiting)
        let expr = Expression::function_call(
            Expression::reference("&&", false),
            vec![
                Expression::value(RuntimeValue::Boolean(false)),
                Expression::value(RuntimeValue::Boolean(true)),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(false));

        // Test logical OR (short-circuiting)
        let expr = Expression::function_call(
            Expression::reference("||", false),
            vec![
                Expression::value(RuntimeValue::Boolean(true)),
                Expression::value(RuntimeValue::Boolean(false)),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Boolean(true));
    }

    #[test]
    fn test_execute_dot_access() {
        let mut context = create_test_context();

        let expr = Expression::DotAccessExpression {
            object: Box::new(Expression::reference("@", false)),
            field: "name".to_string(),
        };

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("John".to_string()));
    }

    #[test]
    fn test_execute_dot_access_missing_key() {
        let mut context = create_test_context();

        let expr = Expression::DotAccessExpression {
            object: Box::new(Expression::reference("@", false)),
            field: "nonexistent".to_string(),
        };

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Null);
    }

    #[test]
    fn test_execute_array_indexing() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("index", false),
            vec![Expression::value(RuntimeValue::Number(0.0)), Expression::reference("scores", false)],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(85.0));
    }

    #[test]
    fn test_execute_array_negative_indexing() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("index", false),
            vec![
                Expression::value(RuntimeValue::Number(-1.0)),
                Expression::reference("scores", false),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(78.0));
    }

    #[test]
    fn test_execute_string_indexing() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("index", false),
            vec![
                Expression::value(RuntimeValue::Number(0.0)),
                Expression::value(RuntimeValue::String("hello".to_string())),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("h".to_string()));
    }

    #[test]
    fn test_execute_pipeline() {
        let mut context = create_test_context();

        // Simple pipeline: @ | name
        let expr = Expression::PipeExpression {
            stages: vec![Expression::reference("@", false), Expression::reference("name", false)],
        };

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("John".to_string()));
    }

    #[test]
    fn test_execute_function_call_not_implemented() {
        let mut context = create_test_context();

        // Add a function to the context so it can be found
        context
            .builtins
            .insert("count".to_string(), RuntimeValue::Function("count".to_string()));
        context.stack[0].insert("count".to_string(), RuntimeValue::Function("count".to_string()));

        let expr = Expression::function_call(Expression::reference("count", false), vec![]);

        let result = execute_expression(&expr, &mut context);
        assert!(matches!(result, Err(ExecutionError::Custom(_))));
    }

    #[test]
    fn test_execute_function_call_non_function() {
        let mut context = create_test_context();

        let expr = Expression::function_call(Expression::value(RuntimeValue::String("not a function".to_string())), vec![]);

        let result = execute_expression(&expr, &mut context);
        assert!(matches!(result, Err(ExecutionError::NotCallable(_))));
    }

    #[test]
    fn test_execute_division_by_zero() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("/", false),
            vec![
                Expression::value(RuntimeValue::Number(10.0)),
                Expression::value(RuntimeValue::Number(0.0)),
            ],
        );

        let result = execute_expression(&expr, &mut context);
        assert!(matches!(result, Err(ExecutionError::DivisionByZero)));
    }

    #[test]
    fn test_execute_type_mismatch() {
        let mut context = create_test_context();

        // Try to add string and number
        let expr = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::value(RuntimeValue::String("hello".to_string())),
                Expression::value(RuntimeValue::Number(5.0)),
            ],
        );

        let result = execute_expression(&expr, &mut context);
        assert!(matches!(result, Err(ExecutionError::TypeMismatch(_))));
    }

    #[test]
    fn test_execute_string_concatenation() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::value(RuntimeValue::String("hello ".to_string())),
                Expression::value(RuntimeValue::String("world".to_string())),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::String("hello world".to_string()));
    }

    #[test]
    fn test_execute_array_concatenation() {
        let mut context = create_test_context();

        let expr = Expression::function_call(
            Expression::reference("+", false),
            vec![
                Expression::value(RuntimeValue::Array(vec![RuntimeValue::Number(1.0), RuntimeValue::Number(2.0)])),
                Expression::value(RuntimeValue::Array(vec![RuntimeValue::Number(3.0), RuntimeValue::Number(4.0)])),
            ],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        let expected = RuntimeValue::Array(vec![
            RuntimeValue::Number(1.0),
            RuntimeValue::Number(2.0),
            RuntimeValue::Number(3.0),
            RuntimeValue::Number(4.0),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_contextualized_expressions_with_builtins() {
        // Create context with array data
        let data = RuntimeValue::Array(vec![
            RuntimeValue::Object({
                let mut map = HashMap::new();
                map.insert("name".to_string(), RuntimeValue::String("John".to_string()));
                map.insert("age".to_string(), RuntimeValue::Number(30.0));
                map
            }),
            RuntimeValue::Object({
                let mut map = HashMap::new();
                map.insert("name".to_string(), RuntimeValue::String("Jane".to_string()));
                map.insert("age".to_string(), RuntimeValue::Number(25.0));
                map
            }),
        ]);

        let mut context = ExecutionContext::with_builtins(data);

        // Test filter with contextualized expressions: filter (@.age > 26) @
        let condition = Expression::function_call(
            Expression::reference(">", false),
            vec![
                Expression::DotAccessExpression {
                    object: Box::new(Expression::reference("@", false)),
                    field: "age".to_string(),
                },
                Expression::value(RuntimeValue::Number(26.0)),
            ],
        );

        let filter_expr = Expression::function_call(
            Expression::reference("filter", false),
            vec![condition, Expression::reference("@", false)],
        );

        let result = execute_expression(&filter_expr, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 1);
            if let RuntimeValue::Object(obj) = &arr[0] {
                assert_eq!(obj["name"], RuntimeValue::String("John".to_string()));
                assert_eq!(obj["age"], RuntimeValue::Number(30.0));
            } else {
                panic!("Expected object in filtered result");
            }
        } else {
            panic!("Expected array result from filter");
        }
    }

    #[test]
    fn test_map_with_contextualized_expressions() {
        // Create context with array data
        let data = RuntimeValue::Array(vec![
            RuntimeValue::Object({
                let mut map = HashMap::new();
                map.insert("name".to_string(), RuntimeValue::String("John".to_string()));
                map.insert("age".to_string(), RuntimeValue::Number(30.0));
                map
            }),
            RuntimeValue::Object({
                let mut map = HashMap::new();
                map.insert("name".to_string(), RuntimeValue::String("Jane".to_string()));
                map.insert("age".to_string(), RuntimeValue::Number(25.0));
                map
            }),
        ]);

        let mut context = ExecutionContext::with_builtins(data);

        // Test map with contextualized expressions: map @.name @
        let transformation = Expression::DotAccessExpression {
            object: Box::new(Expression::reference("@", false)),
            field: "name".to_string(),
        };

        let map_expr = Expression::function_call(
            Expression::reference("map", false),
            vec![transformation, Expression::reference("@", false)],
        );

        let result = execute_expression(&map_expr, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], RuntimeValue::String("John".to_string()));
            assert_eq!(arr[1], RuntimeValue::String("Jane".to_string()));
        } else {
            panic!("Expected array result from map");
        }
    }
}
