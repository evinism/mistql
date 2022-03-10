use crate::{Error, Result};

pub fn float(args: Vec<serde_json::Value>) -> Result<serde_json::Value> {
    if let Some(val) = args.get(0) {
        match val {
            // serde_json::from_str(pair.as_str());
            serde_json::Value::Number(num) => match num.as_f64() {
                Some(float_num) => Ok(serde_json::Value::from(float_num)),
                None => Err(Error::eval(format!("invalid number {}", num))),
            },
            serde_json::Value::String(string) => match string.trim().parse::<f64>() {
                Ok(val) => Ok(val.into()),
                Err(err) => Err(Error::eval(err.to_string())),
            },
            serde_json::Value::Bool(true) => Ok(serde_json::Value::from(1.0)),
            serde_json::Value::Bool(false) => Ok(serde_json::Value::from(0.0)),
            serde_json::Value::Null => Ok(serde_json::Value::from(0.0)),
            _ => Err(Error::eval("argument does not cast to float".to_string())),
        }
    } else {
        Err(Error::eval("float requires one argument".to_string()))
    }
}
