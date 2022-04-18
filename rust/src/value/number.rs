use crate::{Error, Result};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl TryFrom<&str> for Number {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match (value.parse::<i64>(), value.parse::<f64>()) {
            (Ok(i), _) => Ok(Self::Int(i)),
            (_, Ok(f)) => Ok(Self::Float(f)),
            (Err(_), Err(err)) => Err(Error::query(format!("unparseable number: {:?}", err))),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => l == r,
            (Number::Float(l), Number::Float(r)) => l == r,
            (Number::Int(l), Number::Float(r)) => (*l as f64) == *r,
            (Number::Float(l), Number::Int(r)) => *l == (*r as f64),
        }
    }
}
impl Eq for Number {}

#[cfg(test)]
mod tests {
    use super::Number;

    #[test]
    fn parses_from_string() {
        assert_eq!(Number::Int(4), Number::try_from("4").unwrap());
        assert_eq!(Number::Float(4.0), Number::try_from("4.0").unwrap());
        assert!(Number::try_from("abc").is_err())
    }
}
