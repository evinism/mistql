use super::args::ArgParser;
use crate::prefix::truthiness;
use crate::{expr, Error, Result, Value};

pub fn find(arg_parser: ArgParser) -> Result<Value> {
    let args = arg_parser.one_func_one_arg()?;
    match args {
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
