use crate::eval::Value;
use crate::{Error, Result};

pub fn gte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Boolean(l >= r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Boolean(l as f64 >= r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Boolean(l >= r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Boolean(l >= r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l >= r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn gt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Boolean(l > r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Boolean(l as f64 > r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Boolean(l > r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Boolean(l > r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l > r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn lte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Boolean(l <= r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Boolean(l as f64 <= r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Boolean(l <= r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Boolean(l <= r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l <= r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn lt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => Ok(Value::Boolean(l < r)),
        (Value::Int(l), Value::Float(r)) => Ok(Value::Boolean((l as f64) < r)),
        (Value::Float(l), Value::Int(r)) => Ok(Value::Boolean(l < r as f64)),
        (Value::Float(l), Value::Float(r)) => Ok(Value::Boolean(l < r)),
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l < r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn eq(left: Value, right: Value) -> Result<Value> {
    Ok(Value::Boolean(left == right))
}

pub fn ne(left: Value, right: Value) -> Result<Value> {
    Ok(Value::Boolean(left != right))
}
