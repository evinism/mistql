use crate::{Error, Number, Result, Value};

pub fn add(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Int(l + r)))
        }
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l as f64 + r)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Float(l + r as f64)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l + r)))
        }
        (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
        (Value::Array(l), Value::Array(r)) => Ok(Value::Array(vec![l, r].concat())),
        _ => Err(Error::eval("invalid types for addition".to_string())),
    }
}

pub fn subtract(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Int(l - r)))
        }
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l as f64 - r)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Float(l - r as f64)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l - r)))
        }
        _ => Err(Error::eval("can't subtract non-numbers".to_string())),
    }
}

pub fn multiply(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Int(l * r)))
        }
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l as f64 * r)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Float(l * r as f64)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l * r)))
        }
        _ => Err(Error::eval("can't multiply non-numbers".to_string())),
    }
}

pub fn divide(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Int(l / r)))
        }
        (Value::Number(Number::Int(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l as f64 / r)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Int(r))) => {
            Ok(Value::Number(Number::Float(l / r as f64)))
        }
        (Value::Number(Number::Float(l)), Value::Number(Number::Float(r))) => {
            Ok(Value::Number(Number::Float(l / r)))
        }
        _ => Err(Error::eval("can't divide non-numbers".to_string())),
    }
}

pub fn modulo(left: Value, right: Value) -> Result<Value> {
    if let (Value::Number(Number::Int(l)), Value::Number(Number::Int(r))) = (left, right) {
        Ok(Value::Number(Number::Int(l % r)))
    } else {
        Err(Error::eval("can't modulo non-integers".to_string()))
    }
}
