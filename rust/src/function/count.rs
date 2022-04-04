use crate::{Error, Result, Value};

pub fn count(args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(Value::Array(val))) => Ok(Value::Int(val.len() as i64)),
        (1, Some(val)) => Err(Error::eval(format!(
            "argument to count must be an array (got {:?}",
            val
        ))),
        (n, _) => Err(Error::eval(format!("count expected 1 argument, got {}", n))),
    }
}

#[cfg(test)]
mod tests {
    use super::count;
    use crate::Value;

    #[test]
    fn count_takes_one_arg() {
        assert!(count(vec![]).is_err());
        assert!(count(vec![Value::Array(vec![])]).is_ok());
        assert!(count(vec![Value::Array(vec![]), Value::Array(vec![])]).is_err());
    }

    #[test]
    fn count_arg_must_be_an_array() {
        assert!(count(vec![Value::Int(1)]).is_err());
        assert!(count(vec![Value::String("abc".to_string())]).is_err());
    }

    #[test]
    fn count_counts() {
        assert_eq!(Value::Int(0), count(vec![Value::Array(vec![])]).unwrap());

        assert_eq!(
            Value::Int(3),
            count(vec![Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ])])
            .unwrap()
        );
    }
}
