use crate::{Error, Node, Result, Rule};
use pest::iterators::Pair;

pub struct Value(serde_json::Value);

impl Node for Value {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        match serde_json::from_str(expr.as_str()) {
            Ok(val) => Ok(Self(val)),
            Err(_) => Err(Error::query(format!("unparseable value {:?}", expr))),
        }
    }

    fn evaluate(&self, _context: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.0.clone().into())
    }
}
