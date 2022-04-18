use crate::{Error, Number, Result, Value};

pub fn gte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Boolean(l >= r))
        }
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l as f64 >= r))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Boolean(l >= r as f64))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l >= r))
        }
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l >= r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn gt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => Ok(Value::Boolean(l > r)),
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l as f64 > r))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Boolean(l > r as f64))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l > r))
        }
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l > r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn lte(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Boolean(l <= r))
        }
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l as f64 <= r))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Boolean(l <= r as f64))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l <= r))
        }
        (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l <= r)),
        _ => Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        )),
    }
}

pub fn lt(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => Ok(Value::Boolean(l < r)),
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean((l as f64) < r))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Boolean(l < r as f64))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Boolean(l < r))
        }
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
