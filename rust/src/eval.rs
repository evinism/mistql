use crate::parse::{Expression, Value};
use crate::Result;

impl<'a> Expression<'a> {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::At => Ok(context.clone()),
            Self::Value(val) => val.evaluate(),
            Self::EOI => Ok(context.clone()),
        }
    }
}

impl<'a> Value<'a> {
    pub fn evaluate(&self) -> Result<serde_json::Value> {
        match self {
            Self::String(str) => Ok(serde_json::from_str(str).unwrap()),
            Self::Number(num) => Ok(num.clone().into()),
            Self::Null => Ok(serde_json::Value::Null),
        }
    }
}

#[cfg(test)]
mod tests {}
