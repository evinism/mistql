use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn apply(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let (target, func) = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(target), Some(func), None, None) => (target, func),
        (None, Some(func), Some(target), None) => (expr::eval(target, data, None)?, func),
        _ => {
            return Err(Error::eval(
                "map requires one function and one target".to_string(),
            ))
        }
    };
    expr::eval(func.clone(), &target, None)
}

#[cfg(test)]
mod tests {
    // this is pretty extensively used in the integration tests and additional unit
    // tests would add little value
}
