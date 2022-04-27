use crate::{expr, Result, Value};
use super::args::ArgParser;

pub fn apply(arg_parser: ArgParser) -> Result<Value> {
    let (func, target) = arg_parser.one_func_one_arg()?;
    expr::eval(func.clone(), &target, None)
}

#[cfg(test)]
mod tests {
    // this is pretty extensively used in the integration tests and additional unit
    // tests would add little value
}
