use super::args::ArgParser;
use crate::{expr, Error, Result, Value};

pub fn reduce(arg_parser: ArgParser) -> Result<Value> {
    let (func_arg, init_arg, target_arg) = arg_parser.three_args()?;
    match (
        func_arg.to_pair()?,
        init_arg.to_value(arg_parser.data)?,
        target_arg.to_value(arg_parser.data)?,
    ) {
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
