use crate::{Error, Result};

pub fn gte(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l >= r).into())
    } else if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
        Ok((l >= r).into())
    } else {
        Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        ))
    }
}

pub fn gt(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l > r).into())
    } else if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
        Ok((l > r).into())
    } else {
        Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        ))
    }
}

pub fn lte(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l <= r).into())
    } else if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
        Ok((l <= r).into())
    } else {
        Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        ))
    }
}

pub fn lt(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    if let (Some(l), Some(r)) = (left.as_f64(), right.as_f64()) {
        Ok((l < r).into())
    } else if let (Some(l), Some(r)) = (left.as_str(), right.as_str()) {
        Ok((l < r).into())
    } else {
        Err(Error::eval(
            "can only compare numbers or strings".to_string(),
        ))
    }
}

pub fn eq(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    Ok((left == right).into())
}

pub fn ne(left: serde_json::Value, right: serde_json::Value) -> Result<serde_json::Value> {
    Ok((left != right).into())
}
