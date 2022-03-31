use pest::iterators::Pair;

use crate::{Result, Rule};

mod array;
mod expr;
mod function;
mod index;
mod infix;
mod object;
mod prefix;
mod terminal;
mod value;

pub use value::Value;

pub fn eval(pair: Pair<Rule>, data: serde_json::Value) -> Result<serde_json::Value> {
    match expr::eval(pair, &data.into(), None) {
        Ok(val) => Ok(val.into()),
        Err(e) => Err(e),
    }
}
