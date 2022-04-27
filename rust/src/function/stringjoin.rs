use super::args::ArgParser;
use crate::{Error, Result, Value};

pub fn stringjoin(arg_parser: ArgParser) -> Result<Value> {
    match arg_parser.two_args()? {
        (Value::String(join), Value::Array(target)) => {
            let joined = match target.first() {
                None => Ok(String::new()),
                Some(Value::String(init)) => {
                    target
                        .iter()
                        .skip(1)
                        .try_fold(init.clone(), |acc, elt| match elt {
                            Value::String(s) => Ok(format!("{}{}{}", acc, join, s).to_string()),
                            _ => Err(Error::eval("stringjoin target is not a string".to_string())),
                        })
                }
                _ => Err(Error::eval("stringjoin target is not a string".to_string())),
            }?;

            Ok(Value::String(joined))
        }
        _ => Err(Error::eval(
            "stringjoin args must be a string an array of strings".to_string(),
        )),
    }
}
