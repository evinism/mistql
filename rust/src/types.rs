//! Runtime value types for MistQL
//!
//! This module implements the 8 core MistQL types with proper type safety,
//! conversion, equality, comparison, and truthiness operations.

use crate::executor::ExecutionError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// Custom regex wrapper that implements Serialize/Deserialize/
// TODO: Do we really need this?
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
        let mut regex_builder = regex::RegexBuilder::new(pattern);

        for flag in flags.chars() {
            match flag {
                'i' => {
                    regex_builder.case_insensitive(true);
                }
                'g' => { /* Global flag is handled by the regex engine */ }
                'm' => {
                    regex_builder.multi_line(true);
                }
                's' => {
                    regex_builder.dot_matches_new_line(true);
                }
                'x' => {
                    regex_builder.ignore_whitespace(true);
                }
                'U' => {
                    regex_builder.swap_greed(true);
                }
                _ => {
                    return Err(regex::Error::Syntax("Invalid flag".to_string()));
                }
            }
        }

        let compiled = regex_builder.build()?;
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

// The 8 core MistQL types
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

// MistQL runtime value representing all possible data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HashMap<String, RuntimeValue>),
    Array(Vec<RuntimeValue>),
    Function(String),
    Regex(MistQLRegex),
}

impl RuntimeValue {
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

    pub fn comparable(&self) -> bool {
        matches!(self, RuntimeValue::Boolean(_) | RuntimeValue::Number(_) | RuntimeValue::String(_))
    }
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: lol.
        // Always use permissive mode for display to ensure graceful output
        match self.to_serde_value(true) {
            Ok(value) => value.fmt(f),
            // Should never happen, but just in case.
            Err(_) => write!(f, "non-external"),
        }
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
            (RuntimeValue::Regex(a), RuntimeValue::Regex(b)) => a.pattern() == b.pattern() && a.flags() == b.flags(),
            _ => false,
        }
    }
}

impl Eq for RuntimeValue {}

impl RuntimeValue {
    pub fn to_serde_value(&self, permissive: bool) -> Result<serde_json::Value, ExecutionError> {
        match self {
            RuntimeValue::Null => Ok(serde_json::Value::Null),
            RuntimeValue::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
            RuntimeValue::Number(n) => match serde_json::Number::from_f64(*n) {
                Some(number) => Ok(serde_json::Value::Number(number)),
                None => Err(ExecutionError::CannotConvertToJSON(format!("float conversion failed on {}", n))),
            },
            RuntimeValue::String(s) => Ok(serde_json::Value::String(s.clone())),
            RuntimeValue::Array(arr) => {
                let mut result = Vec::new();
                for v in arr {
                    result.push(v.to_serde_value(permissive)?);
                }
                Ok(serde_json::Value::Array(result))
            }
            RuntimeValue::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, value) in obj {
                    map.insert(key.clone(), value.to_serde_value(permissive)?);
                }
                Ok(serde_json::Value::Object(map))
            }
            RuntimeValue::Function(_) => {
                if permissive {
                    Ok(serde_json::Value::String("[function]".to_string()))
                } else {
                    Err(ExecutionError::CannotConvertToJSON("function in non-permissive mode".to_string()))
                }
            }
            RuntimeValue::Regex(_) => {
                if permissive {
                    Ok(serde_json::Value::String("[regex]".to_string()))
                } else {
                    Err(ExecutionError::CannotConvertToJSON("regex in non-permissive mode".to_string()))
                }
            }
        }
    }

    pub fn to_serde_value_default(&self) -> Result<serde_json::Value, ExecutionError> {
        self.to_serde_value(false)
    }

    // Compare two values for ordering (<, >, <=, >=)
    pub fn compare(&self, other: &Self) -> Result<std::cmp::Ordering, String> {
        if self.get_type() != other.get_type() {
            return Err("Cannot compare MistQL values of different types".to_string());
        }

        if !self.comparable() {
            return Err(format!("Cannot compare MistQL values of type {}", self.get_type()));
        }

        match (self, other) {
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(a.cmp(b)),
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => a.partial_cmp(b).ok_or_else(|| "Invalid number comparison".to_string()),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(a.cmp(b)),
            _ => Err(format!("Cannot compare MistQL values of type {}", self.get_type())),
        }
    }

    // Convert to string for serialization (use escape characters)
    pub fn to_string_serialize(&self) -> String {
        match self {
            // Needs JavaScript-like number formatting for MistQL compatibility
            RuntimeValue::Number(n) => self.format_number_as_string(*n),
            // Needs string-escaping for serialization
            RuntimeValue::String(s) => format!("\"{}\"", s),
            RuntimeValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_serialize()).collect();
                format!("[{}]", items.join(","))
            }
            RuntimeValue::Object(obj) => {
                let items: Vec<String> = obj.iter().map(|(k, v)| format!("\"{}\":{}", k, v.to_string_serialize())).collect();
                format!("{{{}}}", items.join(","))
            }
            RuntimeValue::Null => "null".to_string(),
            RuntimeValue::Boolean(b) => b.to_string(),
            RuntimeValue::Function(_) => "[function]".to_string(),
            RuntimeValue::Regex(_) => "[regex]".to_string(),
        }
    }

    // Convert to string for display (don't use escape characters)
    pub fn to_string_display(&self) -> String {
        match self {
            RuntimeValue::String(s) => s.clone(),
            RuntimeValue::Number(n) => self.format_number_as_string(*n),
            RuntimeValue::Boolean(b) => b.to_string(),
            RuntimeValue::Null => "null".to_string(),
            RuntimeValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_display()).collect();
                format!("[{}]", items.join(","))
            }
            RuntimeValue::Object(obj) => {
                let items: Vec<String> = obj.iter().map(|(k, v)| format!("\"{}\":{}", k, v.to_string_display())).collect();
                format!("{{{}}}", items.join(","))
            }
            RuntimeValue::Function(_) => "[function]".to_string(),
            RuntimeValue::Regex(_) => "[regex]".to_string(),
        }
    }

    // Format a number as a string to match JavaScript's behavior
    fn format_number_as_string(&self, n: f64) -> String {
        // Handle special cases
        if n.is_nan() {
            return "NaN".to_string();
        }
        if n.is_infinite() {
            return if n > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() };
        }

        // For very large or very small numbers, use scientific notation
        let abs_n = n.abs();
        if abs_n <= 1e-7 || abs_n >= 1e21 {
            let scientific = format!("{:e}", n);
            if scientific.contains("e-") {
                return scientific;
            } else {
                return scientific.replace('e', "e+");
            }
        }

        // For numbers in the range [1e-7, 1e21), use regular formatting
        if n.fract() == 0.0 {
            format!("{:.0}", n)
        } else {
            let formatted = format!("{:.15}", n);
            let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
            trimmed.to_string()
        }
    }

    // Access object property
    pub fn access(&self, key: &str) -> RuntimeValue {
        match self {
            RuntimeValue::Object(obj) => obj.get(key).cloned().unwrap_or(RuntimeValue::Null),
            _ => RuntimeValue::Null,
        }
    }

    // Get object keys
    pub fn keys(&self) -> Vec<String> {
        match self {
            RuntimeValue::Object(obj) => obj.keys().cloned().collect(),
            _ => Vec::new(),
        }
    }
}

fn runtime_value_from_serde_value(value: &serde_json::Value) -> Result<RuntimeValue, ExecutionError> {
    match value {
        serde_json::Value::Null => Ok(RuntimeValue::Null),
        serde_json::Value::Bool(b) => Ok(RuntimeValue::Boolean(*b)),
        serde_json::Value::Number(n) => {
            match n.as_f64() {
                Some(f) => Ok(RuntimeValue::Number(f)),
                // Going against the docs, see comment:
                // https://www.mistql.com/docs/reference/types
                // Likely the original decision was because JSON does not support NaN or Infinity.
                None => Err(ExecutionError::CannotConvertToRuntimeValue(
                    "number in non-permissive mode".to_string(),
                )),
            }
        }
        serde_json::Value::String(s) => Ok(RuntimeValue::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let values = arr
                .iter()
                .map(runtime_value_from_serde_value)
                .collect::<Result<Vec<RuntimeValue>, ExecutionError>>()?;
            Ok(RuntimeValue::Array(values))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (key, value) in obj {
                map.insert(key.clone(), runtime_value_from_serde_value(value)?);
            }
            Ok(RuntimeValue::Object(map))
        }
    }
}

impl TryFrom<&serde_json::Value> for RuntimeValue {
    type Error = ExecutionError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        runtime_value_from_serde_value(value)
    }
}

impl TryFrom<serde_json::Value> for RuntimeValue {
    type Error = ExecutionError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        runtime_value_from_serde_value(&value)
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

        let runtime_val: RuntimeValue = (&json_val).try_into().unwrap();
        assert_eq!(runtime_val.get_type(), RuntimeValueType::Object);

        let back_to_json = runtime_val.to_serde_value_default().unwrap();

        // Test that the conversion preserves the structure and values
        assert_eq!(back_to_json["name"], json_val["name"]);
        assert_eq!(back_to_json["active"], json_val["active"]);

        // For numbers, we need to handle the precision difference
        if let (Some(back_age), Some(orig_age)) = (back_to_json["age"].as_f64(), json_val["age"].as_f64()) {
            assert_eq!(back_age, orig_age);
        } else {
            panic!("Age conversion failed");
        }
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
}

#[cfg(test)]
mod basic_types {
    use super::*;

    #[test]
    fn test_null_type() {
        let null_val = RuntimeValue::Null;
        assert_eq!(null_val.get_type(), RuntimeValueType::Null);
        assert!(!null_val.truthy());
        assert!(!null_val.comparable());
    }

    #[test]
    fn test_boolean_type() {
        let true_val = RuntimeValue::Boolean(true);
        let false_val = RuntimeValue::Boolean(false);

        assert_eq!(true_val.get_type(), RuntimeValueType::Boolean);
        assert_eq!(false_val.get_type(), RuntimeValueType::Boolean);

        assert!(true_val.truthy());
        assert!(!false_val.truthy());
        assert!(true_val.comparable());
        assert!(false_val.comparable());
    }

    #[test]
    fn test_number_type() {
        let pos_num = RuntimeValue::Number(42.0);
        let neg_num = RuntimeValue::Number(-10.5);
        let zero = RuntimeValue::Number(0.0);

        assert_eq!(pos_num.get_type(), RuntimeValueType::Number);
        assert_eq!(neg_num.get_type(), RuntimeValueType::Number);
        assert_eq!(zero.get_type(), RuntimeValueType::Number);

        assert!(pos_num.truthy());
        assert!(neg_num.truthy());
        assert!(!zero.truthy());
        assert!(pos_num.comparable());
        assert!(neg_num.comparable());
        assert!(zero.comparable());
    }

    #[test]
    fn test_string_type() {
        let non_empty = RuntimeValue::String("hello".to_string());
        let empty = RuntimeValue::String("".to_string());
        let unicode = RuntimeValue::String("🚀🌟".to_string());

        assert_eq!(non_empty.get_type(), RuntimeValueType::String);
        assert_eq!(empty.get_type(), RuntimeValueType::String);
        assert_eq!(unicode.get_type(), RuntimeValueType::String);

        assert!(non_empty.truthy());
        assert!(!empty.truthy());
        assert!(unicode.truthy());
        assert!(non_empty.comparable());
        assert!(empty.comparable());
        assert!(unicode.comparable());
    }

    #[test]
    fn test_object_type() {
        let mut obj = HashMap::new();
        obj.insert("key".to_string(), RuntimeValue::String("value".to_string()));
        let non_empty_obj = RuntimeValue::Object(obj);

        let empty_obj = RuntimeValue::Object(HashMap::new());

        assert_eq!(non_empty_obj.get_type(), RuntimeValueType::Object);
        assert_eq!(empty_obj.get_type(), RuntimeValueType::Object);

        assert!(non_empty_obj.truthy());
        assert!(!empty_obj.truthy());
        assert!(!non_empty_obj.comparable());
        assert!(!empty_obj.comparable());
    }

    #[test]
    fn test_array_type() {
        let non_empty_arr = RuntimeValue::Array(vec![RuntimeValue::Number(1.0), RuntimeValue::String("test".to_string())]);
        let empty_arr = RuntimeValue::Array(vec![]);

        assert_eq!(non_empty_arr.get_type(), RuntimeValueType::Array);
        assert_eq!(empty_arr.get_type(), RuntimeValueType::Array);

        assert!(non_empty_arr.truthy());
        assert!(!empty_arr.truthy());
        assert!(!non_empty_arr.comparable());
        assert!(!empty_arr.comparable());
    }

    #[test]
    fn test_function_type() {
        let func = RuntimeValue::Function("test_func".to_string());

        assert_eq!(func.get_type(), RuntimeValueType::Function);
        assert!(func.truthy());
        assert!(!func.comparable());
    }

    #[test]
    fn test_regex_type() {
        let regex = RuntimeValue::Regex(MistQLRegex::new("test", "").unwrap());

        assert_eq!(regex.get_type(), RuntimeValueType::Regex);
        assert!(regex.truthy());
        assert!(!regex.comparable());
    }
}

#[cfg(test)]
mod equality_tests {
    use super::*;

    #[test]
    fn test_null_equality() {
        let null1 = RuntimeValue::Null;
        let null2 = RuntimeValue::Null;
        assert_eq!(null1, null2);
    }

    #[test]
    fn test_boolean_equality() {
        let true1 = RuntimeValue::Boolean(true);
        let true2 = RuntimeValue::Boolean(true);
        let false1 = RuntimeValue::Boolean(false);

        assert_eq!(true1, true2);
        assert_ne!(true1, false1);
    }

    #[test]
    fn test_number_equality() {
        let num1 = RuntimeValue::Number(42.0);
        let num2 = RuntimeValue::Number(42.0);
        let num3 = RuntimeValue::Number(43.0);

        assert_eq!(num1, num2);
        assert_ne!(num1, num3);
    }

    #[test]
    fn test_string_equality() {
        let str1 = RuntimeValue::String("hello".to_string());
        let str2 = RuntimeValue::String("hello".to_string());
        let str3 = RuntimeValue::String("world".to_string());

        assert_eq!(str1, str2);
        assert_ne!(str1, str3);
    }

    #[test]
    fn test_object_equality() {
        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), RuntimeValue::Number(1.0));
        obj1.insert("b".to_string(), RuntimeValue::String("test".to_string()));

        let mut obj2 = HashMap::new();
        obj2.insert("a".to_string(), RuntimeValue::Number(1.0));
        obj2.insert("b".to_string(), RuntimeValue::String("test".to_string()));

        let mut obj3 = HashMap::new();
        obj3.insert("a".to_string(), RuntimeValue::Number(1.0));
        obj3.insert("b".to_string(), RuntimeValue::String("different".to_string()));

        let runtime_obj1 = RuntimeValue::Object(obj1);
        let runtime_obj2 = RuntimeValue::Object(obj2);
        let runtime_obj3 = RuntimeValue::Object(obj3);

        assert_eq!(runtime_obj1, runtime_obj2);
        assert_ne!(runtime_obj1, runtime_obj3);
    }

    #[test]
    fn test_array_equality() {
        let arr1 = RuntimeValue::Array(vec![RuntimeValue::Number(1.0), RuntimeValue::String("test".to_string())]);

        let arr2 = RuntimeValue::Array(vec![RuntimeValue::Number(1.0), RuntimeValue::String("test".to_string())]);

        let arr3 = RuntimeValue::Array(vec![RuntimeValue::Number(1.0), RuntimeValue::String("different".to_string())]);

        assert_eq!(arr1, arr2);
        assert_ne!(arr1, arr3);
    }

    #[test]
    fn test_function_equality() {
        let func1 = RuntimeValue::Function("test".to_string());
        let func2 = RuntimeValue::Function("test".to_string());
        let func3 = RuntimeValue::Function("different".to_string());

        assert_eq!(func1, func2);
        assert_ne!(func1, func3);
    }

    #[test]
    fn test_regex_equality() {
        let regex1 = RuntimeValue::Regex(MistQLRegex::new("test", "").unwrap());
        let regex2 = RuntimeValue::Regex(MistQLRegex::new("test", "").unwrap());
        let regex3 = RuntimeValue::Regex(MistQLRegex::new("different", "").unwrap());

        assert_eq!(regex1, regex2);
        assert_ne!(regex1, regex3);
    }

    #[test]
    fn test_cross_type_inequality() {
        let null = RuntimeValue::Null;
        let bool_val = RuntimeValue::Boolean(true);
        let num = RuntimeValue::Number(1.0);
        let str_val = RuntimeValue::String("1".to_string());

        assert_ne!(null, bool_val);
        assert_ne!(bool_val, num);
        assert_ne!(num, str_val);
        assert_ne!(str_val, null);
    }
}

#[cfg(test)]
mod comparison_tests {
    use super::*;

    #[test]
    fn test_boolean_comparison() {
        let true_val = RuntimeValue::Boolean(true);
        let false_val = RuntimeValue::Boolean(false);

        assert_eq!(false_val.compare(&true_val).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(true_val.compare(&false_val).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(true_val.compare(&true_val).unwrap(), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_number_comparison() {
        let small = RuntimeValue::Number(10.0);
        let large = RuntimeValue::Number(20.0);
        let same = RuntimeValue::Number(10.0);

        assert_eq!(small.compare(&large).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(large.compare(&small).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(small.compare(&same).unwrap(), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_string_comparison() {
        let a = RuntimeValue::String("apple".to_string());
        let b = RuntimeValue::String("banana".to_string());
        let same = RuntimeValue::String("apple".to_string());

        assert_eq!(a.compare(&b).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(b.compare(&a).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(a.compare(&same).unwrap(), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_unicode_string_comparison() {
        let emoji1 = RuntimeValue::String("🚀".to_string());
        let emoji2 = RuntimeValue::String("🌟".to_string());

        // Unicode comparison should work
        let result = emoji1.compare(&emoji2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_incomparable_types() {
        let null = RuntimeValue::Null;
        let obj = RuntimeValue::Object(HashMap::new());
        let arr = RuntimeValue::Array(vec![]);
        let func = RuntimeValue::Function("test".to_string());
        let regex = RuntimeValue::Regex(MistQLRegex::new("test", "").unwrap());

        // Null should not be comparable
        assert!(null.compare(&null).is_err());

        // Objects should not be comparable
        assert!(obj.compare(&obj).is_err());

        // Arrays should not be comparable
        assert!(arr.compare(&arr).is_err());

        // Functions should not be comparable
        assert!(func.compare(&func).is_err());

        // Regex should not be comparable
        assert!(regex.compare(&regex).is_err());
    }

    #[test]
    fn test_different_type_comparison() {
        let bool_val = RuntimeValue::Boolean(true);
        let num = RuntimeValue::Number(1.0);
        let str_val = RuntimeValue::String("1".to_string());

        assert!(bool_val.compare(&num).is_err());
        assert!(num.compare(&str_val).is_err());
        assert!(str_val.compare(&bool_val).is_err());
    }
}

#[cfg(test)]
mod serde_conversion_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_null_conversion() {
        let json_null = json!(null);
        let runtime_null: RuntimeValue = (&json_null).try_into().unwrap();
        assert_eq!(runtime_null, RuntimeValue::Null);

        let back_to_json = runtime_null.to_serde_value_default().unwrap();
        let back_to_json: serde_json::Value = back_to_json.try_into().unwrap();
        assert_eq!(back_to_json, json_null);
    }

    #[test]
    fn test_boolean_conversion() {
        let json_true = json!(true);
        let json_false = json!(false);

        let runtime_true: RuntimeValue = (&json_true).try_into().unwrap();
        let runtime_false: RuntimeValue = (&json_false).try_into().unwrap();

        assert_eq!(runtime_true, RuntimeValue::Boolean(true));
        assert_eq!(runtime_false, RuntimeValue::Boolean(false));

        assert_eq!(runtime_true.to_serde_value_default().unwrap(), json_true);
        assert_eq!(runtime_false.to_serde_value_default().unwrap(), json_false);
    }

    #[test]
    fn test_number_conversion() {
        let json_int = json!(42);
        let json_float = json!(3.14);
        let json_negative = json!(-100);

        let runtime_int: RuntimeValue = (&json_int).try_into().unwrap();
        let runtime_float: RuntimeValue = (&json_float).try_into().unwrap();
        let runtime_negative: RuntimeValue = (&json_negative).try_into().unwrap();

        assert_eq!(runtime_int, RuntimeValue::Number(42.0));
        assert_eq!(runtime_float, RuntimeValue::Number(3.14));
        assert_eq!(runtime_negative, RuntimeValue::Number(-100.0));

        // Note: When converting back to JSON, integers become floats
        // This is expected behavior since RuntimeValue stores all numbers as f64
        let back_int = runtime_int.to_serde_value_default().unwrap();
        let back_float = runtime_float.to_serde_value_default().unwrap();
        let back_negative = runtime_negative.to_serde_value_default().unwrap();

        // Test that the numeric values are preserved
        assert_eq!(back_int.as_f64().unwrap(), 42.0);
        assert_eq!(back_float.as_f64().unwrap(), 3.14);
        assert_eq!(back_negative.as_f64().unwrap(), -100.0);
    }

    #[test]
    fn test_string_conversion() {
        let json_str = json!("hello world");
        let json_empty = json!("");
        let json_unicode = json!("🚀🌟");

        let runtime_str: RuntimeValue = (&json_str).try_into().unwrap();
        let runtime_empty: RuntimeValue = (&json_empty).try_into().unwrap();
        let runtime_unicode: RuntimeValue = (&json_unicode).try_into().unwrap();

        assert_eq!(runtime_str, RuntimeValue::String("hello world".to_string()));
        assert_eq!(runtime_empty, RuntimeValue::String("".to_string()));
        assert_eq!(runtime_unicode, RuntimeValue::String("🚀🌟".to_string()));

        assert_eq!(runtime_str.to_serde_value_default().unwrap(), json_str);
        assert_eq!(runtime_empty.to_serde_value_default().unwrap(), json_empty);
        assert_eq!(runtime_unicode.to_serde_value_default().unwrap(), json_unicode);
    }

    #[test]
    fn test_array_conversion() {
        let json_arr = json!([1, "test", true, null]);
        let runtime_arr: RuntimeValue = (&json_arr).try_into().unwrap();

        let expected = RuntimeValue::Array(vec![
            RuntimeValue::Number(1.0),
            RuntimeValue::String("test".to_string()),
            RuntimeValue::Boolean(true),
            RuntimeValue::Null,
        ]);

        assert_eq!(runtime_arr, expected);

        // Test that the conversion preserves the structure and values
        let back_to_json = runtime_arr.to_serde_value_default().unwrap();
        if let Some(back_array) = back_to_json.as_array() {
            assert_eq!(back_array.len(), 4);
            assert_eq!(back_array[0].as_f64().unwrap(), 1.0);
            assert_eq!(back_array[1].as_str().unwrap(), "test");
            assert_eq!(back_array[2].as_bool().unwrap(), true);
            assert!(back_array[3].is_null());
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_object_conversion() {
        let json_obj = json!({
            "name": "John",
            "age": 30,
            "active": true,
            "scores": [1, 2, 3]
        });

        let runtime_obj: RuntimeValue = (&json_obj).try_into().unwrap();
        let back_to_json = runtime_obj.to_serde_value_default().unwrap();

        // Test that the conversion preserves the structure and values
        assert_eq!(back_to_json["name"], json_obj["name"]);
        assert_eq!(back_to_json["active"], json_obj["active"]);

        // For numbers, test the numeric value rather than exact JSON type
        assert_eq!(back_to_json["age"].as_f64().unwrap(), 30.0);
        assert_eq!(json_obj["age"].as_i64().unwrap(), 30);

        // For arrays, test the structure
        if let (Some(back_scores), Some(orig_scores)) = (back_to_json["scores"].as_array(), json_obj["scores"].as_array()) {
            assert_eq!(back_scores.len(), orig_scores.len());
            for (back_score, orig_score) in back_scores.iter().zip(orig_scores.iter()) {
                assert_eq!(back_score.as_f64().unwrap(), orig_score.as_i64().unwrap() as f64);
            }
        } else {
            panic!("Expected arrays");
        }
    }

    #[test]
    fn test_nested_structure_conversion() {
        let json_nested = json!({
            "users": [
                {"name": "Alice", "age": 25},
                {"name": "Bob", "age": 30}
            ],
            "metadata": {
                "count": 2,
                "active": true
            }
        });

        let runtime_nested: RuntimeValue = (&json_nested).try_into().unwrap();
        let back_to_json = runtime_nested.to_serde_value_default().unwrap();

        // Test that the structure is preserved
        assert_eq!(back_to_json["metadata"]["active"], json_nested["metadata"]["active"]);
        assert_eq!(back_to_json["metadata"]["count"].as_f64().unwrap(), 2.0);

        // Test users array structure
        if let (Some(back_users), Some(orig_users)) = (back_to_json["users"].as_array(), json_nested["users"].as_array()) {
            assert_eq!(back_users.len(), orig_users.len());
            for (back_user, orig_user) in back_users.iter().zip(orig_users.iter()) {
                assert_eq!(back_user["name"], orig_user["name"]);
                assert_eq!(back_user["age"].as_f64().unwrap(), orig_user["age"].as_i64().unwrap() as f64);
            }
        } else {
            panic!("Expected users arrays");
        }
    }

    #[test]
    fn test_function_and_regex_serialization() {
        let func = RuntimeValue::Function("test_func".to_string());
        let regex = RuntimeValue::Regex(MistQLRegex::new("test", "i").unwrap());

        // Test permissive mode
        let func_json = func.to_serde_value(true).unwrap(); // Use permissive mode
        let regex_json = regex.to_serde_value(true).unwrap(); // Use permissive mode

        assert_eq!(func_json, json!("[function]"));
        assert_eq!(regex_json, json!("[regex]"));

        // Test non-permissive mode (should return errors)
        assert!(func.to_serde_value(false).is_err());
        assert!(regex.to_serde_value(false).is_err());
    }
}

#[cfg(test)]
mod type_conversion_tests {
    use super::*;

    #[test]
    fn test_to_string_conversion() {
        // String to string
        let str_val = RuntimeValue::String("hello".to_string());
        assert_eq!(str_val.to_string_serialize(), "\"hello\"");

        // Number to string
        let int_num = RuntimeValue::Number(42.0);
        assert_eq!(int_num.to_string_serialize(), "42");

        let float_num = RuntimeValue::Number(3.14);
        assert_eq!(float_num.to_string_serialize(), "3.14");

        // Other types use JSON serialization
        let bool_val = RuntimeValue::Boolean(true);
        assert_eq!(bool_val.to_string_serialize(), "true");

        let null = RuntimeValue::Null;
        assert_eq!(null.to_string_serialize(), "null");
    }

    #[test]
    fn test_to_string_complex_types() {
        let obj = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("key".to_string(), RuntimeValue::String("value".to_string()));
            map
        });

        let arr = RuntimeValue::Array(vec![RuntimeValue::Number(1.0), RuntimeValue::String("test".to_string())]);

        // These should serialize to JSON
        let obj_str = obj.to_string_serialize();
        assert!(obj_str.contains("key"));
        assert!(obj_str.contains("value"));

        let arr_str = arr.to_string_serialize();
        assert!(arr_str.contains("1"));
        assert!(arr_str.contains("test"));
    }
}

#[cfg(test)]
mod object_operation_tests {
    use super::*;

    #[test]
    fn test_object_access() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), RuntimeValue::String("John".to_string()));
        obj.insert("age".to_string(), RuntimeValue::Number(30.0));
        obj.insert("active".to_string(), RuntimeValue::Boolean(true));

        let runtime_obj = RuntimeValue::Object(obj);

        assert_eq!(runtime_obj.access("name"), RuntimeValue::String("John".to_string()));
        assert_eq!(runtime_obj.access("age"), RuntimeValue::Number(30.0));
        assert_eq!(runtime_obj.access("active"), RuntimeValue::Boolean(true));
        assert_eq!(runtime_obj.access("missing"), RuntimeValue::Null);
    }

    #[test]
    fn test_object_access_non_object() {
        let str_val = RuntimeValue::String("hello".to_string());
        let num_val = RuntimeValue::Number(42.0);
        let null_val = RuntimeValue::Null;

        assert_eq!(str_val.access("key"), RuntimeValue::Null);
        assert_eq!(num_val.access("key"), RuntimeValue::Null);
        assert_eq!(null_val.access("key"), RuntimeValue::Null);
    }

    #[test]
    fn test_object_keys() {
        let mut obj = HashMap::new();
        obj.insert("a".to_string(), RuntimeValue::Number(1.0));
        obj.insert("b".to_string(), RuntimeValue::Number(2.0));
        obj.insert("c".to_string(), RuntimeValue::Number(3.0));

        let runtime_obj = RuntimeValue::Object(obj);
        let mut keys = runtime_obj.keys();
        keys.sort(); // Sort for consistent testing

        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_object_keys_non_object() {
        let str_val = RuntimeValue::String("hello".to_string());
        let num_val = RuntimeValue::Number(42.0);
        let null_val = RuntimeValue::Null;

        assert_eq!(str_val.keys(), Vec::<String>::new());
        assert_eq!(num_val.keys(), Vec::<String>::new());
        assert_eq!(null_val.keys(), Vec::<String>::new());
    }

    #[test]
    fn test_empty_object() {
        let empty_obj = RuntimeValue::Object(HashMap::new());

        assert_eq!(empty_obj.access("any_key"), RuntimeValue::Null);
        assert_eq!(empty_obj.keys(), Vec::<String>::new());
        assert!(!empty_obj.truthy());
    }
}

#[cfg(test)]
mod regex_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_regex_creation() {
        let regex = MistQLRegex::new("test", "").unwrap();
        assert_eq!(regex.pattern(), "test");
        assert_eq!(regex.flags(), "");
    }

    #[test]
    fn test_regex_with_flags() {
        let regex = MistQLRegex::new("test", "i").unwrap();
        assert_eq!(regex.pattern(), "test");
        assert_eq!(regex.flags(), "i");
    }

    #[test]
    fn test_regex_equality() {
        let regex1 = MistQLRegex::new("test", "").unwrap();
        let regex2 = MistQLRegex::new("test", "").unwrap();
        let regex3 = MistQLRegex::new("different", "").unwrap();
        let regex4 = MistQLRegex::new("test", "i").unwrap();

        assert_eq!(regex1, regex2);
        assert_ne!(regex1, regex3);
        assert_ne!(regex1, regex4);
    }

    #[test]
    fn test_regex_serialization() {
        let regex = MistQLRegex::new("test", "i").unwrap();
        let runtime_regex = RuntimeValue::Regex(regex);

        let json = runtime_regex.to_serde_value(true).unwrap(); // Use permissive mode
        assert_eq!(json, json!("[regex]"));
    }

    #[test]
    fn test_invalid_regex() {
        let result = MistQLRegex::new("[invalid", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_regex_compilation() {
        let regex = MistQLRegex::new(r"\d+", "").unwrap();
        let compiled = regex.as_regex();

        // Test that the compiled regex works
        assert!(compiled.is_match("123"));
        assert!(!compiled.is_match("abc"));
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_unicode_strings() {
        let unicode_str = RuntimeValue::String("Hello 世界 🌍".to_string());
        assert_eq!(unicode_str.get_type(), RuntimeValueType::String);
        assert!(unicode_str.truthy());
        assert_eq!(unicode_str.to_string_serialize(), "\"Hello 世界 🌍\"");
    }

    #[test]
    fn test_empty_structures() {
        let empty_str = RuntimeValue::String("".to_string());
        let empty_arr = RuntimeValue::Array(vec![]);
        let empty_obj = RuntimeValue::Object(HashMap::new());

        assert!(!empty_str.truthy());
        assert!(!empty_arr.truthy());
        assert!(!empty_obj.truthy());
    }

    #[test]
    fn test_nested_empty_structures() {
        let nested_empty = RuntimeValue::Array(vec![
            RuntimeValue::Array(vec![]),
            RuntimeValue::Object(HashMap::new()),
            RuntimeValue::String("".to_string()),
        ]);

        // The array itself is truthy because it's not empty
        assert!(nested_empty.truthy());

        // But its elements are falsy
        if let RuntimeValue::Array(arr) = &nested_empty {
            assert!(!arr[0].truthy()); // Empty array
            assert!(!arr[1].truthy()); // Empty object
            assert!(!arr[2].truthy()); // Empty string
        }
    }

    #[test]
    fn test_deep_nesting() {
        let mut level3 = HashMap::new();
        level3.insert("value".to_string(), RuntimeValue::String("deep".to_string()));

        let level2 = RuntimeValue::Object(level3);
        let level1 = RuntimeValue::Array(vec![level2]);

        let root = RuntimeValue::Object({
            let mut map = HashMap::new();
            map.insert("nested".to_string(), level1);
            map
        });

        assert!(root.truthy());
        assert_eq!(root.get_type(), RuntimeValueType::Object);
    }

    #[test]
    fn test_special_string_values() {
        let newline_str = RuntimeValue::String("\n".to_string());
        let tab_str = RuntimeValue::String("\t".to_string());
        let space_str = RuntimeValue::String(" ".to_string());

        // All non-empty strings are truthy
        assert!(newline_str.truthy());
        assert!(tab_str.truthy());
        assert!(space_str.truthy());
    }

    #[test]
    fn test_number_edge_cases() {
        let zero = RuntimeValue::Number(0.0);
        let negative_zero = RuntimeValue::Number(-0.0);

        assert!(!zero.truthy());
        assert!(!negative_zero.truthy());

        // -0.0 should equal 0.0
        assert_eq!(zero, negative_zero);
    }
}
