use crate::value::Value;
use crate::{Error, Node, Result, Rule};
use pest::iterators::Pair;

pub enum SimpleExpr {
    At,
    Value(Value),
    Array(Vec<SimpleExpr>),
}

impl Node for SimpleExpr {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        match expr.as_rule() {
            Rule::at => Ok(Self::At),
            Rule::bool | Rule::number | Rule::string | Rule::null => {
                Ok(Self::Value(Value::from_pair(expr)?))
            }
            Rule::array => {
                let elts = expr
                    .into_inner()
                    .map(|elt| SimpleExpr::from_pair(elt))
                    .collect::<Result<Vec<Self>>>()?;
                Ok(Self::Array(elts))
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
            SimpleExpr::Array(arr) => arr.iter().map(|elt| elt.evaluate(context)).collect(),
        }
    }
}
