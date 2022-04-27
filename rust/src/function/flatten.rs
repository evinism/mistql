use super::args::ArgParser;
use crate::{Error, Result, Value};

pub fn flatten(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;
    match arg {
        Value::Array(val) => {
            let mut flattened: Vec<Value> = vec![];
            for elt in val.into_iter() {
                if let Value::Array(a) = elt {
                    a.into_iter().for_each(|e| flattened.push(e));
                } else {
                    return Err(Error::eval(format!("can't flatten non-array {}", elt)));
                }
            }
            Ok(Value::Array(flattened))
        }
        _ => Err(Error::eval(format!("flatten expected array, got {}", arg))),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn flatten_takes_one_arg() {
        assert!(query_value(
            "flatten [[1, 2], [3, 4]]".to_string(),
            serde_json::Value::Null
        )
        .is_ok());
        assert!(query_value(
            "flatten [[1, 2], [3, 4]] [[5,6], [7,8".to_string(),
            serde_json::Value::Null
        )
        .is_err());
    }

    #[test]
    fn flatten_only_flattens_arrays() {
        assert!(query_value("flatten 1".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("flatten \"abc\"".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("flatten {a: {b: 2}}".to_string(), serde_json::Value::Null).is_err());
        assert!(query_value("flatten [[1,2], 3]".to_string(), serde_json::Value::Null).is_err());
    }
}
