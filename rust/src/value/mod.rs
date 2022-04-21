use std::cmp::Ordering;
use std::collections::BTreeMap;

mod display;
mod json;
mod number;

pub use number::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Ident(String),
    Regex(String, Option<String>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(l), Value::Number(r)) => Some(l.cmp(&r)),
            (Value::String(l), Value::String(r)) => Some(l.cmp(&r)),
            (Value::Boolean(l), Value::Boolean(r)) => Some(l.cmp(&r)),
            _ => None,
        }
    }
}
