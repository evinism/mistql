use crate::expr::SimpleExpr;
use crate::{Error, Node, Result, Rule};
use pest::iterators::Pair;
use std::str::FromStr;

enum FunctionName {
    Count,
    Log,
}

impl FromStr for FunctionName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "count" => Ok(FunctionName::Count),
            "log" => Ok(FunctionName::Log),
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
            FunctionName::Count => self.count(context),
            FunctionName::Log => self.log(context),
        }
    }
}

impl Function {
    fn count(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self.args[0].evaluate(context)? {
            serde_json::Value::Array(arr) => Ok(arr.len().into()),
            _ => Err(Error::eval(
                "argument to count must be an array".to_string(),
            )),
        }
    }

    fn log(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        let arg = self.args[0].evaluate(context)?;
        dbg!(arg.clone());
        Ok(arg)
    }
}
