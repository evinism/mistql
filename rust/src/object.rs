use crate::{Node, Result, Rule, SimpleExpr};
use pest::iterators::Pair;
use std::collections::HashMap;

pub struct Object(HashMap<String, SimpleExpr>);

impl Node for Object {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        let elts = expr
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

                Ok((key_str.into(), SimpleExpr::from_pair(val)?))
            })
            .collect::<Result<HashMap<String, SimpleExpr>>>()?;
        Ok(Object(elts))
    }

    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        let keyvals = self
            .0
            .iter()
            .map(|(key, val)| match val.evaluate(context) {
                Ok(v) => Ok((key.clone(), v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<(String, serde_json::Value)>>>()?;
        Ok(serde_json::Map::from_iter(keyvals).into())
    }
}
