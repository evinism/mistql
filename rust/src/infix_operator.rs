use crate::{Error, Node, Result, Rule, SimpleExpr};
use pest::iterators::Pair;

enum InfixOperator {
    Equals,
}

pub struct InfixExpression {
    operator: InfixOperator,
    lhs: Box<SimpleExpr>,
    rhs: Box<SimpleExpr>,
}

impl Node for InfixExpression {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        let mut infix_iter = expr.into_inner();
        let lhs = infix_iter.next().unwrap();
        let operator = match infix_iter.next().unwrap().as_rule() {
            Rule::eq_op => Ok(InfixOperator::Equals),
            _ => Err(Error::query("unimplemented infix operator".to_string())),
        }?;
        let rhs = infix_iter.next().unwrap();

        Ok(Self {
            operator: operator,
            lhs: Box::new(SimpleExpr::from_pair(lhs)?),
            rhs: Box::new(SimpleExpr::from_pair(rhs)?),
        })
    }

    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        let lhs = self.lhs.evaluate(context)?;
        let rhs = self.rhs.evaluate(context)?;
        match self.operator {
            InfixOperator::Equals => Ok((lhs == rhs).into()),
        }
    }
}
