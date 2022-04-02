use crate::eval::Value;
use crate::{Error, Result};

pub fn count(args: Vec<Value>) -> Result<Value> {
    if let Some(Value::Array(vals)) = args.get(0) {
        Ok(Value::Int(vals.len() as i64))
    } else {
        Err(Error::eval(format!(
            "argument to count must be an array (got {:?}",
            args
        )))
    }
}

pub fn entries(args: Vec<Value>) -> Result<Value> {
    if let Some(Value::Object(obj)) = args.get(0) {
        Ok(Value::Array(
            obj.iter()
                .map(|(k, v)| Value::Array(vec![k.clone(), v.clone()]))
                .collect::<Result<Vec<Value>>>()?,
        ))
    } else {
        Err(Error::eval(format!(
            "argument to entries must be an object (got {:?}",
            args
        )))
    }
}

pub fn float(args: Vec<Value>) -> Result<Value> {
    if let Some(val) = args.get(0) {
        match val {
            Value::Float(_) => Ok(val.clone()),
            Value::Int(num) => Ok(Value::Float(*num as f64)),
            Value::String(string) => match string.trim().parse::<f64>() {
                Ok(val) => Ok(Value::Float(val)),
                Err(err) => Err(Error::eval(err.to_string())),
            },
            Value::Boolean(true) => Ok(Value::Float(1.0)),
            Value::Boolean(false) => Ok(Value::Float(0.0)),
            Value::Null => Ok(Value::Float(0.0)),
            _ => Err(Error::eval("argument does not cast to float".to_string())),
        }
    } else {
        Err(Error::eval("float requires one argument".to_string()))
    }
}

pub fn log(args: Vec<Value>) -> Result<Value> {
    if let Some(val) = args.get(0) {
        Ok(val.clone())
    } else {
        Err(Error::eval("log requires one argument".to_string()))
    }
}

pub fn string(args: Vec<Value>) -> Result<Value> {
    if let Some(val) = args.get(0) {
        Ok(Value::String(val.to_string()))
    } else {
        Err(Error::eval("string requires one argument".to_string()))
    }
}
