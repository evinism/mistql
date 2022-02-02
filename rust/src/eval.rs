use crate::parse::{Expression, Literal, Value};
use crate::Result;

impl<'a> Expression<'a> {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::Value(val) => val.evaluate(context),
        }
    }
}

impl<'a> Value<'a> {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::At => Ok(context.clone()),
            Self::Literal(val) => val.evaluate(context),
            Self::EOI => Ok(context.clone()),
        }
    }
}

impl<'a> Literal<'a> {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::Object(obj) => Ok(obj
                .iter()
                .map(|(key, val)| (key.clone(), val.evaluate(context).unwrap()))
                .collect()),
            Self::Array(arr) => arr.iter().map(|elt| elt.evaluate(context)).collect(),
            Self::String(str) => Ok(serde_json::from_str(str).unwrap()),
            Self::Number(num) => Ok(num.clone().into()),
            Self::Boolean(bool) => Ok(bool.clone().into()),
            Self::Null => Ok(serde_json::Value::Null),
        }
    }
}

#[cfg(test)]
mod tests {}
