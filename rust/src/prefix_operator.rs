use crate::expr::SimpleExpr;
use crate::{Node, Result, Rule};
use pest::iterators::Pair;

enum PrefixOperator {
    Not,
}

pub struct PrefixedValue {
    operator: PrefixOperator,
    operand: Box<SimpleExpr>,
}

impl Node for PrefixedValue {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        let mut prefix_iter = expr.into_inner();
        let operator = match prefix_iter.next().unwrap().as_rule() {
            Rule::not_op => PrefixOperator::Not,
            _ => unreachable!("unrecognized prefix operator"),
        };
        let operand = prefix_iter.next().unwrap();

        Ok(Self {
            operator: operator,
            operand: Box::new(SimpleExpr::from_pair(operand)?),
        })
    }

    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self.operator {
            PrefixOperator::Not => {
                let val = self.operand.evaluate(context)?;
                let truth = truthiness(val);
                Ok(truth.into())
            }
        }
    }
}

fn truthiness(val: serde_json::Value) -> bool {
    match val {
        serde_json::Value::Null => false,
        serde_json::Value::Bool(bool) => bool,
        serde_json::Value::Number(n) => n.as_f64() != Some(0.0),
        serde_json::Value::String(s) => s.len() != 0,
        serde_json::Value::Array(arr) => arr.len() != 0,
        serde_json::Value::Object(obj) => obj.len() != 0,
    }
}
