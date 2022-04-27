use super::args::ArgParser;
use crate::{Error, Result, Value};

pub fn string(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;

    if let Value::Regex(_, _) = arg {
        Err(Error::eval("can't cast regex to string".to_string()))
    } else {
        Ok(Value::String(arg.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn string_takes_one_arg() {
        assert!(query_value("string [1,2,3]".to_string(), serde_json::Value::Null).is_ok());
        assert!(query_value("string [1,2,3] 4".to_string(), serde_json::Value::Null).is_err());
    }

    // since this function delegates to the Display trait on value, unit
    // tests appear there
}
