use super::args::ArgParser;
use crate::infix::arithmetic::add;
use crate::{Error, Number, Result, Value};

pub fn sum(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;

    match arg {
        Value::Array(arr) => arr
            .into_iter()
            .try_fold(Value::Number(Number::Int(0)), |acc, x| add(acc, x)),
        _ => Err(Error::eval(format!(
            "argument to sum must be an array (got {:?}",
            arg
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn sum_takes_one_arg() {
        assert!(query_value("sum [1,2,3]".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("sum [1,2,3] null".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn sum_arg_must_be_an_array() {
        assert!(query_value("sum \"123\"".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("sum {a: 1, b: 2}".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn sum_arg_must_only_contain_numbers() {
        assert!(query_value("sum [1,2.0,3.75]".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("sum [1,true,3]".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("sum [1,2,nukll]".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn sum_sums() {
        assert_eq!(
            query_value("sum []".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(0 as i64)
        );

        assert_eq!(
            query_value("sum [1,2,3]".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(6 as i64)
        );

        assert_eq!(
            query_value("sum [1,2.0,3.75]".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(6.75 as f64)
        );
    }
}
