use crate::error::Result;
use crate::parse::{Expression, Value};

impl Expression {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::At => Ok(context.clone()),
            Self::Value(val) => val.evaluate(),
            Self::EOI => Ok(context.clone()),
        }
    }
}

impl Value {
    pub fn evaluate(&self) -> Result<serde_json::Value> {
        match self {
            Self::Number(num) if num.fract() != 0.0 => Ok(num.clone().into()),
            Self::Number(num) => Ok((num.clone() as i64).into()),
            Self::Null => Ok(serde_json::Value::Null),
        }
    }
}

#[cfg(test)]
mod tests {}
