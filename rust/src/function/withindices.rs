use super::args::ArgParser;
use crate::{Error, Number, Result, Value};

pub fn withindices(arg_parser: ArgParser) -> Result<Value> {
    let arg = arg_parser.one_arg()?.to_value(arg_parser.data)?;

    match arg {
        Value::Array(arr) => Ok(Value::Array(
            arr.iter()
                .enumerate()
                .map(|(i, elt)| {
                    Value::Array(vec![Value::Number(Number::Int(i as i64)), elt.clone()])
                })
                .collect(),
        )),
        _ => Err(Error::eval(format!(
            "argument to withindices must be an array (got {:?}",
            arg
        ))),
    }
}
