//! Built-in functions for MistQL
//!
//! This module implements all the built-in functions that are available in MistQL,
//! including array operations, object operations, string operations, mathematical
//! functions, and utility functions.

use crate::executor::{execute_expression, ExecutionContext, ExecutionError};
use crate::parser::Expression;
use crate::types::RuntimeValue;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::rc::Rc;

// ============================================================================
// GLOBAL BUILTINS
// ============================================================================

#[derive(Debug, Clone)]
pub struct Builtin {
    name: String,
    callable: fn(&[Expression], &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError>,
}
impl Builtin {
    pub fn new(
        name: String,
        callable: fn(&[Expression], &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError>,
    ) -> Self {
        Self {
            name,
            callable,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn execute(&self, arguments: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
        (self.callable)(arguments, context)
    }
}
fn init_builtin_names() -> HashMap<String, Builtin> {
    HashMap::from([
        // Unary operators.
        ("!/unary".to_string(), Builtin::new("!/unary".to_string(), not)),
        ("-/unary".to_string(), Builtin::new("-/unary".to_string(), negate)),

        // Binary operators.
        ("||".to_string(), Builtin::new("||".to_string(), or)),
        ("&&".to_string(), Builtin::new("&&".to_string(), and)),
        ("==".to_string(), Builtin::new("==".to_string(), equal)),
        ("!=".to_string(), Builtin::new("!=".to_string(), notequal)),
        (">".to_string(), Builtin::new(">".to_string(), greaterthan)),
        ("<".to_string(), Builtin::new("<".to_string(), lessthan)),
        (">=".to_string(), Builtin::new(">=".to_string(), greaterthanorequal)),
        ("<=".to_string(), Builtin::new("<=".to_string(), lessthanorequal)),
        ("=~".to_string(), Builtin::new("=~".to_string(), regex_match)),
        ("+".to_string(), Builtin::new("+".to_string(), add_operator)),
        ("-".to_string(), Builtin::new("-".to_string(), subtract_operator)),
        ("*".to_string(), Builtin::new("*".to_string(), multiply_operator)),
        ("/".to_string(), Builtin::new("/".to_string(), divide_operator)),
        ("%".to_string(), Builtin::new("%".to_string(), modulo_operator)),

        // Utility functions.
        ("log".to_string(), Builtin::new("log".to_string(), log)),
        ("if".to_string(), Builtin::new("if".to_string(), if_function)),
        ("apply".to_string(), Builtin::new("apply".to_string(), apply)),

        // Array operations.
        ("count".to_string(), Builtin::new("count".to_string(), count)),
        ("filter".to_string(), Builtin::new("filter".to_string(), filter)),
        ("map".to_string(), Builtin::new("map".to_string(), map)),
        ("find".to_string(), Builtin::new("find".to_string(), find)),
        ("reverse".to_string(), Builtin::new("reverse".to_string(), reverse)),
        ("flatten".to_string(), Builtin::new("flatten".to_string(), flatten)),
        ("sum".to_string(), Builtin::new("sum".to_string(), sum)),
        ("sort".to_string(), Builtin::new("sort".to_string(), sort)),
        ("sortby".to_string(), Builtin::new("sortby".to_string(), sortby)),
        ("reduce".to_string(), Builtin::new("reduce".to_string(), reduce)),
        ("groupby".to_string(), Builtin::new("groupby".to_string(), groupby)),
        ("withindices".to_string(), Builtin::new("withindices".to_string(), withindices)),
        ("sequence".to_string(), Builtin::new("sequence".to_string(), sequence)),

        // Object operations.
        ("keys".to_string(), Builtin::new("keys".to_string(), keys)),
        ("values".to_string(), Builtin::new("values".to_string(), values)),
        ("entries".to_string(), Builtin::new("entries".to_string(), entries)),
        ("fromentries".to_string(), Builtin::new("fromentries".to_string(), fromentries)),
        ("mapkeys".to_string(), Builtin::new("mapkeys".to_string(), mapkeys)),
        ("mapvalues".to_string(), Builtin::new("mapvalues".to_string(), mapvalues)),
        ("filterkeys".to_string(), Builtin::new("filterkeys".to_string(), filterkeys)),
        ("filtervalues".to_string(), Builtin::new("filtervalues".to_string(), filtervalues)),

        // String operations.
        ("split".to_string(), Builtin::new("split".to_string(), split)),
        ("stringjoin".to_string(), Builtin::new("stringjoin".to_string(), stringjoin)),
        ("replace".to_string(), Builtin::new("replace".to_string(), replace)),
        ("match".to_string(), Builtin::new("match".to_string(), match_function)),
        ("regex".to_string(), Builtin::new("regex".to_string(), regex)),

        // Mathematical functions.
        ("range".to_string(), Builtin::new("range".to_string(), range)),
        ("summarize".to_string(), Builtin::new("summarize".to_string(), summarize)),

        // Indexing.
        ("index".to_string(), Builtin::new("index".to_string(), index)),

        // Type conversion.
        ("string".to_string(), Builtin::new("string".to_string(), string)),
        ("float".to_string(), Builtin::new("float".to_string(), float)),
    ])
}
static BUILTIN_NAMES: OnceLock<HashMap<String, Builtin>> = OnceLock::new();
pub fn get_builtin_names() -> &'static HashMap<String, Builtin> {
    BUILTIN_NAMES.get_or_init(init_builtin_names)
}
pub fn get_builtin(name: &str) -> Option<&'static Builtin> {
    get_builtin_names().get(name)
}
pub fn is_stock_builtin(name: &str) -> bool {
    get_builtin_names().contains_key(name)
}

// ============================================================================
// CUSTOM FUNCTION REGISTRY
// ============================================================================

// Custom function signature - matches builtin functions
pub type CustomFunction = fn(&[Expression], &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError>;

// Metadata for custom functions
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub min_args: usize,
    pub max_args: Option<usize>,
    pub description: String,
}

// Registry for custom functions
#[derive(Debug, Clone)]
pub struct CustomFunctionRegistry {
    pub functions: HashMap<String, CustomFunction>,
    metadata: HashMap<String, FunctionMetadata>,
}

impl CustomFunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn register_function(&mut self, name: String, function: CustomFunction, metadata: FunctionMetadata) -> Result<(), ExecutionError> {
        // Check for conflicts with stock builtins
        if is_stock_builtin(&name) {
            return Err(ExecutionError::Custom(format!(
                "Cannot register custom function '{}': conflicts with stock builtin",
                name
            )));
        }

        self.functions.insert(name.clone(), function);
        self.metadata.insert(name.clone(), metadata);
        Ok(())
    }

    pub fn execute(&self, name: &str, args: &[Expression], context: &mut ExecutionContext) -> Option<Result<Rc<RuntimeValue>, ExecutionError>> {
        self.functions.get(name).map(|function| function(args, context))
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn get_metadata(&self, name: &str) -> Option<&FunctionMetadata> {
        self.metadata.get(name)
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

// Validate the number of arguments for a builtin function.
pub fn validate_args(name: &str, args: &[Expression], min_args: usize, max_args: Option<usize>) -> Result<(), ExecutionError> {
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

pub fn assert_number(value: Rc<RuntimeValue>) -> Result<f64, ExecutionError> {
    match &*value {
        RuntimeValue::Number(n) => Ok(*n),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected number, got {}", value.get_type()))),
    }
}

pub fn assert_array(value: Rc<RuntimeValue>) -> Result<Rc<Vec<Rc<RuntimeValue>>>, ExecutionError> {
    match &*value {
        RuntimeValue::Array(array) => Ok(array.clone()),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected array, got {}", value.get_type()))),
    }
}

pub fn assert_object(value: Rc<RuntimeValue>) -> Result<Rc<HashMap<String, Rc<RuntimeValue>>>, ExecutionError> {
    match &*value {
        RuntimeValue::Object(obj) => Ok(obj.clone()),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected object, got {}", value.get_type()))),
    }
}

pub fn assert_string(value: Rc<RuntimeValue>) -> Result<String, ExecutionError> {
    match &*value {
        RuntimeValue::String(s) => Ok(s.clone()),
        _ => Err(ExecutionError::TypeMismatch(format!("Expected string, got {}", value.get_type()))),
    }
}

// ============================================================================
// UNARY OPERATORS
// ============================================================================

pub fn not(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("!/unary", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;
    Ok(Rc::new(RuntimeValue::Boolean(!value.truthy())))
}

pub fn negate(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("-/unary", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;
    match &*value {
        RuntimeValue::Number(n) => Ok(Rc::new(RuntimeValue::Number(-n))),
        _ => Err(ExecutionError::TypeMismatch(format!("Cannot negate {}", value.get_type()))),
    }
}

// ============================================================================
// BINARY OPERATORS
// ============================================================================

pub fn or(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("||", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    if left.truthy() {
        Ok(left.clone())
    } else {
        Ok(right.clone())
    }
}

pub fn and(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("&&", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    if left.truthy() {
        Ok(right.clone())
    } else {
        Ok(left.clone())
    }
}

pub fn equal(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("==", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    Ok(Rc::new(RuntimeValue::Boolean(left == right)))
}

pub fn notequal(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("!=", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    Ok(Rc::new(RuntimeValue::Boolean(left != right)))
}

pub fn greaterthan(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args(">", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match left.compare(&right) {
        Ok(std::cmp::Ordering::Greater) => Ok(Rc::new(RuntimeValue::Boolean(true))),
        Ok(_) => Ok(Rc::new(RuntimeValue::Boolean(false))),
        Err(e) => Err(ExecutionError::TypeMismatch(e)),
    }
}

pub fn lessthan(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("<", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match left.compare(&right) {
        Ok(std::cmp::Ordering::Less) => Ok(Rc::new(RuntimeValue::Boolean(true))),
        Ok(_) => Ok(Rc::new(RuntimeValue::Boolean(false))),
        Err(e) => Err(ExecutionError::TypeMismatch(e)),
    }
}

pub fn greaterthanorequal(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args(">=", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match left.compare(&right) {
        Ok(std::cmp::Ordering::Greater) | Ok(std::cmp::Ordering::Equal) => Ok(Rc::new(RuntimeValue::Boolean(true))),
        Ok(_) => Ok(Rc::new(RuntimeValue::Boolean(false))),
        Err(e) => Err(ExecutionError::TypeMismatch(e)),
    }
}

pub fn lessthanorequal(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("<=", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match left.compare(&right) {
        Ok(std::cmp::Ordering::Less) | Ok(std::cmp::Ordering::Equal) => Ok(Rc::new(RuntimeValue::Boolean(true))),
        Ok(_) => Ok(Rc::new(RuntimeValue::Boolean(false))),
        Err(e) => Err(ExecutionError::TypeMismatch(e)),
    }
}

pub fn regex_match(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("=~", args, 2, Some(2))?;
    let target = execute_expression(&args[0], context)?;
    let pattern = execute_expression(&args[1], context)?;
    match (&*target, &*pattern) {
        (RuntimeValue::String(s), RuntimeValue::Regex(r)) => Ok(Rc::new(RuntimeValue::Boolean(r.as_regex().is_match(s)))),
        (RuntimeValue::String(s), RuntimeValue::String(p)) => {
            // Treat string as regex pattern
            match regex::Regex::new(p) {
                Ok(regex) => Ok(Rc::new(RuntimeValue::Boolean(regex.is_match(s)))),
                Err(_) => Err(ExecutionError::TypeMismatch(format!("Invalid regex pattern: {}", p))),
            }
        }
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot match {:?} with {:?}",
            target.get_type(),
            pattern.get_type()
        ))),
    }
}

pub fn add_operator(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("+", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match (&*left, &*right) {
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::number(a + b)),
        (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::string(format!("{}{}", a, b))),
        (RuntimeValue::Array(a), RuntimeValue::Array(b)) => {
            let mut result: Vec<Rc<RuntimeValue>> = Vec::new();
            result.extend(a.iter().map(|v| v.clone()).collect::<Vec<Rc<RuntimeValue>>>());
            result.extend(b.iter().map(|v| v.clone()).collect::<Vec<Rc<RuntimeValue>>>());
            Ok(RuntimeValue::array(result))
        }
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot add {:?} and {:?}",
            left.get_type(),
            right.get_type()
        ))),
    }
}

pub fn subtract_operator(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("-", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match (&*left, &*right) {
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(Rc::new(RuntimeValue::Number(a - b))),
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot subtract {:?} from {:?}",
            right.get_type(),
            left.get_type()
        ))),
    }
}

pub fn multiply_operator(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("*", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match (&*left, &*right) {
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(Rc::new(RuntimeValue::Number(a * b))),
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot multiply {:?} and {:?}",
            left.get_type(),
            right.get_type()
        ))),
    }
}

pub fn divide_operator(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("/", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match (&*left, &*right) {
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
            if *b == 0.0 {
                Err(ExecutionError::DivisionByZero)
            } else {
                Ok(Rc::new(RuntimeValue::Number(a / b)))
            }
        }
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot divide {:?} by {:?}",
            left.get_type(),
            right.get_type()
        ))),
    }
}

pub fn modulo_operator(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("%", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    match (&*left, &*right) {
        (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(Rc::new(RuntimeValue::Number(a % b))),
        _ => Err(ExecutionError::TypeMismatch(format!(
            "Cannot modulo {:?} by {:?}",
            left.get_type(),
            right.get_type()
        ))),
    }
}

pub fn dot_operator(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args(".", args, 2, Some(2))?;
    let left = execute_expression(&args[0], context)?;
    let right = execute_expression(&args[1], context)?;
    // TODO: Implement this.
    match (&*left, &*right) {
        _ => Err(ExecutionError::Custom("Not implemented".to_string())),
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

// Log function - prints the value and returns it.
pub fn log(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("log", args, 1, Some(1))?;

    let value = execute_expression(&args[0], context)?;
    println!("MistQL: {}", value.to_string_serialize());
    Ok(value.clone())
}

// If function - conditional execution.
pub fn if_function(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("if", args, 3, Some(3))?;

    let condition = execute_expression(&args[0], context)?;
    if condition.truthy() {
        execute_expression(&args[1], context)
    } else {
        execute_expression(&args[2], context)
    }
}

// Apply function - applies an expression to a value (sets @ context).
pub fn apply(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
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
pub fn count(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("count", args, 1, Some(1))?;

    let array = assert_array(execute_expression(&args[0], context)?)?;
    Ok(RuntimeValue::number(array.len() as f64))
}

// Filter array elements based on a condition.
pub fn filter(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("filter", args, 2, Some(2))?;

    let condition = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut result = Vec::new();
    for item in array.iter() {
        context.push_context(item.clone());

        let condition_eval = execute_expression(condition, context)?;

        // If the condition evaluates to a function, call it with @ as the argument
        let condition_result = if let RuntimeValue::Function(_) = &*condition_eval {
            let call_expr = Expression::FnExpression {
                function: Box::new(condition.clone()),
                arguments: vec![Expression::RefExpression { name: "@".to_string(), absolute: false }],
            };
            execute_expression(&call_expr, context)?
        } else {
            condition_eval
        };

        context.pop_context()?;
        if condition_result.truthy() {
            result.push(item.clone());
        }
    }

    Ok(RuntimeValue::array(result))
}

// Transform array elements.
pub fn map(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("map", args, 2, Some(2))?;

    let transformation = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut result = Vec::new();
    for item in array.iter() {
        context.push_context(item.clone());

        let transformed_result = execute_expression(transformation, context)?;

        // If the transformation evaluates to a function, call it with @ as the argument
        let transformed = if let RuntimeValue::Function(_) = &*transformed_result {
            let call_expr = Expression::FnExpression {
                function: Box::new(transformation.clone()),
                arguments: vec![Expression::RefExpression { name: "@".to_string(), absolute: false }],
            };
            execute_expression(&call_expr, context)?
        } else {
            transformed_result
        };

        context.pop_context()?;
        result.push(transformed);
    }

    Ok(RuntimeValue::array(result))
}

// Find the first element that matches a condition.
pub fn find(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("find", args, 2, Some(2))?;

    let condition = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    for item in array.iter() {
        context.push_context(item.clone());

        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            return Ok(item.clone());
        }
    }

    Ok(RuntimeValue::null())
}

// Reverse an array.
pub fn reverse(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("reverse", args, 1, Some(1))?;

    let result = execute_expression(&args[0], context)?;
    let array = assert_array(result)?;
    let mut result = Vec::new();
    result.extend(array.iter().rev().map(|v| v.clone()));
    Ok(RuntimeValue::array(result))
}

// Flatten nested arrays.
pub fn flatten(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("flatten", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = Vec::new();
    for item in array.iter() {
        match &**item {
            RuntimeValue::Array(nested) => result.extend(nested.iter().cloned()),
            _ => {
                return Err(ExecutionError::TypeMismatch(format!(
                    "flatten: all elements must be arrays, got {}",
                    item.get_type()
                )))
            }
        }
    }

    Ok(RuntimeValue::array(result))
}

// Sum all numbers in an array.
pub fn sum(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("sum", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;

    let array = match &*value {
        RuntimeValue::Array(arr) => arr.clone(),
        _ => return Err(ExecutionError::TypeMismatch(format!("Expected array, got {}", value.get_type()))),
    };

    let mut total = 0.0;
    for item in array.iter() {
        let num = match &**item {
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

    Ok(RuntimeValue::number(total))
}

// Sort an array in ascending order.
pub fn sort(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("sort", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    if array.is_empty() {
        return Ok(RuntimeValue::array(Vec::new()));
    }

    let first_type = array[0].get_type();
    for item in array.iter() {
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

    let mut result = Vec::new();
    result.extend(array.iter().cloned());
    result.sort_by(|a, b| a.compare(b).unwrap());

    Ok(RuntimeValue::array(result))
}

// Sort an array by a key expression.
pub fn sortby(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("sortby", args, 2, Some(2))?;

    let key_expr = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut with_key: Vec<(Rc<RuntimeValue>, Rc<RuntimeValue>)> = Vec::new();
    for item in array.iter() {
        context.push_context(item.clone());

        let key_eval = execute_expression(key_expr, context)?;

        // If the key expression evaluates to a function, call it with @ as the argument
        let key = if let RuntimeValue::Function(_) = &*key_eval {
            let call_expr = Expression::FnExpression {
                function: Box::new(key_expr.clone()),
                arguments: vec![Expression::RefExpression { name: "@".to_string(), absolute: false }],
            };
            execute_expression(&call_expr, context)?
        } else {
            key_eval
        };

        context.pop_context()?;

        if !key.comparable() {
            return Err(ExecutionError::Custom("sortby: Cannot sort non-comparable values".to_string()));
        }

        with_key.push((key, item.clone()));
    }

    with_key.sort_by(|a, b| a.0.compare(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let result: Vec<Rc<RuntimeValue>> = with_key.into_iter().map(|(_, value)| value).collect();

    Ok(RuntimeValue::array(result))
}

// Reduce an array to a single value.
pub fn reduce(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("reduce", args, 3, Some(3))?;

    let reducer_expr = &args[0];
    let initial = execute_expression(&args[1], context)?;
    let array = assert_array(execute_expression(&args[2], context)?)?;

    let mut accumulator = initial.clone();
    for item in array.iter() {
        let acc_cur = RuntimeValue::array(vec![accumulator.clone(), item.clone()]);
        context.push_context(acc_cur.clone());

        let reducer_eval = execute_expression(reducer_expr, context)?;

        // If the reducer evaluates to a function, call it with @ as the argument
        accumulator = if let RuntimeValue::Function(_) = &*reducer_eval {
            let call_expr = Expression::FnExpression {
                function: Box::new(reducer_expr.clone()),
                arguments: vec![Expression::RefExpression { name: "@".to_string(), absolute: false }],
            };
            execute_expression(&call_expr, context)?
        } else {
            reducer_eval
        };

        context.pop_context()?;
    }

    Ok(accumulator)
}

// Group array elements by a key expression.
pub fn groupby(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("groupby", args, 2, Some(2))?;

    let key_expr = &args[0];
    let array = assert_array(execute_expression(&args[1], context)?)?;

    let mut groups: HashMap<String, Vec<Rc<RuntimeValue>>> = HashMap::new();

    for item in array.iter() {
        context.push_context(item.clone());

        let key = execute_expression(key_expr, context)?;
        context.pop_context()?;

        // Convert the key to a string for grouping
        let key_string = key.to_string_display();

        // Add the item to the group for this key
        groups.entry(key_string)
            .or_insert_with(Vec::new)
            .push(item.clone());
    }

    // Convert the HashMap<String, Vec<Rc<RuntimeValue>>> to HashMap<String, Rc<RuntimeValue>>
    let result: HashMap<String, Rc<RuntimeValue>> = groups
        .into_iter()
        .map(|(k, v)| (k, RuntimeValue::array(v)))
        .collect();

    Ok(RuntimeValue::object(result))
}

// Add indices to array elements.
pub fn withindices(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("withindices", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = Vec::new();
    for (index, item) in array.iter().enumerate() {
        let index_value = RuntimeValue::number(index as f64);
        let pair = RuntimeValue::array(vec![index_value, item.clone()]);
        result.push(pair);
    }

    Ok(RuntimeValue::array(result))
}

// Find subsequences satisfying conditions.
pub fn sequence(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    if args.len() < 2 {
        return Err(ExecutionError::Custom("sequence takes at least 2 arguments".to_string()));
    }

    let predicates = &args[..args.len() - 1];
    let array = assert_array(execute_expression(&args[args.len() - 1], context)?)?;

    // Create bitmasks for each predicate.
    let mut bitmasks: Vec<Vec<bool>> = Vec::new();
    for predicate in predicates {
        let mut bitmask = Vec::new();
        for item in array.iter() {
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
        result.push(RuntimeValue::array(subsequence));
    }

    Ok(RuntimeValue::array(result))
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
pub fn keys(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("keys", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    // Sort keys to match JavaScript behavior.
    let mut sorted_keys: Vec<String> = object.keys().cloned().collect();
    sorted_keys.sort();

    let keys: Vec<Rc<RuntimeValue>> = sorted_keys.iter().map(|k| RuntimeValue::string(k.clone())).collect();

    Ok(RuntimeValue::array(keys))
}

// Return the values of an object.
pub fn values(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("values", args, 1, Some(1))?;
    let target = execute_expression(&args[0], context)?;

    match &*target {
        RuntimeValue::Object(object) => {
            // Sort keys to match JavaScript behavior.
            let mut sorted_keys: Vec<String> = object.keys().cloned().collect();
            sorted_keys.sort();

            let values: Vec<Rc<RuntimeValue>> = sorted_keys.iter().map(|key| object[key].clone()).collect();
            Ok(RuntimeValue::array(values))
        }
        _ => {
            // For non-objects, return empty array (matches Python behavior)
            Ok(RuntimeValue::array(Vec::new()))
        }
    }
}

// Return key-value pairs as an array of [key, value] arrays.
pub fn entries(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("entries", args, 1, Some(1))?;
    let object = assert_object(execute_expression(&args[0], context)?)?;

    // Sort keys to match JavaScript behavior.
    let mut sorted_keys: Vec<String> = object.keys().cloned().collect();
    sorted_keys.sort();

    let mut result = Vec::new();
    for key in sorted_keys {
        let value = object[&key].clone();
        let entry = RuntimeValue::array(vec![Rc::new(RuntimeValue::String(key)), value.clone()]);
        result.push(entry);
    }

    Ok(RuntimeValue::array(result))
}

// Create an object from an array of [key, value] pairs.
pub fn fromentries(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("fromentries", args, 1, Some(1))?;
    let array = assert_array(execute_expression(&args[0], context)?)?;

    let mut result = HashMap::new();
    for entry in array.iter() {
        let entry_array = assert_array(entry.clone())?;

        // TODO: Is this correct?
        let key = if !entry_array.is_empty() {
            entry_array[0].to_string_display()
        } else {
            "null".to_string()
        };

        // TODO: Is this correct?
        let value = if entry_array.len() > 1 {
            entry_array[1].clone()
        } else {
            Rc::new(RuntimeValue::Null)
        };

        result.insert(key, value);
    }

    Ok(RuntimeValue::object(result))
}

// Map object keys.
pub fn mapkeys(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("mapkeys", args, 2, Some(2))?;

    let transformation = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object.iter() {
        context.push_context(RuntimeValue::string(key.clone()));

        let new_key = execute_expression(transformation, context)?;
        context.pop_context()?;
        // Dispatch to string_serialize to handle nested objects and arrays.
        // Otherwise use string_display to handle strings.
        match &*new_key {
            RuntimeValue::Object(_) => {
                result.insert(new_key.to_string_serialize(), value.clone());
            }
            RuntimeValue::Array(_) => {
                result.insert(new_key.to_string_serialize(), value.clone());
            }
            _ => {
                result.insert(new_key.to_string_display(), value.clone());
            }
        }
    }

    Ok(RuntimeValue::object(result))
}

// Map object values.
pub fn mapvalues(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("mapvalues", args, 2, Some(2))?;

    let transformation = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object.iter() {
        context.push_context(value.clone());

        let new_value = execute_expression(transformation, context)?;
        context.pop_context()?;
        result.insert(key.clone(), new_value.clone());
    }

    Ok(RuntimeValue::object(result))
}

// Filter object keys based on a condition.
pub fn filterkeys(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("filterkeys", args, 2, Some(2))?;

    let condition = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object.iter() {
        context.push_context(RuntimeValue::string(key.clone()));

        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            result.insert(key.clone(), value.clone());
        }
    }

    Ok(RuntimeValue::object(result))
}

// Filter object values based on a condition.
pub fn filtervalues(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("filtervalues", args, 2, Some(2))?;

    let condition = &args[0];
    let object = assert_object(execute_expression(&args[1], context)?)?;

    let mut result = HashMap::new();
    for (key, value) in object.iter() {
        context.push_context(value.clone());

        let condition_result = execute_expression(condition, context)?;
        context.pop_context()?;
        if condition_result.truthy() {
            result.insert(key.clone(), value.clone());
        }
    }

    Ok(RuntimeValue::object(result))
}

// ============================================================================
// STRING OPERATIONS
// ============================================================================

// Split a string by a delimiter.
pub fn split(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("split", args, 2, Some(2))?;

    let delimiter_value = execute_expression(&args[0], context)?;
    let string = assert_string(execute_expression(&args[1], context)?)?;

    let parts: Vec<Rc<RuntimeValue>> = match &*delimiter_value {
        RuntimeValue::String(delimiter) => {
            if delimiter.is_empty() {
                // Special case: empty delimiter splits into individual characters.
                string.chars().map(|c| RuntimeValue::string(c.to_string())).collect()
            } else {
                string.split(delimiter).map(|s| RuntimeValue::string(s.to_string())).collect()
            }
        }
        RuntimeValue::Regex(regex) => {
            let compiled_regex = regex.as_regex();
            compiled_regex.split(&string).map(|s| RuntimeValue::string(s.to_string())).collect()
        }
        _ => {
            return Err(ExecutionError::TypeMismatch(format!(
                "Expected string or regex for split delimiter, got {}",
                delimiter_value.get_type()
            )));
        }
    };

    Ok(RuntimeValue::array(parts))
}

// Join an array of strings.
pub fn stringjoin(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("stringjoin", args, 2, Some(2))?;

    let delimiter = assert_string(execute_expression(&args[0], context)?)?;
    let array = assert_array(execute_expression(&args[1], context)?)?;

    // Convert each element to a string, casting non-strings
    let strings: Vec<String> = array.iter().map(|v| {
        match &**v {
            RuntimeValue::String(s) => s.clone(),
            // For non-strings, use to_string_display which doesn't add quotes
            _ => v.to_string_display(),
        }
    }).collect();

    let result = strings.join(&delimiter);

    Ok(RuntimeValue::string(result))
}

// Replace substrings in a string.
pub fn replace(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("replace", args, 3, Some(3))?;

    let pattern = execute_expression(&args[0], context)?;
    let replacement = assert_string(execute_expression(&args[1], context)?)?;
    let target = assert_string(execute_expression(&args[2], context)?)?;

    match &*pattern {
        RuntimeValue::String(pattern_str) => {
            // Replace first occurrence only.
            let result = if let Some(pos) = target.find(pattern_str) {
                let mut result = target.clone();
                result.replace_range(pos..pos + pattern_str.len(), &replacement);
                result
            } else {
                target.clone()
            };
            Ok(RuntimeValue::string(result))
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
            Ok(RuntimeValue::string(result))
        }
        _ => Err(ExecutionError::TypeMismatch("replace: pattern must be string or regex".to_string())),
    }
}

// Test if a string matches a pattern.
pub fn match_function(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("match", args, 2, Some(2))?;

    let pattern = execute_expression(&args[0], context)?;
    let target = assert_string(execute_expression(&args[1], context)?)?;

    let matches = match &*pattern {
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

    Ok(RuntimeValue::boolean(matches))
}

// Create a regex object.
pub fn regex(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("regex", args, 1, Some(2))?;

    let pattern = assert_string(execute_expression(&args[0], context)?)?;
    let flags = if args.len() == 2 {
        assert_string(execute_expression(&args[1], context)?)?
    } else {
        String::new()
    };

    let regex_obj =
        crate::types::MistQLRegex::new(&pattern, &flags).map_err(|e| ExecutionError::Custom(format!("Invalid regex: {}", e)))?;

    Ok(RuntimeValue::regex(regex_obj))
}

// ============================================================================
// TYPE CONVERSION
// ============================================================================

// Convert a value to a string.
pub fn string(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("string", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;

    // Helper function to check if value or any nested values contain function/regex
    fn contains_non_stringable(value: &RuntimeValue) -> bool {
        match value {
            RuntimeValue::Function(_) | RuntimeValue::Regex(_) => true,
            RuntimeValue::Array(arr) => arr.iter().any(|v| contains_non_stringable(v)),
            RuntimeValue::Object(obj) => obj.values().any(|v| contains_non_stringable(v)),
            _ => false,
        }
    }

    if contains_non_stringable(&value) {
        return Err(ExecutionError::TypeMismatch("Cannot cast function or regex to string".to_string()));
    }

    match &*value {
        RuntimeValue::String(s) => Ok(RuntimeValue::string(s.clone())),
        _ => Ok(RuntimeValue::string(value.to_string_serialize())),
    }
}

// Convert a value to a number.
pub fn float(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("float", args, 1, Some(1))?;
    let value = execute_expression(&args[0], context)?;

    let num = match &*value {
        RuntimeValue::Number(n) => Ok(*n),
        RuntimeValue::String(s) => {
            // Trim whitespace before parsing
            let trimmed = s.trim();
            trimmed.parse::<f64>().map_err(|_| ExecutionError::Custom(format!("Cannot convert string to float: {}", s)))
        }
        RuntimeValue::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
        RuntimeValue::Null => Ok(0.0),
        _ => Err(ExecutionError::TypeMismatch(format!("Cannot convert {} to float", value.get_type()))),
    };

    let num = num?;
    Ok(RuntimeValue::number(num))
}

// ============================================================================
// MATHEMATICAL FUNCTIONS
// ============================================================================

// Generate a range of numbers.
pub fn range(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
    validate_args("range", args, 1, Some(3))?;

    let validate_integer = |value: Rc<RuntimeValue>, arg_name: &str| -> Result<i64, ExecutionError> {
        match &*value {
            RuntimeValue::Number(n) => {
                if n.fract() == 0.0 {
                    Ok(*n as i64)
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
            return Ok(RuntimeValue::array(vec![]));
        }
        let mut result = Vec::new();
        let mut current = start;
        while current > stop {
            result.push(RuntimeValue::number(current as f64));
            current += step;
        }
        return Ok(RuntimeValue::array(result));
    }

    // Validate positive step.
    if step <= 0 {
        return Err(ExecutionError::Custom("range: step must be greater than 0".to_string()));
    }

    // Check if step direction matches stop - start direction.
    if (stop - start).signum() != step.signum() && stop != start {
        return Ok(RuntimeValue::array(vec![]));
    }

    let mut result = Vec::new();
    let mut current = start;
    while current < stop {
        result.push(RuntimeValue::number(current as f64));
        current += step;
    }

    Ok(RuntimeValue::array(result))
}

// Provide a statistical summary of numbers.
pub fn summarize(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
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
    result.insert("max".to_string(), RuntimeValue::number(max));
    result.insert("min".to_string(), RuntimeValue::number(min));
    result.insert("mean".to_string(), RuntimeValue::number(mean));
    result.insert("median".to_string(), RuntimeValue::number(median));
    result.insert("variance".to_string(), RuntimeValue::number(variance));
    result.insert("stddev".to_string(), RuntimeValue::number(stddev));

    Ok(RuntimeValue::object(result))
}

// ============================================================================
// INDEXING FUNCTION
// ============================================================================

pub fn index(args: &[Expression], context: &mut ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
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
fn index_single(index: Rc<RuntimeValue>, operand: Rc<RuntimeValue>) -> Result<Rc<RuntimeValue>, ExecutionError> {
    match &*operand {
        RuntimeValue::Array(arr) => {
            let index_num = assert_number(index)?;
            if index_num.fract() != 0.0 {
                return Err(ExecutionError::Custom("index: Non-integers cannot be used on arrays".to_string()));
            };

            let len = arr.len() as isize;

            // Handle negative indices.
            let actual_index = if index_num < 0.0 {
                len + index_num as isize
            } else {
                index_num as isize
            };

            // Handle regular and out of bounds.
            Ok(arr.get(actual_index as usize).cloned().unwrap_or(RuntimeValue::null()))
        }
        RuntimeValue::String(s) => {
            let index_num = assert_number(index)?;
            if index_num.fract() != 0.0 {
                return Err(ExecutionError::Custom("index: Non-integers cannot be used on strings".to_string()));
            }

            let len = s.len() as isize;
            let actual_index = if index_num < 0.0 {
                len + index_num as isize
            } else {
                index_num as isize
            };

            Ok(s.chars()
                .nth(actual_index as usize)
                .map(|c| RuntimeValue::string(c.to_string()))
                .unwrap_or(RuntimeValue::null()))
        }
        RuntimeValue::Object(obj) => {
            let key = assert_string(index)?;
            Ok(obj.get(&key).cloned().unwrap_or(RuntimeValue::null()))
        }
        RuntimeValue::Null => {
            // Indexing null returns null, but only if index is number or string.
            match &*index {
                RuntimeValue::Number(_) | RuntimeValue::String(_) => Ok(RuntimeValue::null()),
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
fn index_double(start: Rc<RuntimeValue>, end: Rc<RuntimeValue>, operand: Rc<RuntimeValue>) -> Result<Rc<RuntimeValue>, ExecutionError> {
    match &*operand {
        RuntimeValue::Array(arr) => {
            let start_idx = match &*start {
                RuntimeValue::Number(n) => *n as isize,
                RuntimeValue::Null => 0,
                _ => {
                    return Err(ExecutionError::TypeMismatch(format!(
                        "Slice start must be number or null, got {:?}",
                        start.get_type()
                    )))
                }
            };

            let end_idx = match &*end {
                RuntimeValue::Number(n) => *n as isize,
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
                Ok(RuntimeValue::array(vec![]))
            } else {
                Ok(RuntimeValue::array(arr[start_idx..end_idx].to_vec()))
            }
        }
        RuntimeValue::String(s) => {
            let start_idx = match &*start {
                RuntimeValue::Number(n) => *n as isize,
                RuntimeValue::Null => 0,
                _ => {
                    return Err(ExecutionError::TypeMismatch(format!(
                        "Slice start must be number or null, got {:?}",
                        start.get_type()
                    )))
                }
            };

            let end_idx = match &*end {
                RuntimeValue::Number(n) => *n as isize,
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
                Ok(RuntimeValue::string(String::new()))
            } else {
                // Handle Unicode characters properly
                let chars: Vec<char> = s.chars().collect();
                if start_idx >= chars.len() || end_idx > chars.len() {
                    Ok(RuntimeValue::string(String::new()))
                } else {
                    Ok(RuntimeValue::string(chars[start_idx..end_idx].iter().collect()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LiteralValue;
    use crate::types::RuntimeValueType;

    fn create_test_context() -> ExecutionContext {
        let data = RuntimeValue::object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), RuntimeValue::string("John".to_string()));
            map.insert("age".to_string(), RuntimeValue::number(30.0));
            map
        });

        ExecutionContext::with_builtins(data)
    }

    #[test]
    fn test_count_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ArrayExpression {
            items: vec![
                Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                Expression::ValueExpression { value: LiteralValue::Number(3.0) },
            ],
        }];

        let result = count(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::number(3.0));
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
                            value: LiteralValue::Number(2.0),
                        },
                    ],
                ),
                Expression::ValueExpression {
                    value: LiteralValue::Number(0.0),
                },
            ],
        );

        let args = vec![
            condition,
            Expression::ArrayExpression {
                items: vec![
                    Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(4.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(5.0) },
                ],
            },
        ];

        let result = filter(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 2);
            assert_eq!(*arr[0], RuntimeValue::Number(2.0));
            assert_eq!(*arr[1], RuntimeValue::Number(4.0));
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
                    value: LiteralValue::Number(2.0),
                },
            ],
        );

        let args = vec![
            transformation,
            Expression::ArrayExpression {
                items: vec![
                    Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                ],
            },
        ];

        let result = map(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 3);
            assert_eq!(*arr[0], RuntimeValue::Number(2.0));
            assert_eq!(*arr[1], RuntimeValue::Number(4.0));
            assert_eq!(*arr[2], RuntimeValue::Number(6.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_sum_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ArrayExpression {
            items: vec![
                Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                Expression::ValueExpression { value: LiteralValue::Number(3.0) },
            ],
        }];

        let result = sum(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::number(6.0));
    }

    #[test]
    fn test_keys_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ObjectExpression {
            entries: {
                let mut map = HashMap::new();
                map.insert("a".to_string(), Expression::ValueExpression { value: LiteralValue::Number(1.0) });
                map.insert("b".to_string(), Expression::ValueExpression { value: LiteralValue::Number(2.0) });
                map
            },
        }];

        let result = keys(&args, &mut context).unwrap();

        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 2);
            // Keys order is not guaranteed, so we check that both keys are present
            let keys: Vec<String> = arr
                .iter()
                .map(|v| match &**v {
                    RuntimeValue::String(s) => s.clone(),
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
            value: LiteralValue::Number(42.0),
        }];

        let result = string(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::string("42".to_string()));
    }

    #[test]
    fn test_if_function() {
        let mut context = create_test_context();

        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::Boolean(true),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("yes".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("no".to_string()),
            },
        ];

        let result = if_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::string("yes".to_string()));

        // Test false condition
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::Boolean(false),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("yes".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("no".to_string()),
            },
        ];

        let result = if_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::string("no".to_string()));
    }

    #[test]
    fn test_sort_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ArrayExpression {
            items: vec![
                Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                Expression::ValueExpression { value: LiteralValue::Number(2.0) },
            ],
        }];

        let result = sort(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 3);
            assert_eq!(*arr[0], RuntimeValue::Number(1.0));
            assert_eq!(*arr[1], RuntimeValue::Number(2.0));
            assert_eq!(*arr[2], RuntimeValue::Number(3.0));
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
                    value: LiteralValue::Number(4.0),
                },
            ],
        );

        let args = vec![
            transformation,
            Expression::ArrayExpression {
                items: vec![
                    Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(8.0) },
                ],
            },
        ];

        let result = sortby(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 4);
            assert_eq!(*arr[0], RuntimeValue::Number(8.0)); // 8 % 4 = 0
            assert_eq!(*arr[1], RuntimeValue::Number(1.0)); // 1 % 4 = 1
            assert_eq!(*arr[2], RuntimeValue::Number(2.0)); // 2 % 4 = 2
            assert_eq!(*arr[3], RuntimeValue::Number(3.0)); // 3 % 4 = 3
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
                            value: LiteralValue::Number(0.0),
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
                            value: LiteralValue::Number(1.0),
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
                value: LiteralValue::Number(0.0),
            }, // initial
            Expression::ArrayExpression {
                items: vec![
                    Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                ],
            },
        ];

        let result = reduce(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::number(6.0));
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
            Expression::ArrayExpression {
                items: vec![
                    Expression::ObjectExpression {
                        entries: {
                            let mut map = HashMap::new();
                            map.insert("gender".to_string(), Expression::ValueExpression { value: LiteralValue::String("female".to_string()) });
                            map.insert("name".to_string(), Expression::ValueExpression { value: LiteralValue::String("hayley".to_string()) });
                            map
                        },
                    },
                    Expression::ObjectExpression {
                        entries: {
                            let mut map = HashMap::new();
                            map.insert("gender".to_string(), Expression::ValueExpression { value: LiteralValue::String("male".to_string()) });
                            map.insert("name".to_string(), Expression::ValueExpression { value: LiteralValue::String("abhik".to_string()) });
                            map
                        },
                    },
                    Expression::ObjectExpression {
                        entries: {
                            let mut map = HashMap::new();
                            map.insert("gender".to_string(), Expression::ValueExpression { value: LiteralValue::String("female".to_string()) });
                            map.insert("name".to_string(), Expression::ValueExpression { value: LiteralValue::String("emily".to_string()) });
                            map
                        },
                    },
                ],
            },
        ];

        let result = groupby(&args, &mut context).unwrap();
        println!("result: {}", result.to_string_display());
        if let RuntimeValue::Object(groups) = &*result {
            assert_eq!(groups.len(), 2);
            assert!(groups.contains_key("female"));
            assert!(groups.contains_key("male"));

            if let RuntimeValue::Array(female_group) = &**groups.get("female").unwrap() {
                assert_eq!(female_group.len(), 2);
            }

            if let RuntimeValue::Array(male_group) = &**groups.get("male").unwrap() {
                assert_eq!(male_group.len(), 1);
            }
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_withindices_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ArrayExpression {
            items: vec![
                Expression::ValueExpression { value: LiteralValue::String("a".to_string()) },
                Expression::ValueExpression { value: LiteralValue::String("b".to_string()) },
                Expression::ValueExpression { value: LiteralValue::String("c".to_string()) },
            ],
        }];

        let result = withindices(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 3);

            // Check first pair
            if let RuntimeValue::Array(pair) = arr[0].as_ref() {
                assert_eq!(pair[0], RuntimeValue::number(0.0));
                assert_eq!(pair[1], RuntimeValue::string("a".to_string()));
            }

            // Check second pair
            if let RuntimeValue::Array(pair) = arr[1].as_ref() {
                assert_eq!(pair[0], RuntimeValue::number(1.0));
                assert_eq!(pair[1], RuntimeValue::string("b".to_string()));
            }

            // Check third pair
            if let RuntimeValue::Array(pair) = arr[2].as_ref() {
                assert_eq!(pair[0], RuntimeValue::number(2.0));
                assert_eq!(pair[1], RuntimeValue::string("c".to_string()));
            }
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_entries_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ObjectExpression {
            entries: {
                let mut map = HashMap::new();
                map.insert("foo".to_string(), Expression::ValueExpression { value: LiteralValue::String("bar".to_string()) });
                map.insert("baz".to_string(), Expression::ValueExpression { value: LiteralValue::Number(42.0) });
                map
            },
        }];

        let result = entries(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 2);

            // Check that entries are [key, value] pairs
            for entry in arr.iter() {
                if let RuntimeValue::Array(pair) = &**entry {
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

        let args = vec![Expression::ArrayExpression {
            items: vec![
                Expression::ArrayExpression {
                    items: vec![
                        Expression::ValueExpression { value: LiteralValue::String("foo".to_string()) },
                        Expression::ValueExpression { value: LiteralValue::String("bar".to_string()) },
                    ],
                },
                Expression::ArrayExpression {
                    items: vec![
                        Expression::ValueExpression { value: LiteralValue::String("baz".to_string()) },
                        Expression::ValueExpression { value: LiteralValue::Number(42.0) },
                    ],
                },
            ],
        }];

        let result = fromentries(&args, &mut context).unwrap();
        if let RuntimeValue::Object(obj) = &*result {
            assert_eq!(obj.len(), 2);
            assert_eq!(obj.get("foo").unwrap(), &RuntimeValue::string("bar".to_string()));
            assert_eq!(obj.get("baz").unwrap(), &RuntimeValue::number(42.0));
        } else {
            panic!("Expected object result");
        }
    }

    #[test]
    fn test_range_function() {
        let mut context = create_test_context();

        // Test range(5) -> [0, 1, 2, 3, 4]
        let args = vec![Expression::ValueExpression {
            value: LiteralValue::Number(5.0),
        }];

        let result = range(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 5);
            assert_eq!(*arr[0], RuntimeValue::Number(0.0));
            assert_eq!(*arr[1], RuntimeValue::Number(1.0));
            assert_eq!(*arr[2], RuntimeValue::Number(2.0));
            assert_eq!(*arr[3], RuntimeValue::Number(3.0));
            assert_eq!(*arr[4], RuntimeValue::Number(4.0));
        } else {
            panic!("Expected array result");
        }

        // Test range(3, 8) -> [3, 4, 5, 6, 7]
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::Number(3.0),
            },
            Expression::ValueExpression {
                value: LiteralValue::Number(8.0),
            },
        ];

        let result = range(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 5);
            assert_eq!(*arr[0], RuntimeValue::Number(3.0));
            assert_eq!(*arr[4], RuntimeValue::Number(7.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_summarize_function() {
        let mut context = create_test_context();

        let args = vec![Expression::ArrayExpression {
            items: vec![
                Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                Expression::ValueExpression { value: LiteralValue::Number(4.0) },
                Expression::ValueExpression { value: LiteralValue::Number(5.0) },
                Expression::ValueExpression { value: LiteralValue::Number(6.0) },
            ],
        }];

        let result = summarize(&args, &mut context).unwrap();
        if let RuntimeValue::Object(stats) = &*result {
            assert_eq!(stats.get("max").unwrap(), &RuntimeValue::number(6.0));
            assert_eq!(stats.get("min").unwrap(), &RuntimeValue::number(1.0));
            assert_eq!(stats.get("mean").unwrap(), &RuntimeValue::number(3.5));
            assert_eq!(stats.get("median").unwrap(), &RuntimeValue::number(3.5));
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
                value: LiteralValue::Number(1.0),
            },
            Expression::ArrayExpression {
                items: vec![
                    Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                ],
            },
        ];

        let result = index(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::number(2.0));

        // Test object indexing: index "key" {key: "value"} -> "value"
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::String("key".to_string()),
            },
            Expression::ObjectExpression {
                entries: {
                    let mut map = HashMap::new();
                    map.insert("key".to_string(), Expression::ValueExpression { value: LiteralValue::String("value".to_string()) });
                    map
                },
            },
        ];

        let result = index(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::string("value".to_string()));

        // Test slicing: index 1 3 [1, 2, 3, 4, 5] -> [2, 3]
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::Number(1.0),
            },
            Expression::ValueExpression {
                value: LiteralValue::Number(3.0),
            },
            Expression::ArrayExpression {
                items: vec![
                    Expression::ValueExpression { value: LiteralValue::Number(1.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(2.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(3.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(4.0) },
                    Expression::ValueExpression { value: LiteralValue::Number(5.0) },
                ],
            },
        ];

        let result = index(&args, &mut context).unwrap();
        if let RuntimeValue::Array(arr) = &*result {
            assert_eq!(arr.len(), 2);
            assert_eq!(*arr[0], RuntimeValue::Number(2.0));
            assert_eq!(*arr[1], RuntimeValue::Number(3.0));
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
                value: LiteralValue::String("haha".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("z".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("haha".to_string()),
            },
        ];

        let result = replace(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::string("z".to_string()));
    }

    #[test]
    fn test_match_function() {
        let mut context = create_test_context();

        // Test string matching
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::String("test".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("this is a test".to_string()),
            },
        ];

        let result = match_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::boolean(true));

        // Test non-matching
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::String("xyz".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("this is a test".to_string()),
            },
        ];

        let result = match_function(&args, &mut context).unwrap();
        assert_eq!(result, RuntimeValue::boolean(false));
    }

    #[test]
    fn test_regex_function() {
        let mut context = create_test_context();

        // Test regex creation
        let args = vec![
            Expression::ValueExpression {
                value: LiteralValue::String("test".to_string()),
            },
            Expression::ValueExpression {
                value: LiteralValue::String("i".to_string()),
            },
        ];

        let result = regex(&args, &mut context).unwrap();
        assert_eq!(result.get_type(), RuntimeValueType::Regex);
    }

    // ============================================================================
    // CUSTOM FUNCTION REGISTRY TESTS
    // ============================================================================

    #[test]
    fn test_custom_function_registry_creation() {
        let registry = CustomFunctionRegistry::new();
        assert_eq!(registry.list_functions().len(), 0);
        assert!(!registry.has_function("nonexistent"));
    }

    #[test]
    fn test_custom_function_registration() {
        let mut registry = CustomFunctionRegistry::new();

        // Define a simple custom function
        fn custom_add(args: &[Expression], context: &mut crate::executor::ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
            validate_args("custom_add", args, 2, Some(2))?;
            let left = execute_expression(&args[0], context)?;
            let right = execute_expression(&args[1], context)?;

            match (&*left, &*right) {
                (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::number(a + b)),
                _ => Err(ExecutionError::TypeMismatch("custom_add requires two numbers".to_string())),
            }
        }

        let metadata = FunctionMetadata {
            name: "custom_add".to_string(),
            min_args: 2,
            max_args: Some(2),
            description: "Add two numbers".to_string(),
        };

        // Register the function
        registry.register_function("custom_add".to_string(), custom_add, metadata).unwrap();

        assert!(registry.has_function("custom_add"));
        assert_eq!(registry.list_functions(), vec!["custom_add"]);

        let retrieved_metadata = registry.get_metadata("custom_add").unwrap();
        assert_eq!(retrieved_metadata.name, "custom_add");
        assert_eq!(retrieved_metadata.min_args, 2);
        assert_eq!(retrieved_metadata.max_args, Some(2));
        assert_eq!(retrieved_metadata.description, "Add two numbers");
    }

    #[test]
    fn test_custom_function_conflict_with_stock_builtin() {
        let mut registry = CustomFunctionRegistry::new();

        fn dummy_function(_args: &[Expression], _context: &mut crate::executor::ExecutionContext) -> Result<Rc<RuntimeValue>, ExecutionError> {
            Ok(RuntimeValue::null())
        }

        let metadata = FunctionMetadata {
            name: "count".to_string(),
            min_args: 1,
            max_args: Some(1),
            description: "Dummy count function".to_string(),
        };

        // This should fail because "count" is a stock builtin
        let result = registry.register_function("count".to_string(), dummy_function, metadata);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("conflicts with stock builtin"));
    }
}
