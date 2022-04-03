use crate::eval::infix::arithmetic::add;
use crate::{eval::Value, Error, Result};

pub fn sum(args: Vec<Value>) -> Result<Value> {
    match (args.len(), args.get(0)) {
        (1, Some(Value::Array(val))) => val
            .iter()
            .try_fold(Value::Int(0), |acc, x| add(acc, x.clone())),
        (1, Some(val)) => Err(Error::eval(format!(
            "argument to sum must be an array (got {:?}",
            val
        ))),
        (n, _) => Err(Error::eval(format!("sum expected 1 argument, got {}", n))),
    }
}

#[cfg(test)]
mod tests {
    use super::sum;
    use crate::eval::Value;

    #[test]
    fn sum_takes_one_arg() {
        assert!(sum(vec![]).is_err());
        assert!(sum(vec![Value::Array(vec![])]).is_ok());
        assert!(sum(vec![Value::Array(vec![]), Value::Array(vec![])]).is_err());
    }

    #[test]
    fn sum_arg_must_be_an_array() {
        assert!(sum(vec![Value::Int(1)]).is_err());
        assert!(sum(vec![Value::String("abc".to_string())]).is_err());
    }

    #[test]
    fn sum_arg_must_only_contain_numbers() {
        assert!(sum(vec![Value::Array(vec![Value::Int(1)])]).is_ok());

        assert!(sum(vec![Value::Array(vec![Value::Int(1), Value::Float(2.0)])]).is_ok());

        assert!(sum(vec![Value::Array(vec![
            Value::Int(1),
            Value::Boolean(true)
        ])])
        .is_err());
    }

    #[test]
    fn sum_sums() {
        assert_eq!(Value::Int(0), sum(vec![Value::Array(vec![])]).unwrap());

        assert_eq!(
            Value::Int(6),
            sum(vec![Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ])])
            .unwrap()
        );

        assert_eq!(
            Value::Float(0.0),
            sum(vec![Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Float(-3.0)
            ])])
            .unwrap()
        );
    }
}
