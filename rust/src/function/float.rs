use crate::{Error, Result, Value};

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
