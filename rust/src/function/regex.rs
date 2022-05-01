use super::args::ArgParser;
use crate::{Error, Result, Value};
use regex::Regex;

pub fn regex(arg_parser: ArgParser) -> Result<Value> {
    let (expr, flags) = match (arg_parser.one_arg(), arg_parser.two_args()) {
        (Ok(expr_val), _) => (expr_val, None),
        (_, Ok((expr_val, flags_val))) => (expr_val, Some(flags_val.to_value(arg_parser.data)?)),
        (Err(_), Err(_)) => {
            return Err(Error::eval(
                "regex requires one or two arguments".to_string(),
            ))
        }
    };

    match (expr.to_value(arg_parser.data)?, flags) {
        (Value::String(exp), Some(Value::String(flags))) => Ok(Value::Regex(exp, Some(flags))),
        (Value::String(exp), None) => Ok(Value::Regex(exp, None)),
        _ => Err(Error::eval(
            "regex expression and flags must be strings".to_string(),
        )),
    }
}

pub fn match_fn(arg_parser: ArgParser) -> Result<Value> {
    let (pattern_val, target_val) = arg_parser.two_args()?;

    match_op(
        target_val.to_value(arg_parser.data)?,
        pattern_val.to_value(arg_parser.data)?,
    )
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

pub fn split(arg_parser: ArgParser) -> Result<Value> {
    let (pattern_val, target_val) = arg_parser.two_args()?;

    let pattern = match pattern_val.to_value(arg_parser.data)? {
        Value::Regex(s, _) | Value::String(s) => match Regex::new(&s) {
            Ok(pat) => Ok(pat),
            Err(err) => Err(Error::regex(err)),
        },
        _ => Err(Error::eval(
            "split pattern must be a regex or a string".to_string(),
        )),
    }?;

    let target = match target_val.to_value(arg_parser.data)? {
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

pub fn replace(arg_parser: ArgParser) -> Result<Value> {
    let (pattern_val, replacement_val, target_val) = arg_parser.three_args()?;

    let (pattern, flags) = match pattern_val.to_value(arg_parser.data)? {
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

    let target = match target_val.to_value(arg_parser.data)? {
        Value::String(s) => Ok(s),
        _ => Err(Error::eval("split target must be a string".to_string())),
    }?;

    let replacement = match replacement_val.to_value(arg_parser.data)? {
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
