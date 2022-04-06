use crate::index::{item_index, range_index};
use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;

pub fn index(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    match (
        context_opt,
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
    ) {
        (Some(target), Some(idx), None, None, None) => {
            item_index(&expr::eval(idx, data, None)?, &target)
        }
        (Some(target), Some(low), Some(high), None, None) => range_index(
            &expr::eval(low, data, None)?,
            &expr::eval(high, data, None)?,
            &target,
        ),
        (None, Some(idx), Some(target), None, None) => item_index(
            &expr::eval(idx, data, None)?,
            &expr::eval(target, data, None)?,
        ),
        (None, Some(low), Some(high), Some(target), None) => range_index(
            &expr::eval(low, data, None)?,
            &expr::eval(high, data, None)?,
            &expr::eval(target, data, None)?,
        ),
        _ => return Err(Error::eval("count requires one argument".to_string())),
    }
}
