use super::Value;

use std::cmp::Ordering;

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::Boolean(true), Value::Boolean(false)) => Some(Ordering::Greater),
            (Value::Boolean(false), Value::Boolean(true)) => Some(Ordering::Less),
            // this covers both equality cases
            (Value::Boolean(_), Value::Boolean(_)) => Some(Ordering::Equal),
            (Value::String(left), Value::String(right)) => left.partial_cmp(right),
            (Value::Int(left), Value::Int(right)) => left.partial_cmp(right),
            (Value::Float(left), Value::Float(right)) => left.partial_cmp(right),
            (Value::Int(left), Value::Float(right)) => (*left as f64).partial_cmp(right),
            (Value::Float(left), Value::Int(right)) => left.partial_cmp(&(*right as f64)),
            (_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use std::cmp::Ordering;
    #[test]
    fn null_equals_null() {
        assert!(Value::Null == Value::Null)
    }

    #[test]
    fn compare_booleans() {
        assert!(Value::Boolean(true) == Value::Boolean(true));
        assert!(Value::Boolean(false) == Value::Boolean(false));
        assert!(Value::Boolean(true) > Value::Boolean(false));
        assert!(Value::Boolean(true) >= Value::Boolean(false));
        assert!(Value::Boolean(false) < Value::Boolean(true));
        assert!(Value::Boolean(false) <= Value::Boolean(true));

        assert_eq!(None, Value::Null.partial_cmp(&Value::Boolean(false)));
    }
    #[test]
    fn compare_strings() {
        assert!(Value::String("a".to_string()) == Value::String("a".to_string()));
        assert!(Value::String("b".to_string()) >= Value::String("a".to_string()));
        assert!(Value::String("b".to_string()) > Value::String("a".to_string()));

        assert!(Value::String("a".to_string()) < Value::String("aa".to_string()));
        assert!(Value::String("a".to_string()) <= Value::String("aa".to_string()));
    }

    #[test]
    fn compare_numbers() {
        assert!(Value::Int(2) > Value::Int(1));
        assert!(Value::Int(2) >= Value::Int(1));
        assert!(Value::Int(2) > Value::Float(1.0));
        assert!(Value::Float(2.0) > Value::Int(1));

        assert_eq!(
            Some(Ordering::Equal),
            Value::Int(2).partial_cmp(&Value::Float(2.0))
        );
        assert_eq!(
            Some(Ordering::Equal),
            Value::Float(2.0).partial_cmp(&Value::Int(2))
        );

        assert_eq!(None, Value::Int(2).partial_cmp(&Value::Null));
        assert_eq!(None, Value::Int(2).partial_cmp(&Value::Boolean(true)));
        assert_eq!(
            None,
            Value::Int(2).partial_cmp(&Value::String("a".to_string()))
        );
    }
}
