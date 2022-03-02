use crate::{Node, Result, Rule, SimpleExpr};
use pest::iterators::Pair;

pub struct Array(Vec<SimpleExpr>);

impl Node for Array {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        let elts = expr
            .into_inner()
            .map(|elt| SimpleExpr::from_pair(elt))
            .collect::<Result<Vec<SimpleExpr>>>()?;
        Ok(Array(elts))
    }

    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        self.0.iter().map(|elt| elt.evaluate(context)).collect()
    }
}
