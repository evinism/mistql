use super::args::ArgParser;
use crate::prefix::truthiness;
use crate::{Result, Value};

pub fn if_fn(arg_parser: ArgParser) -> Result<Value> {
    let (predicate, iftrue, iffalse) = arg_parser.three_args()?;

    match truthiness(&predicate) {
        true => Ok(iftrue),
        false => Ok(iffalse),
    }
}

#[cfg(test)]
mod tests {
    // TODO do we need unit tests on this? There are a few integration tests
}
