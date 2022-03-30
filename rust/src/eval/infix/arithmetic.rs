use crate::eval::Value;
use crate::{Error, Result};

pub fn add(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Float(l as f64 + r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l + r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
        _ => Err(Error::eval("can't add non-numbers".to_string())),
    }
}

pub fn subtract(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Float(l as f64 - r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l - r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l - r)),
        _ => Err(Error::eval("can't add non-numbers".to_string())),
    }
}

pub fn multiply(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Float(l as f64 * r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l * r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
        _ => Err(Error::eval("can't add non-numbers".to_string())),
    }
}

pub fn divide(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l / r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Float(l as f64 / r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Float(l / r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l / r)),
        _ => Err(Error::eval("can't add non-numbers".to_string())),
    }
}

pub fn modulo(left: Value, right: Value) -> Result<Value> {
    if let (Value::Int(l), Value::Int(r)) = (left, right) {
        Ok(Value::Int(l % r))
    } else {
        Err(Error::eval("can't modulo non-integers".to_string()))
    }
}
