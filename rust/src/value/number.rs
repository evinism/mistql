use crate::{Error, Result};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

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

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => l.partial_cmp(&r),
            (Number::Float(l), Number::Float(r)) => l.partial_cmp(&r),
            (Number::Int(l), Number::Float(r)) => (*l as f64).partial_cmp(&r),
            (Number::Float(l), Number::Int(r)) => l.partial_cmp(&(*r as f64)),
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => l.cmp(&r),
            (Number::Float(l), Number::Float(r)) if l.is_finite() && r.is_finite() => {
                l.partial_cmp(&r).unwrap()
            }
            (Number::Int(l), Number::Float(r)) if r.is_finite() => {
                (*l as f64).partial_cmp(&r).unwrap()
            }
            (Number::Float(l), Number::Int(r)) if l.is_finite() => {
                l.partial_cmp(&(*r as f64)).unwrap()
            }
            _ => unreachable!(), // need to forbid NaN from existing elsewhere
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l + r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 + r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l + r as f64)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l + r)),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l - r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 - r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l - r as f64)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l - r)),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l * r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 * r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l * r as f64)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l * r)),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(l), Number::Int(r)) => (Number::Int(l / r)),
            (Number::Int(l), Number::Float(r)) => (Number::Float(l as f64 / r)),
            (Number::Float(l), Number::Int(r)) => (Number::Float(l / r as f64)),
            (Number::Float(l), Number::Float(r)) => (Number::Float(l / r)),
        }
    }
}

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
