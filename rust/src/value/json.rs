use super::Value;
use std::collections::BTreeMap;

impl From<serde_json::Value> for Value {
    fn from(val: serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Int(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
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

impl From<Value> for serde_json::Value {
    fn from(val: Value) -> Self {
        match val {
            Value::Null => serde_json::Value::Null,
            Value::Boolean(b) => serde_json::Value::Bool(b),
            Value::Int(i) => serde_json::Value::from(i),
            Value::Float(f) => serde_json::Value::from(f),
            Value::String(s) => serde_json::Value::String(s),
            Value::Array(a) => {
                serde_json::Value::Array(a.iter().map(|elt| elt.clone().into()).collect())
            }
            Value::Object(o) => serde_json::Value::Object(serde_json::Map::from_iter(
                o.iter().map(|(k, v)| (k.clone(), v.clone().into())),
            )),
            Value::Ident(s) => serde_json::Value::String(s),
            Value::Regex(exp, Some(flags)) => {
                serde_json::Value::String(format!("regex {} {}", exp, flags))
            }
            Value::Regex(exp, None) => serde_json::Value::String(format!("regex {}", exp)),
        }
    }
}
