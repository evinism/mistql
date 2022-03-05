use pest::iterators::Pair;

use crate::eval::expr;
use crate::{Result, Rule};

pub fn eval(pair: Pair<Rule>, context: &serde_json::Value) -> Result<serde_json::Value> {
    let elts = pair
        .into_inner()
        .map(|elt| {
            let mut keyval_iter = elt.into_inner();
            let key = keyval_iter.next().unwrap();
            let val = keyval_iter.next().unwrap();

            let key_str = match key.as_rule() {
                Rule::ident => key.as_str(),
                Rule::string => key.into_inner().next().unwrap().as_str(),
                _ => unreachable!("unrecognized string as object key {:?}", key),
            };

            Ok((key_str.into(), expr::eval(val, context)?))
        })
        .collect::<Result<serde_json::Map<String, serde_json::Value>>>()?;
    Ok(elts.into())
}
