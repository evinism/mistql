use super::{Number, Value};
use crate::{Error, Result};
use std::collections::BTreeMap;

impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Number(Number::Int(i))
                } else if let Some(f) = n.as_f64() {
                    Value::Number(Number::Float(f))
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(a) => {
                Value::Array(a.iter().map(|elt| elt.clone().into()).collect())
            }
            serde_json::Value::Object(o) => Value::Object(BTreeMap::from_iter(
                o.iter().map(|(k, v)| (k.clone(), v.clone().into())),
            )),
        }
    }
}

impl TryFrom<Value> for serde_json::Value {
    type Error = Error;

    fn try_from(val: Value) -> Result<Self> {
        match val {
            Value::Null => Ok(serde_json::Value::Null),
            Value::Boolean(b) => Ok(serde_json::Value::Bool(b)),
            Value::Number(Number::Int(i)) => Ok(serde_json::Value::from(i)),
            Value::Number(Number::Float(f)) => Ok(serde_json::Value::from(f)),
            Value::String(s) => Ok(serde_json::Value::String(s)),
            Value::Array(a) => Ok(serde_json::Value::Array(
                a.iter()
                    .map(|elt| elt.clone().try_into())
                    .collect::<Result<Vec<serde_json::Value>>>()?,
            )),
            Value::Object(o) => {
                let mut fields: serde_json::Map<std::string::String, serde_json::Value> =
                    serde_json::Map::new();
                for (k, v) in o.iter() {
                    fields.insert(k.clone(), v.clone().try_into()?);
                }
                Ok(serde_json::Value::Object(fields))
            }
            Value::Ident(_) => Err(Error::eval("can't convert ident to JSON".to_string())),
            Value::Regex(_, _) => Err(Error::eval("can't convert regex to JSON".to_string())),
        }
    }
}
