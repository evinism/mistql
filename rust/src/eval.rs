use crate::parse::{Expression, Reference, Value};

impl<'a> Expression<'a> {
    pub fn evaluate(&'a self, context: &serde_json::Value) -> Result<serde_json::Value, String> {
        match self {
            Expression::Value(value) => value.evaluate(&context),
            _ => Err("Unknown expression type".to_string()),
        }
    }
}

impl<'a> Value<'a> {
    pub fn evaluate(&'a self, context: &serde_json::Value) -> Result<serde_json::Value, String> {
        match self {
            Value::Reference(reference) => reference.evaluate(&context),
            _ => Err("Unknown value type".to_string()),
        }
    }
}

impl<'a> Reference<'a> {
    pub fn evaluate(&'a self, context: &'a serde_json::Value) -> Result<serde_json::Value, String> {
        match self {
            Reference::At => Ok(context.clone()),
            _ => Err("Unknown value type".to_string()),
        }
    }
}
