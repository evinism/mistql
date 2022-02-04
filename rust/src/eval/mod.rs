use crate::parse::{Expression, Literal, Operator, Value};
use crate::Result;

mod function;

impl<'a> Expression<'a> {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::Value(val) => val.evaluate(context),
            Self::Monad {
                op: Operator::Not,
                target,
            } => Ok((!is_truthy(target.evaluate(context)?)).into()),
            Self::FnCall { func, args } => function::call_fn(func.clone(), args.clone(), context),
        }
    }
}

fn is_truthy(val: serde_json::Value) -> bool {
    match val {
        serde_json::Value::Null => false,
        serde_json::Value::Number(num) => num.as_f64().unwrap() != 0.0,
        serde_json::Value::Bool(boolean) => boolean,
        serde_json::Value::String(string) => string.len() > 0,
        serde_json::Value::Array(arr) => arr.len() > 0,
        serde_json::Value::Object(obj) => obj.len() > 0,
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
