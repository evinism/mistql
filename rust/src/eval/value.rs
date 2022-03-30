use std::collections::HashMap;
use std::convert::TryFrom;

use crate::Error;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl TryFrom<serde_json::Value> for Value {
    type Error = Error;

    fn try_from(val: serde_json::Value) -> Result<Self, Self::Error> {
        match val {
            serde_json::Value::Null => Ok(Value::Null),
            serde_json::Value::Bool(b) => Ok(Value::Boolean(b)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(Value::Int(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(Value::Float(f))
                } else {
                    Err(Error::query(format!("{} is not a number", n)))
                }
            }
            serde_json::Value::String(s) => Ok(Value::String(s)),
            _ => Err(Error::unimplemented(format!("json -> value {:?}", val))),
        }
    }
}

impl TryFrom<Value> for serde_json::Value {
    type Error = Error;

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        match val {
            Value::Null => Ok(serde_json::Value::Null),
            Value::Boolean(b) => Ok(serde_json::Value::Bool(b)),
            Value::Int(i) => Ok(serde_json::Value::from(i)),
            Value::Float(f) => Ok(serde_json::Value::from(f)),
            Value::String(s) => Ok(serde_json::Value::String(s)),
            Value::Array(a) => Ok(serde_json::Value::Array(
                a.iter()
                    .map(|elt| elt.clone().try_into())
                    .collect::<Result<Vec<serde_json::Value>, Error>>()?,
            )),
            Value::Object(o) => Ok(serde_json::Value::Object(object_to_json(o)?)),
        }
    }
}

fn object_to_json(
    obj: HashMap<String, Value>,
) -> Result<serde_json::Map<String, serde_json::Value>, Error> {
    let pairs = obj
        .iter()
        .map(|(k, v)| match v.clone().try_into() {
            Ok(val) => Ok((k.clone(), val)),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<(String, serde_json::Value)>, Error>>()?;

    Ok(serde_json::Map::from_iter(pairs))
}
