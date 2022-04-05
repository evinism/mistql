use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn log(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    match (context_opt, arg_itr.next(), arg_itr.next()) {
        (context, Some(arg), None) => match expr::eval(arg, data, context) {
            Ok(result) => {
                dbg!(result.clone());
                Ok(result)
            }
            Err(err) => Err(err),
        },
        (Some(context), None, None) => {
            dbg!(context.clone());
            Ok(context)
        }
        (None, None, _) => Err(Error::eval(
            "log requires one argument (got zero)".to_string(),
        )),
        (_, _, Some(_)) => Err(Error::eval(
            "log requires one argument (got >1)".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn log_takes_one_arg() {
        assert!(query_value("log 1".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("log 1 2".to_string(), serde_json::Value::Null).is_err());
    }

    #[test]
    fn log_uses_implicit_at() {
        assert_eq!(
            query_value("123 | log".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(123)
        );
    }

    #[test]
    fn log_uses_explicit_at() {
        assert_eq!(
            query_value("123 | log @".to_string(), serde_json::Value::Null).unwrap(),
            serde_json::Value::from(123)
        );
    }
}
