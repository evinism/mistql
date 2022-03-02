use crate::array::Array;
use crate::object::Object;
use crate::operator::PrefixedValue;
use crate::value::Value;
use crate::{Error, Node, Result, Rule};
use pest::iterators::Pair;

pub enum SimpleExpr {
    At,
    Value(Value),
    Array(Array),
    Object(Object),
    PrefixedValue(PrefixedValue),
}

impl Node for SimpleExpr {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        match expr.as_rule() {
            Rule::at => Ok(Self::At),
            Rule::bool | Rule::number | Rule::string | Rule::null => {
                Ok(Self::Value(Value::from_pair(expr)?))
            }
            Rule::array => Ok(Self::Array(Array::from_pair(expr)?)),
            Rule::object => Ok(Self::Object(Object::from_pair(expr)?)),
            Rule::prefixed_value => Ok(Self::PrefixedValue(PrefixedValue::from_pair(expr)?)),
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
            SimpleExpr::Array(arr) => arr.evaluate(context),
            SimpleExpr::Object(obj) => obj.evaluate(context),
            SimpleExpr::PrefixedValue(val) => val.evaluate(context),
        }
    }
}
