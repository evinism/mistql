use pest::iterators::Pair;

use crate::eval::expr;
use crate::{Result, Rule};

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    Ok(pair
        .into_inner()
        .map(|elt| expr::eval(elt, context))
        .collect::<Result<Vec<serde_json::Value>>>()?
        .into())
}
