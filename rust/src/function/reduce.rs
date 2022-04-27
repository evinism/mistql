use super::args::ArgParser;
use crate::{expr, Error, Result, Value};

pub fn reduce(arg_parser: ArgParser) -> Result<Value> {
    let args = arg_parser.one_func_two_args()?;
    match args {
        (func, init, Value::Array(val)) => {
            let itr = val.into_iter();
            let mut result = init.clone();
            for elt in itr {
                let func_param = Value::Array(vec![result.clone(), elt]);
                result = expr::eval(func.clone(), &func_param, None)?;
            }

            Ok(result)
        }
        (val, _, _) => Err(Error::eval(format!(
            "argument to reduce must be an array (got {:?}",
            val
        ))),
    }
}
