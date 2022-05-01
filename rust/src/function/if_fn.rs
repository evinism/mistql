use super::args::ArgParser;
use crate::prefix::truthiness;
use crate::{Result, Value};

pub fn if_fn(arg_parser: ArgParser) -> Result<Value> {
    let (predicate_arg, iftrue_arg, iffalse_arg) = arg_parser.three_args()?;
    let (predicate, iftrue, iffalse) = (
        predicate_arg.to_value(arg_parser.data)?,
        iftrue_arg.to_value(arg_parser.data)?,
        iffalse_arg.to_value(arg_parser.data)?,
    );

    match truthiness(&predicate) {
        true => Ok(iftrue),
        false => Ok(iffalse),
    }
}

pub fn if_fn_ident(arg_parser: ArgParser) -> Result<Value> {
    let (predicate_arg, iftrue_arg, iffalse_arg) = arg_parser.three_args()?;
    let (predicate, iftrue, iffalse) = (
        predicate_arg.to_value(arg_parser.data)?,
        iftrue_arg.to_ident()?,
        iffalse_arg.to_ident()?,
    );

    match truthiness(&predicate) {
        true => Ok(iftrue),
        false => Ok(iffalse),
    }
}

#[cfg(test)]
mod tests {
    // TODO do we need unit tests on this? There are a few integration tests
}
