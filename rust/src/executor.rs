//! Expression execution engine
//!
//! This module implements the core execution engine for MistQL expressions,
//! including contextualized expressions, function calls, and pipeline processing.

use crate::builtins::{is_stock_builtin, CustomFunction, CustomFunctionRegistry, FunctionMetadata};
use crate::parser::Expression;
use crate::types::{ValueView, ComputableValue};
use std::collections::HashMap;
use std::borrow::Cow;

// A single frame in the execution stack containing variable bindings.
pub type StackFrame<'a> = HashMap<String, ValueView<'a>>;

// The execution stack containing nested variable scopes.
pub type ExecutionStack<'a> = Vec<StackFrame<'a>>;

// Execution context containing the stack, custom functions, and root data.
#[derive(Debug, Clone)]
pub struct ExecutionContext<'a> {
    stack: ExecutionStack<'a>,
    custom_functions: CustomFunctionRegistry<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
    VariableNotFound(String),
    NotCallable(String),
    TypeMismatch(String),
    CannotCompare(String),
    DivisionByZero,
    InvalidOperation(String),
    CannotConvertToJSON(String),
    CannotConvertToComputableValue(String),
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
            ExecutionError::CannotCompare(msg) => write!(f, "Cannot compare: {}", msg),
            ExecutionError::DivisionByZero => write!(f, "Division by zero"),
            ExecutionError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            ExecutionError::CannotConvertToJSON(msg) => write!(f, "Cannot convert to JSON: {}", msg),
            ExecutionError::CannotConvertToComputableValue(msg) => write!(f, "Cannot convert to ComputableValue: {}", msg),
            ExecutionError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl<'a> ExecutionContext<'a> {
    pub fn new(data: ValueView<'a>) -> Self {
        let mut context = Self {
            stack: Vec::new(),
            custom_functions: CustomFunctionRegistry::new(),
        };

        context.build_initial_stack(data);
        context
    }

    pub fn with_builtins(data: ValueView<'a>) -> Self {
        Self::new(data)
    }

    pub fn with_custom_functions(data: ValueView<'a>, custom_functions: CustomFunctionRegistry<'a>) -> Self {
        let mut context = Self::new(data);
        context.custom_functions = custom_functions;
        context
    }

    // Add methods for custom function management
    pub fn register_custom_function(
        &mut self,
        name: String,
        function: CustomFunction<'a>,
        metadata: FunctionMetadata,
    ) -> Result<(), ExecutionError> {
        self.custom_functions.register_function(name, function, metadata)?;
        Ok(())
    }

    pub fn has_custom_function(&self, name: &str) -> bool {
        self.custom_functions.has_function(name)
    }

    fn build_initial_stack(&mut self, data: ValueView<'a>) {
        // Frame 0: $ variable containing the root data and builtin functions
        let mut dollar_frame = HashMap::new();
        dollar_frame.insert("@".to_string(), data.clone());

        // Add all builtin functions to the $ frame
        for builtin_name in crate::builtins::BUILTIN_NAMES.keys() {
            let builtin = crate::builtins::get_builtin(builtin_name).unwrap();

            dollar_frame.insert(builtin.name().to_string(), builtin.runtime_value().clone());
        }

        let mut functions_frame = HashMap::new();
        functions_frame.insert("$".to_string(), ValueView::from(dollar_frame));
        self.stack.push(functions_frame);
    }

    pub fn push_context(&mut self, value: ValueView<'a>) {
        let mut new_frame = HashMap::new();

        // Always add @ variable.
        new_frame.insert("@".to_string(), value.clone());

        // If the value is an object, populate keys as variables.
        value.iter_object_items().unwrap().for_each(|(key, val)| {
            if is_valid_identifier(key) {
                new_frame.insert(key.to_string(), val.clone());
            }
        });

        self.stack.push(new_frame);
    }

    pub fn pop_context(&mut self) -> Result<(), ExecutionError> {
        // TODO: Does this need to be updated, since we removed builtins?
        if self.stack.len() <= 2 {
            return Err(ExecutionError::Custom("Cannot pop initial stack frames".to_string()));
        }
        self.stack.pop();
        Ok(())
    }

    // Find a variable in the execution stack.
    pub fn find_variable(&self, name: &'a str, absolute: bool) -> Result<ValueView<'a>, ExecutionError> {
        if absolute {
            // For absolute references (like $), only search in the first frame.
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

        // If not found in stack, check custom functions
        if self.custom_functions.has_function(name) {
            return Ok(ValueView::Function(Cow::Borrowed(name)));
        }

        // Finally, check if it's a stock builtin
        if is_stock_builtin(name) {
            return Ok(ValueView::Function(Cow::Borrowed(name)));
        }

        Err(ExecutionError::VariableNotFound(name.to_string()))
    }

    // Get the current @ context value.
    pub fn get_current_context(&self) -> Result<&ValueView<'a>, ExecutionError> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.get("@") {
                return Ok(value);
            }
        }
        Err(ExecutionError::VariableNotFound("@".to_string()))
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

pub fn execute_expression<'a>(expr: &Expression, context: &mut ExecutionContext<'_>) -> Result<ValueView<'a>, ExecutionError> {
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

// Helper function to execute an expression in a contextualized way
// If the expression is a function reference, it creates a function call with @ as argument
// Otherwise, it evaluates the expression directly
pub fn execute_contextualized_expression<'a>(expr: &Expression, context: &mut ExecutionContext<'_>) -> Result<ValueView<'a>, ExecutionError> {
    match expr {
        Expression::RefExpression { name, absolute } => {
            // Check if this is a function reference
            let func_value = context.find_variable(name, *absolute)?;
            if matches!(func_value, ValueView::Function(_)) {
                // Create a function call with @ as argument
                let func_call = Expression::FnExpression {
                    function: Box::new(expr.clone()),
                    arguments: vec![Expression::RefExpression {
                        name: "@".to_string(),
                        absolute: false,
                    }],
                };
                execute_expression(&func_call, context)
            } else {
                // Not a function, evaluate directly
                execute_expression(expr, context)
            }
        }
        _ => {
            // Not a function reference, evaluate directly
            execute_expression(expr, context)
        }
    }
}

fn execute_array<'a>(items: &[Expression], context: &mut ExecutionContext<'_>) -> Result<ValueView<'a>, ExecutionError> {
    let mut result = Vec::new();
    for item in items {
        let value = execute_expression(item, context)?;
        result.push(value.as_value().unwrap().clone());
    }
    Ok(ValueView::from(&result))
}

fn execute_object<'a>(
    entries: &std::collections::HashMap<String, Expression>,
    context: &mut ExecutionContext<'_>,
) -> Result<ValueView<'a>, ExecutionError> {
    let mut result = std::collections::HashMap::new();
    for (key, expr) in entries {
        let value = execute_expression(expr, context)?;
        result.insert(key.clone(), value);
    }
    Ok(ValueView::Object(result))
}

fn execute_function_call(
    function: &Expression,
    arguments: &[Expression],
    context: &mut ExecutionContext<'a>,
) -> Result<RuntimeValue, ExecutionError> {
    let func_value = execute_expression(function, context)?;

    match func_value {
        RuntimeValue::Function(func_name) => {
            // First try custom functions
            if context.custom_functions.has_function(&func_name) {
                // We know the function exists, so we can safely execute it
                if let Some(function) = context.custom_functions.functions.get(&func_name) {
                    function(arguments, context)
                } else {
                    // This should never happen since we just checked has_function
                    Err(ExecutionError::Custom(format!("Custom function '{}' not found", func_name)))
                }
            } else {
                // Fall back to stock builtins
                let builtin = crate::builtins::get_builtin(&func_name).unwrap();
                builtin.call(arguments, context)
            }
        }
        _ => Err(ExecutionError::NotCallable(func_value.get_type().to_string())),
    }
}

fn execute_pipeline(stages: &[Expression], context: &mut ExecutionContext<'a>) -> Result<RuntimeValue, ExecutionError> {
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
                    // In pipe context, non-function references should throw an error
                    return Err(ExecutionError::NotCallable(func_value.get_type().to_string()));
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
                    // In pipe context, non-function results should throw an error
                    return Err(ExecutionError::NotCallable(func_value.get_type().to_string()));
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
                    // In pipe context, non-function results should throw an error
                    return Err(ExecutionError::NotCallable(stage_result.get_type().to_string()));
                }
            }
        };

        context.pop_context()?;
        data = result;
    }

    Ok(data)
}

fn execute_dot_access(object: &Expression, field: &str, context: &mut ExecutionContext<'a>) -> Result<RuntimeValue, ExecutionError> {
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
    use crate::types::MistQLValueType;
    use std::collections::HashMap;

    fn create_test_context() -> ExecutionContext<'a> {
        let data = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), RuntimeValue::String("John".to_string()));
            map.insert("age".to_string(), RuntimeValue::Number(30.0));
            map
        });

        ExecutionContext<'a>::new(data)
    }

    #[test]
    fn test_execution_context_creation() {
        let data = RuntimeValue::String("test".to_string());
        let context = ExecutionContext<'a>::new(data.clone());

        assert_eq!(context.get_root_data(), &data);
        assert!(context.stack_depth() >= 2); // builtins+$ and data frames
    }

    #[test]
    fn test_variable_lookup() {
        let context = create_test_context();

        // Test @ variable
        let at_value = context.find_variable("@", false).unwrap();
        assert_eq!(at_value.get_type(), MistQLValueType::Object);

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
        let context = create_test_context();

        // Add a builtin function
        // With the new architecture, builtins are checked directly in find_variable
        // No need to modify the context

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
    use std::collections::HashMap;

    fn create_test_context() -> ExecutionContext<'a> {
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

        ExecutionContext<'a>::with_builtins(data)
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

        let result = execute_expression(&expr, &mut context);
        assert!(matches!(result, Err(ExecutionError::NotCallable(_))));
    }

    #[test]
    fn test_execute_function_call_not_implemented() {
        let mut context = create_test_context();

        // Test calling a non-existent function
        let expr = Expression::function_call(Expression::reference("nonexistent", false), vec![]);

        let result = execute_expression(&expr, &mut context);
        assert!(matches!(result, Err(ExecutionError::VariableNotFound(_))));
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

        let mut context = ExecutionContext<'a>::with_builtins(data);

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

        let mut context = ExecutionContext<'a>::with_builtins(data);

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

#[cfg(test)]
mod custom_function_tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_context() -> ExecutionContext<'a> {
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

        ExecutionContext<'a>::with_builtins(data)
    }

    #[test]
    fn test_custom_function_registration_and_execution() {
        let mut context = create_test_context();

        // Define a custom function that doubles a number
        fn custom_double(args: &[Expression], context: &mut ExecutionContext<'a>) -> Result<RuntimeValue, ExecutionError> {
            crate::builtins::validate_args("custom_double", args, 1, Some(1))?;
            let value = execute_expression(&args[0], context)?;

            match value {
                RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n * 2.0)),
                _ => Err(ExecutionError::TypeMismatch("custom_double requires a number".to_string())),
            }
        }

        // Register the custom function
        context
            .register_custom_function(
                "custom_double".to_string(),
                custom_double,
                crate::builtins::FunctionMetadata {
                    name: "custom_double".to_string(),
                    min_args: 1,
                    max_args: Some(1),
                    description: "Double a number".to_string(),
                },
            )
            .unwrap();

        // Test that the function is registered
        assert!(context.has_custom_function("custom_double"));

        // Test executing the custom function
        let expr = Expression::function_call(
            Expression::reference("custom_double", false),
            vec![Expression::value(RuntimeValue::Number(21.0))],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(42.0));
    }

    #[test]
    fn test_custom_function_with_stock_builtin_interaction() {
        let mut context = create_test_context();

        // Define a custom function that adds 10 to a number
        fn add_ten(args: &[Expression], context: &mut ExecutionContext<'a>) -> Result<RuntimeValue, ExecutionError> {
            crate::builtins::validate_args("add_ten", args, 1, Some(1))?;
            let value = execute_expression(&args[0], context)?;

            match value {
                RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n + 10.0)),
                _ => Err(ExecutionError::TypeMismatch("add_ten requires a number".to_string())),
            }
        }

        // Register the custom function
        context
            .register_custom_function(
                "add_ten".to_string(),
                add_ten,
                crate::builtins::FunctionMetadata {
                    name: "add_ten".to_string(),
                    min_args: 1,
                    max_args: Some(1),
                    description: "Add 10 to a number".to_string(),
                },
            )
            .unwrap();

        // Test combining custom function with stock builtin
        // First use stock builtin sum, then custom function add_ten
        let expr = Expression::function_call(
            Expression::reference("add_ten", false),
            vec![Expression::function_call(
                Expression::reference("sum", false),
                vec![Expression::reference("scores", false)],
            )],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        // sum([85, 92, 78]) = 255, add_ten(255) = 265
        assert_eq!(result, RuntimeValue::Number(265.0));
    }

    #[test]
    fn test_custom_function_conflict_with_stock_builtin() {
        let mut context = create_test_context();

        fn dummy_function(_args: &[Expression], _context: &mut ExecutionContext<'a>) -> Result<RuntimeValue, ExecutionError> {
            Ok(RuntimeValue::Null)
        }

        // Try to register a function with a stock builtin name
        let result = context.register_custom_function(
            "count".to_string(),
            dummy_function,
            crate::builtins::FunctionMetadata {
                name: "count".to_string(),
                min_args: 1,
                max_args: Some(1),
                description: "Dummy count".to_string(),
            },
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("conflicts with stock builtin"));
    }

    #[test]
    fn test_stock_builtin_priority() {
        let mut context = create_test_context();

        // Even if we somehow had a custom function with the same name,
        // stock builtins should still work
        let expr = Expression::function_call(
            Expression::reference("count", false),
            vec![Expression::value(RuntimeValue::Array(vec![
                RuntimeValue::Number(1.0),
                RuntimeValue::Number(2.0),
                RuntimeValue::Number(3.0),
            ]))],
        );

        let result = execute_expression(&expr, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::Number(3.0));
    }

    #[test]
    fn test_custom_function_in_pipeline() {
        let mut context = create_test_context();

        // Define a custom function that squares a number
        fn square(args: &[Expression], context: &mut ExecutionContext<'a>) -> Result<RuntimeValue, ExecutionError> {
            crate::builtins::validate_args("square", args, 1, Some(1))?;
            let value = execute_expression(&args[0], context)?;

            match value {
                RuntimeValue::Number(n) => Ok(RuntimeValue::Number(n * n)),
                _ => Err(ExecutionError::TypeMismatch("square requires a number".to_string())),
            }
        }

        // Register the custom function
        context
            .register_custom_function(
                "square".to_string(),
                square,
                crate::builtins::FunctionMetadata {
                    name: "square".to_string(),
                    min_args: 1,
                    max_args: Some(1),
                    description: "Square a number".to_string(),
                },
            )
            .unwrap();

        // Test using custom function directly first
        let direct_expr = Expression::function_call(
            Expression::reference("square", false),
            vec![Expression::value(RuntimeValue::Number(5.0))],
        );
        let direct_result = execute_expression(&direct_expr, &mut context).unwrap();
        assert_eq!(direct_result, RuntimeValue::Number(25.0));

        // If we get here, the direct call works, so the issue is in the pipeline
        println!("Direct function call works!");

        // First test that stock builtin works in pipeline
        // @.scores | count
        let stock_expr = Expression::PipeExpression {
            stages: vec![
                Expression::DotAccessExpression {
                    object: Box::new(Expression::reference("@", false)),
                    field: "scores".to_string(),
                },
                Expression::reference("count", false),
            ],
        };
        let stock_result = execute_expression(&stock_expr, &mut context).unwrap();
        assert_eq!(stock_result, RuntimeValue::Number(3.0));
        println!("Stock builtin in pipeline works!");

        // Test using custom function in a pipeline
        // @.scores | map square
        let expr = Expression::PipeExpression {
            stages: vec![
                Expression::DotAccessExpression {
                    object: Box::new(Expression::reference("@", false)),
                    field: "scores".to_string(),
                },
                Expression::function_call(Expression::reference("map", false), vec![Expression::reference("square", false)]),
            ],
        };

        let result = execute_expression(&expr, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            // 85^2 = 7225, 92^2 = 8464, 78^2 = 6084
            assert_eq!(arr[0], RuntimeValue::Number(7225.0));
            assert_eq!(arr[1], RuntimeValue::Number(8464.0));
            assert_eq!(arr[2], RuntimeValue::Number(6084.0));
        } else {
            panic!("Expected array result");
        }
    }
}
