use super::args::ArgParser;
use crate::prefix::truthiness;
use crate::{expr, Error, Result, Value};

pub fn find(arg_parser: ArgParser) -> Result<Value> {
    let (func_arg, target_arg) = arg_parser.two_args()?;
    match (func_arg.to_pair()?, target_arg.to_value(arg_parser.data)?) {
        (func, Value::Array(val)) => {
            for elt in val.into_iter() {
                let predicate = expr::eval(func.clone(), &elt, None)?;
                if truthiness(&predicate) {
                    return Ok(elt);
                }
            }
            return Ok(Value::Null);
        }
        (val, _) => Err(Error::eval(format!(
            "argument to find must be an array (got {:?}",
            val
        ))),
    }
}
