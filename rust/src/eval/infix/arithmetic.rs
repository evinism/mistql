use crate::{Error, Result};

pub fn add(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l + r).into())
    } else if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l + r).into())
    } else {
        Err(Error::eval("can't add non-numbers".to_string()))
    }
}

pub fn subtract(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l - r).into())
    } else if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l - r).into())
    } else {
        Err(Error::eval("can't subtract non-numbers".to_string()))
    }
}

pub fn multiply(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l * r).into())
    } else if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l * r).into())
    } else {
        Err(Error::eval("can't multiply non-numbers".to_string()))
    }
}

pub fn divide(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l / r).into())
    } else if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l / r).into())
    } else {
        Err(Error::eval("can't divide non-numbers".to_string()))
    }
}

pub fn modulo(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_i64(), right.as_i64()) {
        Ok((l % r).into())
    } else {
        Err(Error::eval("can't modulo non-integers".to_string()))
    }
}
