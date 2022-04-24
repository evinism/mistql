use crate::{expr, Error, Result, Rule, Value};
use pest::iterators::Pairs;
use regex::Regex;

pub fn regex(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let (exp_val, flags_val) = match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
        (Some(ctx), None, None, None) => (ctx, None),
        // this is a weird case - flags probably shouldn't come from a pipe
        (Some(ctx), Some(val), None, None) => (expr::eval(val, data, None)?, Some(ctx)),
        (None, Some(val), None, None) => (expr::eval(val, data, None)?, None),
        (None, Some(arg1), Some(arg2), None) => (
            expr::eval(arg1, data, None)?,
            Some(expr::eval(arg2, data, None)?),
        ),
        _ => {
            return Err(Error::eval(
                "regex requires one or two arguments".to_string(),
            ))
        }
    };

    match (exp_val, flags_val) {
        (Value::String(exp), Some(Value::String(flags))) => Ok(Value::Regex(exp, Some(flags))),
        (Value::String(exp), None) => Ok(Value::Regex(exp, None)),
        _ => Err(Error::eval(
            "regex expression and flags must be strings".to_string(),
        )),
    }
}

pub fn match_fn(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let (pattern_val, target_val) =
        match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
            (Some(target), Some(pattern), None, None) => (expr::eval(pattern, data, None)?, target),
            (None, Some(pattern), Some(target), None) => (
                expr::eval(pattern, data, None)?,
                expr::eval(target, data, None)?,
            ),
            _ => {
                return Err(Error::eval(
                    "match requires a regex and a target".to_string(),
                ))
            }
        };

    match_op(target_val, pattern_val)
}

pub fn match_op(left: Value, right: Value) -> Result<Value> {
    let pattern = match right {
        Value::String(s) | Value::Regex(s, None) => match Regex::new(&s) {
            Ok(pat) => Ok(pat),
            Err(err) => Err(Error::regex(err)),
        },
        Value::Regex(pat, Some(flags)) => match Regex::new(&format!("(?{}){}", flags, pat)) {
            Ok(pat) => Ok(pat),
            Err(err) => Err(Error::regex(err)),
        },
        _ => Err(Error::eval(
            "match pattern must be a regex or a string".to_string(),
        )),
    }?;

    let target = match left {
        Value::String(s) => Ok(s),
        _ => Err(Error::eval("match target must be a string".to_string())),
    }?;

    Ok(Value::Boolean(pattern.is_match(&target)))
}

pub fn split(mut arg_itr: Pairs<Rule>, data: &Value, context_opt: Option<Value>) -> Result<Value> {
    let (pattern_val, target_val) =
        match (context_opt, arg_itr.next(), arg_itr.next(), arg_itr.next()) {
            (Some(target), Some(pattern), None, None) => (expr::eval(pattern, data, None)?, target),
            (None, Some(pattern), Some(target), None) => (
                expr::eval(pattern, data, None)?,
                expr::eval(target, data, None)?,
            ),
            _ => {
                return Err(Error::eval(
                    "split requires a regex and a target".to_string(),
                ))
            }
        };

    let pattern = match pattern_val {
        Value::Regex(s, _) | Value::String(s) => match Regex::new(&s) {
            Ok(pat) => Ok(pat),
            Err(err) => Err(Error::regex(err)),
        },
        _ => Err(Error::eval(
            "split pattern must be a regex or a string".to_string(),
        )),
    }?;

    let target = match target_val {
        Value::String(s) => Ok(s),
        _ => Err(Error::eval("split target must be a string".to_string())),
    }?;

    Ok(Value::Array(
        pattern
            .split(&target)
            .map(|elt| Value::String(elt.to_string()))
            .collect(),
    ))
}

pub fn replace(
    mut arg_itr: Pairs<Rule>,
    data: &Value,
    context_opt: Option<Value>,
) -> Result<Value> {
    let (pattern_val, target_val, replacement_val) = match (
        context_opt,
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
        arg_itr.next(),
    ) {
        (Some(target), Some(pattern), Some(replacement), None, None) => (
            expr::eval(pattern, data, None)?,
            target,
            expr::eval(replacement, data, None)?,
        ),
        (None, Some(pattern), Some(target), Some(replacement), None) => (
            expr::eval(pattern, data, None)?,
            expr::eval(target, data, None)?,
            expr::eval(replacement, data, None)?,
        ),
        _ => {
            return Err(Error::eval(
                "replace requires a regex and a target".to_string(),
            ))
        }
    };

    let (pattern, flags) = match pattern_val {
        Value::Regex(s, None) | Value::String(s) => match Regex::new(&s) {
            Ok(pat) => Ok((pat, "".to_string())),
            Err(err) => Err(Error::regex(err)),
        },
        Value::Regex(s, Some(f)) => match Regex::new(&s) {
            Ok(pat) => Ok((pat, f)),
            Err(err) => Err(Error::regex(err)),
        },
        _ => Err(Error::eval(
            "split pattern must be a regex or a string".to_string(),
        )),
    }?;

    let target = match target_val {
        Value::String(s) => Ok(s),
        _ => Err(Error::eval("split target must be a string".to_string())),
    }?;

    let replacement = match replacement_val {
        Value::String(s) => Ok(s),
        _ => Err(Error::eval(
            "replace replacement must be a string".to_string(),
        )),
    }?;

    if flags.contains("g") {
        Ok(Value::String(
            pattern.replace_all(&target, replacement).to_string(),
        ))
    } else {
        Ok(Value::String(
            pattern.replace(&target, replacement).to_string(),
        ))
    }
}
