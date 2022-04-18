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
