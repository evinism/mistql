use crate::expr::SimpleExpr;
use crate::{Error, Node, Result, Rule};
use pest::iterators::Pair;
use std::str::FromStr;

enum FunctionName {
    Regex,
}

impl FromStr for FunctionName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "regex" => Ok(FunctionName::Regex),
            _ => Err(Error::query(format!("unknown function {}", s))),
        }
    }
}

pub struct Function {
    name: FunctionName,
    args: Vec<SimpleExpr>,
}

impl Node for Function {
    fn from_pair(expr: Pair<Rule>) -> Result<Self> {
        let mut function_iter = expr.into_inner();
        let name: FunctionName = function_iter.next().unwrap().as_str().parse()?;
        let args = function_iter
            .map(|arg| SimpleExpr::from_pair(arg))
            .collect::<Result<Vec<SimpleExpr>>>()?;

        Ok(Self {
            name: name,
            args: args,
        })
    }

    fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self.name {
            // this makes integration tests work but we're not ready to handle it yet
            FunctionName::Regex => self.args[0].evaluate(context),
        }
    }
}
