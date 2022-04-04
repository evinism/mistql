use crate::{Error, Result, Value};

pub fn string(args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(val)) => Ok(Value::String(val.to_string())),
        (n, _) => Err(Error::eval(format!("count expected 1 argument, got {}", n))),
    }
}

#[cfg(test)]
mod tests {
    use super::string;
    use crate::Value;

    #[test]
    fn string_takes_one_arg() {
        assert!(string(vec![Value::Int(1)]).is_ok());
        assert!(string(vec![]).is_err());
        assert!(string(vec![Value::Int(1), Value::Float(2.0)]).is_err());
    }

    // since this function delegates to the Display trait on value, unit
    // tests appear there
}
