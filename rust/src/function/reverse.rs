use super::args::ArgParser;
use crate::{Error, Result, Value};

pub fn reverse(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?;

    match arg {
        Value::Array(arr) => Ok(Value::Array(arr.into_iter().rev().collect())),
        _ => Err(Error::eval(format!(
            "argument to reverse must be an array (got {:?}",
            arg
        ))),
    }
}
