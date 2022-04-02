use crate::{eval::Value, Error, Result};

pub fn log(args: Vec<Value>) -> Result<Value> {
    if args.len() == 1 {
        let val = &args[0];
        dbg!(val);
        Ok(val.clone())
    } else {
        Err(Error::eval("log requires one argument".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::log;
    use crate::eval::Value;

    #[test]
    fn log_takes_one_arg() {
        assert!(log(vec![Value::Int(1)]).is_ok());
        assert!(log(vec![]).is_err());
        assert!(log(vec![Value::Int(1), Value::Float(2.0)]).is_err());
    }
}
