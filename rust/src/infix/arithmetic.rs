use crate::{Error, Number, Result, Value};

pub fn add(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
        (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
        (Value::Array(l), Value::Array(r)) => Ok(Value::Array(vec![l, r].concat())),
        _ => Err(Error::eval("invalid types for addition".to_string())),
    }
}

pub fn subtract(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
        _ => Err(Error::eval("can't subtract non-numbers".to_string())),
    }
}

pub fn multiply(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
        _ => Err(Error::eval("can't multiply non-numbers".to_string())),
    }
}

pub fn divide(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) if r != Number::Int(0) => Ok(Value::Number(l / r)),
        (Value::Number(_), Value::Number(_)) => {
            Err(Error::eval("can't divide by zero".to_string()))
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

pub fn sqrt(val: Value) -> Result<Value> {
    match val {
        Value::Number(Number::Int(num)) if num > 0 => {
            Ok(Value::Number(Number::Float((num as f64).sqrt())))
        }
        Value::Number(Number::Float(num)) if num > 0.0 => {
            Ok(Value::Number(Number::Float(num.sqrt())))
        }
        Value::Number(Number::Int(_)) | Value::Number(Number::Float(_)) => Err(
            Error::unimplemented("no support for imaginary numbers".to_string()),
        ),
        _ => Err(Error::eval(
            "can't take square root of non-numbers".to_string(),
        )),
    }
}
