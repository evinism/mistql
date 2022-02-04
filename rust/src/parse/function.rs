use crate::Error;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Count,
    Regex,
    Sum,
}

impl FromStr for Function {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "count" => Ok(Function::Count),
            "regex" => Ok(Function::Regex),
            "sum" => Ok(Function::Sum),
            _ => Err(Error::query(format!("unknown function {}", s))),
        }
    }
}
