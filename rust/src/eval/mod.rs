use pest::iterators::Pair;

use crate::{Result, Rule};

mod array;
mod expr;
mod value;

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    expr::eval(pair, context)
}
