//! Runtime value types for MistQL
//!
//! This module implements the 8 core MistQL types with proper type safety,
//! conversion, equality, comparison, and truthiness operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use regex::Regex;

/// Custom regex wrapper that implements Serialize/Deserialize
#[derive(Debug, Clone)]
pub struct MistQLRegex {
    pattern: String,
    flags: String,
    compiled: Regex,
}

impl PartialEq for MistQLRegex {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern && self.flags == other.flags
    }
}

impl Eq for MistQLRegex {}

impl MistQLRegex {
    pub fn new(pattern: &str, flags: &str) -> Result<Self, regex::Error> {
        let compiled = Regex::new(pattern)?;
        Ok(Self {
            pattern: pattern.to_string(),
            flags: flags.to_string(),
            compiled,
        })
    }

    pub fn as_regex(&self) -> &Regex {
        &self.compiled
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn flags(&self) -> &str {
        &self.flags
    }
}

impl Serialize for MistQLRegex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MistQLRegex", 2)?;
        state.serialize_field("pattern", &self.pattern)?;
        state.serialize_field("flags", &self.flags)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for MistQLRegex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct MistQLRegexVisitor;

        impl<'de> Visitor<'de> for MistQLRegexVisitor {
            type Value = MistQLRegex;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a regex object with pattern and flags")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MistQLRegex, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut pattern: Option<String> = None;
                let mut flags: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "pattern" => {
                            if pattern.is_some() {
                                return Err(de::Error::duplicate_field("pattern"));
                            }
                            pattern = Some(map.next_value()?);
                        }
                        "flags" => {
                            if flags.is_some() {
                                return Err(de::Error::duplicate_field("flags"));
                            }
                            flags = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(de::Error::unknown_field(key, &["pattern", "flags"]));
                        }
                    }
                }
                let pattern = pattern.ok_or_else(|| de::Error::missing_field("pattern"))?;
                let flags = flags.unwrap_or_default();
                MistQLRegex::new(&pattern, &flags).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_map(MistQLRegexVisitor)
    }
}

/// The 8 core MistQL types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuntimeValueType {
    Null,
    Boolean,
    Number,
    String,
    Object,
    Array,
    Function,
    Regex,
}

impl fmt::Display for RuntimeValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValueType::Null => write!(f, "null"),
            RuntimeValueType::Boolean => write!(f, "boolean"),
            RuntimeValueType::Number => write!(f, "number"),
            RuntimeValueType::String => write!(f, "string"),
            RuntimeValueType::Object => write!(f, "object"),
            RuntimeValueType::Array => write!(f, "array"),
            RuntimeValueType::Function => write!(f, "function"),
            RuntimeValueType::Regex => write!(f, "regex"),
        }
    }
}

/// MistQL runtime value representing all possible data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HashMap<String, RuntimeValue>),
    Array(Vec<RuntimeValue>),
    Function(String), // TODO: Implement proper function type
    Regex(MistQLRegex),
}

impl RuntimeValue {
    /// Get the type of this runtime value
    pub fn get_type(&self) -> RuntimeValueType {
        match self {
            RuntimeValue::Null => RuntimeValueType::Null,
            RuntimeValue::Boolean(_) => RuntimeValueType::Boolean,
            RuntimeValue::Number(_) => RuntimeValueType::Number,
            RuntimeValue::String(_) => RuntimeValueType::String,
            RuntimeValue::Object(_) => RuntimeValueType::Object,
            RuntimeValue::Array(_) => RuntimeValueType::Array,
            RuntimeValue::Function(_) => RuntimeValueType::Function,
            RuntimeValue::Regex(_) => RuntimeValueType::Regex,
        }
    }

    /// Check if this value is truthy according to MistQL rules
    pub fn truthy(&self) -> bool {
        match self {
            RuntimeValue::Null => false,
            RuntimeValue::Boolean(b) => *b,
            RuntimeValue::Number(n) => *n != 0.0,
            RuntimeValue::String(s) => !s.is_empty(),
            RuntimeValue::Object(o) => !o.is_empty(),
            RuntimeValue::Array(a) => !a.is_empty(),
            RuntimeValue::Function(_) => true,
            RuntimeValue::Regex(_) => true,
        }
    }

    /// Check if this value is comparable (can be used in <, >, <=, >= operations)
    pub fn comparable(&self) -> bool {
        matches!(self, RuntimeValue::Boolean(_) | RuntimeValue::Number(_) | RuntimeValue::String(_))
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuntimeValue::Null, RuntimeValue::Null) => true,
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => a == b,
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => a == b,
            (RuntimeValue::String(a), RuntimeValue::String(b)) => a == b,
            (RuntimeValue::Object(a), RuntimeValue::Object(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for (key, value) in a {
                    if let Some(other_value) = b.get(key) {
                        if value != other_value {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            }
            (RuntimeValue::Array(a), RuntimeValue::Array(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(a, b)| a == b)
            }
            (RuntimeValue::Function(a), RuntimeValue::Function(b)) => a == b, // Referential equality
            (RuntimeValue::Regex(a), RuntimeValue::Regex(b)) => {
                a.pattern() == b.pattern() && a.flags() == b.flags()
            }
            _ => false,
        }
    }
}

impl Eq for RuntimeValue {}

impl RuntimeValue {
    /// Convert from serde_json::Value to RuntimeValue
    pub fn from_serde_value(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => RuntimeValue::Null,
            serde_json::Value::Bool(b) => RuntimeValue::Boolean(*b),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    RuntimeValue::Number(f)
                } else {
                    RuntimeValue::Number(0.0) // Fallback for very large numbers
                }
            }
            serde_json::Value::String(s) => RuntimeValue::String(s.clone()),
            serde_json::Value::Array(arr) => {
                RuntimeValue::Array(arr.iter().map(Self::from_serde_value).collect())
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (key, value) in obj {
                    map.insert(key.clone(), Self::from_serde_value(value));
                }
                RuntimeValue::Object(map)
            }
        }
    }

    /// Convert to serde_json::Value
    pub fn to_serde_value(&self) -> serde_json::Value {
        match self {
            RuntimeValue::Null => serde_json::Value::Null,
            RuntimeValue::Boolean(b) => serde_json::Value::Bool(*b),
            RuntimeValue::Number(n) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0)))
            }
            RuntimeValue::String(s) => serde_json::Value::String(s.clone()),
            RuntimeValue::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| v.to_serde_value()).collect())
            }
            RuntimeValue::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, value) in obj {
                    map.insert(key.clone(), value.to_serde_value());
                }
                serde_json::Value::Object(map)
            }
            RuntimeValue::Function(_) => serde_json::Value::String("[function]".to_string()),
            RuntimeValue::Regex(_) => serde_json::Value::String("[regex]".to_string()),
        }
    }

    /// Compare two values for ordering (<, >, <=, >=)
    pub fn compare(&self, other: &Self) -> Result<std::cmp::Ordering, String> {
        if self.get_type() != other.get_type() {
            return Err("Cannot compare MistQL values of different types".to_string());
        }

        if !self.comparable() {
            return Err(format!("Cannot compare MistQL values of type {}", self.get_type()));
        }

        match (self, other) {
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(a.cmp(b)),
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                a.partial_cmp(b).ok_or_else(|| "Invalid number comparison".to_string())
            }
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(a.cmp(b)),
            _ => Err(format!("Cannot compare MistQL values of type {}", self.get_type())),
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            RuntimeValue::String(s) => s.clone(),
            RuntimeValue::Number(n) => {
                // Simple number formatting - TODO: implement proper MistQL number formatting
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            _ => serde_json::to_string(&self.to_serde_value()).unwrap_or_else(|_| "null".to_string()),
        }
    }

    /// Convert to float
    pub fn to_float(&self) -> Result<f64, String> {
        match self {
            RuntimeValue::Number(n) => Ok(*n),
            RuntimeValue::String(s) => {
                s.parse::<f64>().map_err(|_| format!("Cannot convert string to float: {}", s))
            }
            RuntimeValue::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            RuntimeValue::Null => Ok(0.0),
            _ => Err(format!("Cannot convert {} to float", self.get_type())),
        }
    }

    /// Access object property
    pub fn access(&self, key: &str) -> RuntimeValue {
        match self {
            RuntimeValue::Object(obj) => {
                obj.get(key).cloned().unwrap_or(RuntimeValue::Null)
            }
            _ => RuntimeValue::Null,
        }
    }

    /// Get object keys
    pub fn keys(&self) -> Vec<String> {
        match self {
            RuntimeValue::Object(obj) => obj.keys().cloned().collect(),
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_runtime_value_creation() {
        // Test null
        let null_val = RuntimeValue::Null;
        assert_eq!(null_val.get_type(), RuntimeValueType::Null);
        assert!(!null_val.truthy());

        // Test boolean
        let bool_val = RuntimeValue::Boolean(true);
        assert_eq!(bool_val.get_type(), RuntimeValueType::Boolean);
        assert!(bool_val.truthy());

        // Test number
        let num_val = RuntimeValue::Number(42.0);
        assert_eq!(num_val.get_type(), RuntimeValueType::Number);
        assert!(num_val.truthy());

        // Test string
        let str_val = RuntimeValue::String("hello".to_string());
        assert_eq!(str_val.get_type(), RuntimeValueType::String);
        assert!(str_val.truthy());

        // Test empty string
        let empty_str = RuntimeValue::String("".to_string());
        assert!(!empty_str.truthy());
    }

    #[test]
    fn test_serde_conversion() {
        let json_val = json!({
            "name": "John",
            "age": 30,
            "active": true,
            "scores": [1, 2, 3]
        });

        let runtime_val = RuntimeValue::from_serde_value(&json_val);
        assert_eq!(runtime_val.get_type(), RuntimeValueType::Object);

        let back_to_json = runtime_val.to_serde_value();

        // Test that the conversion preserves the structure and values
        assert_eq!(back_to_json["name"], json_val["name"]);
        assert_eq!(back_to_json["active"], json_val["active"]);

        // For numbers, we need to handle the precision difference
        if let (Some(back_age), Some(orig_age)) = (back_to_json["age"].as_f64(), json_val["age"].as_f64()) {
            assert_eq!(back_age, orig_age);
        } else {
            panic!("Age conversion failed");
        }

        // Test array conversion with number precision handling
        if let (Some(back_scores), Some(orig_scores)) = (back_to_json["scores"].as_array(), json_val["scores"].as_array()) {
            assert_eq!(back_scores.len(), orig_scores.len());
            for (back_score, orig_score) in back_scores.iter().zip(orig_scores.iter()) {
                if let (Some(back_num), Some(orig_num)) = (back_score.as_f64(), orig_score.as_f64()) {
                    assert_eq!(back_num, orig_num);
                } else {
                    panic!("Score conversion failed");
                }
            }
        } else {
            panic!("Scores array conversion failed");
        }
    }

    #[test]
    fn test_comparison() {
        let a = RuntimeValue::Number(10.0);
        let b = RuntimeValue::Number(20.0);
        let c = RuntimeValue::String("hello".to_string());

        assert_eq!(a.compare(&b).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(b.compare(&a).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(a.compare(&a).unwrap(), std::cmp::Ordering::Equal);

        // Different types should error
        assert!(a.compare(&c).is_err());
    }

    #[test]
    fn test_equality() {
        let a = RuntimeValue::Number(10.0);
        let b = RuntimeValue::Number(10.0);
        let c = RuntimeValue::Number(20.0);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_object_access() {
        let mut obj = std::collections::HashMap::new();
        obj.insert("name".to_string(), RuntimeValue::String("John".to_string()));
        obj.insert("age".to_string(), RuntimeValue::Number(30.0));

        let runtime_obj = RuntimeValue::Object(obj);

        assert_eq!(runtime_obj.access("name"), RuntimeValue::String("John".to_string()));
        assert_eq!(runtime_obj.access("age"), RuntimeValue::Number(30.0));
        assert_eq!(runtime_obj.access("missing"), RuntimeValue::Null);
    }

    #[test]
    fn test_type_conversion() {
        let num_val = RuntimeValue::Number(42.0);
        assert_eq!(num_val.to_float().unwrap(), 42.0);
        assert_eq!(num_val.to_string(), "42");

        let str_val = RuntimeValue::String("3.14".to_string());
        assert_eq!(str_val.to_float().unwrap(), 3.14);

        let bool_val = RuntimeValue::Boolean(true);
        assert_eq!(bool_val.to_float().unwrap(), 1.0);

        let null_val = RuntimeValue::Null;
        assert_eq!(null_val.to_float().unwrap(), 0.0);
    }
}