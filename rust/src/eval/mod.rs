use pest::iterators::Pair;

use crate::{Error, Result, Rule};

mod value;

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    match pair.as_rule() {
        Rule::at => Ok(context.clone()),
        Rule::bool | Rule::number | Rule::string | Rule::null => value::eval(pair, &context),
        _ => Err(Error::unimplemented(format!(
            "unimplemented rule {:?}",
            pair.as_rule()
        ))),
    }
}
