use super::args::ArgParser;
use crate::{expr, Result, Value};

pub fn apply(arg_parser: ArgParser) -> Result<Value> {
    let (func, target) = arg_parser.two_args()?;
    expr::eval(func.to_pair()?, &target.to_value(arg_parser.data)?, None)
}

#[cfg(test)]
mod tests {
    // this is pretty extensively used in the integration tests and additional unit
    // tests would add little value
}
