use std::collections::BTreeMap;

mod display;
mod json;
mod ord;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Ident(String),
    Regex(String, Option<String>),
}
