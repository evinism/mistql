use crate::infix::arithmetic::add;
use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn sum(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("sum requires one argument".to_string())),
    };

    match arg {
        Value::Array(arr) => arr
            .iter()
            .try_fold(Value::Int(0), |acc, x| add(acc, x.clone())),
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
