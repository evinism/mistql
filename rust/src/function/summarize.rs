use crate::infix::arithmetic::sqrt;
use crate::{expr, Error, Number, Result, Rule, Value};
use pest::iterators::Pairs;
use std::collections::BTreeMap;

pub fn summarize(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let arg = match (context_opt, arg_itr.next(), arg_itr.next()) {
        (Some(val), None, None) => val,
        (None, Some(val), None) => expr::eval(val, data, None)?,
        _ => return Err(Error::eval("summarize requires one argument".to_string())),
    };

    match arg {
        Value::Array(arr) if arr.len() > 0 => {
            let mut nums = arr
                .into_iter()
                .map(|elt| match elt {
                    Value::Number(n) => Ok(n),
                    _ => Err(Error::eval(
                        "argument to summarize must be an array of Numbers".to_string(),
                    )),
                })
                .collect::<Result<Vec<Number>>>()?;
            nums.sort_unstable();
            let mean = mean(&nums);
            let variance = variance(&nums, &mean);

            let mut result = BTreeMap::new();
            result.insert("max".to_string(), max(&nums));
            result.insert("min".to_string(), min(&nums));
            result.insert("mean".to_string(), Value::Number(mean));
            result.insert("median".to_string(), median(&nums));
            result.insert("variance".to_string(), variance.clone());
            result.insert("stddev".to_string(), sqrt(variance)?);
            Ok(Value::Object(result))
        }
        _ => Err(Error::eval(format!(
            "argument to summarize must be an non-empty array (got {:?}",
            arg
        ))),
    }
}

fn max(arr: &Vec<Number>) -> Value {
    // safe unwrap - we know the vec has at least one element
    Value::Number(*arr.last().unwrap())
}

fn min(arr: &Vec<Number>) -> Value {
    // safe unwrap - we know the vec has at least one element
    Value::Number(*arr.first().unwrap())
}

fn mean(arr: &Vec<Number>) -> Number {
    let sum = arr.iter().fold(Number::Int(0), |acc, x| acc + *x);
    sum / Number::Int(arr.len() as i64)
}

fn median(arr: &Vec<Number>) -> Value {
    let res = match arr.len() {
        even if even % 2 == 0 => {
            let low = arr[even / 2];
            let high = arr[(even / 2) + 1];
            (low + high) / Number::Int(2)
        }
        odd => arr[odd / 2],
    };
    Value::Number(res)
}

fn variance(arr: &Vec<Number>, mean: &Number) -> Value {
    let sum_of_diffs = arr
        .iter()
        .map(|val| {
            let diff = *mean - *val;
            diff * diff
        })
        .fold(Number::Float(0.0), |acc, x| acc + x);
    Value::Number(sum_of_diffs / Number::Int(arr.len() as i64 - 1))
}
