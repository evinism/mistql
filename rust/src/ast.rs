use super::Rule;
use crate::error::{Error, Result};
use pest::iterators::Pair;

pub trait Node {
    fn from_pair(expr: Pair<Rule>) -> Result<Self>
    where
        Self: Sized;
    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value>;
}

pub enum SimpleExpr {
    At,
    Value(Value),
}
pub struct Value(serde_json::Value);

impl Node for SimpleExpr {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        match expr.as_rule() {
            Rule::at => Ok(Self::At),
            Rule::bool | Rule::number | Rule::string | Rule::null => {
                Ok(Self::Value(Value::from_pair(expr)?))
            }
            _ => Err(Error::query(format!(
                "unimplemented rule {:?}",
                expr.as_rule()
            ))),
        }
    }

    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            SimpleExpr::At => Ok(context.clone()),
            SimpleExpr::Value(val) => val.evaluate(context),
        }
    }
}

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
