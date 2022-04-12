use crate::prefix::truthiness;
use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn if_fn(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let (predicate, iftrue, iffalse) = match (
        context_opt,
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
    ) {
        (Some(iffalse), Some(predicate), Some(iftrue), None, None) => (
            expr::eval(predicate, data, None)?,
            expr::eval(iftrue, data, None)?,
            iffalse,
        ),
        (None, Some(predicate), Some(iftrue), Some(iffalse), None) => (
            expr::eval(predicate, data, None)?,
            expr::eval(iftrue, data, None)?,
            expr::eval(iffalse, data, None)?,
        ),
        _ => return Err(Error::eval("if requires three arguments".to_string())),
    };

    match truthiness(&predicate) {
        true => Ok(iftrue),
        false => Ok(iffalse),
    }
}

#[cfg(test)]
mod tests {
    // TODO do we need unit tests on this? There are a few integration tests
}
