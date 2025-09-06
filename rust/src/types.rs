//! Runtime value types for MistQL

/// MistQL runtime value representing all possible data types
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(std::collections::HashMap<String, RuntimeValue>),
    Array(Vec<RuntimeValue>),
    Function(String), // TODO: Implement proper function type
    Regex(String),    // TODO: Implement proper regex type
}

// TODO: Implement type conversion, equality, comparison, and truthiness operations
