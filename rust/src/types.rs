//! Runtime value types for MistQL
//!
//! This module implements the 8 core MistQL types with proper type safety,
//! conversion, equality, comparison, and truthiness operations.

use crate::executor::ExecutionError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// The 8 core MistQL types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MistQLValueType {
    Null,
    Boolean,
    Number,
    String,
    Object,
    Array,
    Function,
    Regex,
}

impl fmt::Display for MistQLValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MistQLValueType::Null => write!(f, "null"),
            MistQLValueType::Boolean => write!(f, "boolean"),
            MistQLValueType::Number => write!(f, "number"),
            MistQLValueType::String => write!(f, "string"),
            MistQLValueType::Object => write!(f, "object"),
            MistQLValueType::Array => write!(f, "array"),
            MistQLValueType::Function => write!(f, "function"),
            MistQLValueType::Regex => write!(f, "regex"),
        }
    }
}

// Custom regex wrapper that implements Serialize/Deserialize/
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
                // Not supported in the Javascript version, so we ignore these.
                // 'x' => {
                //     regex_builder.ignore_whitespace(true);
                // }
                // 'U' => {
                //     regex_builder.swap_greed(true);
                // }
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

pub enum Accessor<'a> {
    Key(&'a str),
    Index(usize),
}
impl<'a> From<&'a str> for Accessor<'a> {
    fn from(key: &'a str) -> Self {
        Accessor::Key(key)
    }
}
impl<'a> From<usize> for Accessor<'a> {
    fn from(index: usize) -> Self {
        Accessor::Index(index)
    }
}

#[derive(Debug, Clone)]
pub enum JsonView<'a> {
    Borrowed(&'a Value),
    Owned(Rc<Value>),
}

impl<'a> JsonView<'a> {
    pub fn as_value(&self) -> &Value {
        match self {
            JsonView::Borrowed(value) => value,
            JsonView::Owned(value) => value.as_ref(),
        }
    }

    pub fn get<'s, A>(&'s self, accessor: A) -> Option<JsonView<'s>>
    where
        A: Into<Accessor<'s>>,
    {
        match accessor.into() {
            Accessor::Key(k) => self.as_value().get(k).map(JsonView::from),
            Accessor::Index(i) => self.as_value().get(i).map(JsonView::from),
        }
    }
    pub fn keys(&self) -> Vec<&str> {
        self.as_value().as_object().unwrap().keys().map(|k| k.as_str()).collect()
    }
    pub fn iter_object(&self) -> Option<impl '_ + Iterator<Item = (&str, JsonView<'_>)>> {
        self.as_value()
            .as_object()
            .map(|m: &Map<String, Value>| m.iter().map(|(k, v)| (k.as_str(), JsonView::from(v))))
    }
    pub fn iter_array(&self) -> Option<impl '_ + Iterator<Item = JsonView<'_>>> {
        self.as_value().as_array().map(|arr: &Vec<Value>| arr.iter().map(JsonView::from))
    }
    pub fn to_owned_value(&self) -> Value {
        self.as_value().clone()
    }
}
impl<'a> From<&'a Value> for JsonView<'a> {
    fn from(value: &'a Value) -> Self {
        JsonView::Borrowed(value)
    }
}
impl<'a> From<Value> for JsonView<'a> {
    fn from(value: Value) -> Self {
        JsonView::Owned(Rc::new(value))
    }
}
impl<'a> fmt::Display for JsonView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_value().fmt(f)
    }
}
impl<'a> PartialEq for JsonView<'a> {
    fn eq(&self, other: &Self) -> bool {
        ValueView::from(self).as_computable().unwrap() == ValueView::from(other).as_computable().unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum ValueView<'a> {
    Json(JsonView<'a>),
    Function(Cow<'a, str>),
    Regex(&'a MistQLRegex),
}
impl<'a> ValueView<'a> {
    pub fn as_json(&self) -> Option<&JsonView<'a>> {
        if let ValueView::Json(json) = self {
            Some(json)
        } else {
            None
        }
    }
    pub fn as_value(&self) -> Option<&Value> {
        if let ValueView::Json(json) = self {
            Some(json.as_value())
        } else {
            None
        }
    }

    pub fn get_type(&self) -> MistQLValueType {
        match self {
            ValueView::Json(json) => match json.as_value() {
                Value::Null => MistQLValueType::Null,
                Value::Bool(_) => MistQLValueType::Boolean,
                Value::Number(_) => MistQLValueType::Number,
                Value::String(_) => MistQLValueType::String,
                Value::Array(_) => MistQLValueType::Array,
                Value::Object(_) => MistQLValueType::Object,
            },
            ValueView::Function(_) => MistQLValueType::Function,
            ValueView::Regex(_) => MistQLValueType::Regex,
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            ValueView::Json(json) => match json.as_value() {
                Value::Null => false,
                Value::Bool(b) => *b,
                Value::Number(n) => n.as_f64().unwrap() != 0.0,
                Value::String(s) => !s.is_empty(),
                Value::Array(arr) => !arr.is_empty(),
                Value::Object(obj) => !obj.is_empty(),
            },
            ValueView::Function(_) => true,
            ValueView::Regex(_) => true,
        }
    }

    pub fn comparable(&self) -> bool {
        match self {
            ValueView::Json(json) => match json.as_value() {
                Value::Null => false,
                Value::Bool(_) => true,
                Value::Number(_) => true,
                Value::String(_) => true,
                Value::Array(_) => true,
                Value::Object(_) => true,
            },
            ValueView::Function(_) => false,
            ValueView::Regex(_) => false,
        }
    }

    // Compare two values for ordering (<, >, <=, >=)
    pub fn compare(&self, other: &Self) -> Result<std::cmp::Ordering, ExecutionError> {
        if self.get_type() != other.get_type() {
            return Err(ExecutionError::CannotCompare(
                "Cannot compare MistQL values of different types".to_string(),
            ));
        }
        if !self.comparable() {
            return Err(ExecutionError::CannotCompare(format!(
                "Cannot compare MistQL values of type {}",
                self.get_type()
            )));
        }
        match (self.as_computable().unwrap(), other.as_computable().unwrap()) {
            (ComputableValue::Boolean(a), ComputableValue::Boolean(b)) => Ok(a.cmp(&b)),
            (ComputableValue::Number(a), ComputableValue::Number(b)) => a
                .partial_cmp(&b)
                .ok_or_else(|| ExecutionError::CannotCompare("Invalid number comparison".to_string())),
            (ComputableValue::String(a), ComputableValue::String(b)) => Ok(a.cmp(&b)),
            _ => Err(ExecutionError::TypeMismatch(format!(
                "Cannot compare MistQL values of type {}",
                self.get_type()
            ))),
        }
    }

    pub fn as_computable<'s>(&'s self) -> Result<ComputableValue<'s>, ExecutionError> {
        match self {
            ValueView::Json(json) => match json.as_value() {
                Value::Null => Ok(ComputableValue::Null),
                Value::Bool(b) => Ok(ComputableValue::Boolean(*b)),
                Value::Number(n) => {
                    match n.as_f64() {
                        Some(f) => Ok(ComputableValue::Number(f)),
                        // Going against the docs, see comment:
                        // https://www.mistql.com/docs/reference/types
                        // Likely the original decision was because JSON does not support NaN or Infinity.
                        None => Err(ExecutionError::CannotConvertToComputableValue(
                            "encountered a non-f64 number".to_string(),
                        )),
                    }
                }
                Value::String(s) => Ok(ComputableValue::String(s.clone())),
                Value::Array(_) => Ok(ComputableValue::Array(json.as_value().as_array().unwrap())),
                Value::Object(_) => Ok(ComputableValue::Object(json.as_value().as_object().unwrap())),
            },
            ValueView::Function(name) => Ok(ComputableValue::Function(name.to_string())),
            ValueView::Regex(regex) => Ok(ComputableValue::Regex(regex)),
        }
    }

    // Convert to string for serialization (use escape characters)
    pub fn to_string_serialize(&self) -> String {
        let computable = self.as_computable().unwrap();
        match computable {
            // Needs JavaScript-like number formatting for MistQL compatibility
            ComputableValue::Number(n) => self.format_number_as_string(n),
            // Needs string-escaping for serialization
            ComputableValue::String(s) => format!("\"{}\"", s),
            ComputableValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| ValueView::from(v).to_string_serialize()).collect();
                format!("[{}]", items.join(","))
            }
            ComputableValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, ValueView::from(v).to_string_serialize()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
            ComputableValue::Null => "null".to_string(),
            ComputableValue::Boolean(b) => b.to_string(),
            ComputableValue::Function(_) => "[function]".to_string(),
            ComputableValue::Regex(_) => "[regex]".to_string(),
        }
    }

    // Convert to string for display (don't use escape characters)
    pub fn to_string_display(&self) -> String {
        let computable = self.as_computable().unwrap();
        match computable {
            ComputableValue::String(s) => s.clone(),
            ComputableValue::Number(n) => self.format_number_as_string(n),
            ComputableValue::Boolean(b) => b.to_string(),
            ComputableValue::Null => "null".to_string(),
            ComputableValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| ValueView::from(v).to_string_display()).collect();
                format!("[{}]", items.join(","))
            }
            ComputableValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, ValueView::from(v).to_string_display()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
            ComputableValue::Function(_) => "[function]".to_string(),
            ComputableValue::Regex(_) => "[regex]".to_string(),
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
}
impl<'a> From<JsonView<'a>> for ValueView<'a> {
    fn from(json: JsonView<'a>) -> Self {
        ValueView::Json(json)
    }
}
impl<'a> From<&'a JsonView<'a>> for ValueView<'a> {
    fn from(json: &'a JsonView<'a>) -> Self {
        ValueView::Json(json.clone())
    }
}
impl<'a> From<&'a MistQLRegex> for ValueView<'a> {
    fn from(regex: &'a MistQLRegex) -> Self {
        ValueView::Regex(regex)
    }
}
impl<'a> From<&'a Value> for ValueView<'a> {
    fn from(value: &'a Value) -> Self {
        ValueView::Json(JsonView::from(value))
    }
}
impl<'a> From<Value> for ValueView<'a> {
    fn from(value: Value) -> Self {
        ValueView::Json(JsonView::from(value))
    }
}
impl<'a> From<bool> for ValueView<'a> {
    fn from(b: bool) -> Self {
        ValueView::from(Value::Bool(b))
    }
}
impl<'a> From<f64> for ValueView<'a> {
    fn from(n: f64) -> Self {
        ValueView::from(Value::from(n))
    }
}
impl<'a> From<String> for ValueView<'a> {
    fn from(s: String) -> Self {
        ValueView::from(Value::from(s))
    }
}
impl<'a> From<&'a str> for ValueView<'a> {
    fn from(s: &'a str) -> Self {
        ValueView::from(Value::from(s))
    }
}
impl<'a> From<&'a Vec<Value>> for ValueView<'a> {
    fn from(arr: &'a Vec<Value>) -> Self {
        ValueView::from(Value::Array(arr.clone()))
    }
}
impl<'a> From<&'a Map<String, Value>> for ValueView<'a> {
    fn from(obj: &'a Map<String, Value>) -> Self {
        ValueView::from(Value::Object(obj.clone()))
    }
}
impl<'a> From<&'a ComputableValue<'a>> for ValueView<'a> {
    fn from(c: ComputableValue<'a>) -> Self {
        match c {
            ComputableValue::Null => ValueView::from(Value::Null),
            ComputableValue::Boolean(b) => ValueView::from(b),
            ComputableValue::Number(n) => ValueView::from(n),
            ComputableValue::String(s) => ValueView::from(s),
            ComputableValue::Array(arr) => ValueView::from(arr),
            ComputableValue::Object(obj) => ValueView::from(obj),
            ComputableValue::Function(s) => ValueView::Function(Cow::Borrowed(&s)),
            ComputableValue::Regex(regex) => ValueView::Regex(regex),
        }
    }
}
impl<'a> From<HashMap<String, ValueView<'a>>> for ValueView<'a> {
    fn from(obj: HashMap<String, ValueView<'a>>) -> Self {
        let a = obj.into_iter().map(|(k, v)| (k, v.as_value().unwrap().clone()));
        let o = Value::from_iter(a);
        ValueView::Json(JsonView::from(o))
    }
}
impl<'a> fmt::Display for ValueView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueView::Json(json) => json.as_value().fmt(f),
            ValueView::Function(name) => write!(f, "{}", name),
            ValueView::Regex(regex) => write!(f, "{}", regex.pattern()),
        }
    }
}
impl<'a> PartialEq for ValueView<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_computable().unwrap() == other.as_computable().unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum ComputableValue<'a> {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(&'a Vec<Value>),
    Object(&'a Map<String, Value>),
    Function(String),
    Regex(&'a MistQLRegex),
}

impl<'a> PartialEq for ComputableValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ComputableValue::Null, ComputableValue::Null) => true,
            (ComputableValue::Boolean(a), ComputableValue::Boolean(b)) => a == b,
            (ComputableValue::Number(a), ComputableValue::Number(b)) => a == b,
            (ComputableValue::String(a), ComputableValue::String(b)) => a == b,
            (ComputableValue::Array(a), ComputableValue::Array(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(a, b)| ValueView::from(a) == ValueView::from(b))
            }
            (ComputableValue::Object(a), ComputableValue::Object(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for (key, value) in a.iter() {
                    if let Some(other_value) = b.get(key) {
                        if ValueView::from(value) != ValueView::from(other_value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            }
            (ComputableValue::Function(a), ComputableValue::Function(b)) => a == b,
            (ComputableValue::Regex(a), ComputableValue::Regex(b)) => a.pattern() == b.pattern() && a.flags() == b.flags(),
            _ => false,
        }
    }
}

impl<'a> Eq for ComputableValue<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_view_creation() {
        let json_val = json!({
            "name": "John🚀🌟",
            "age": 30,
            "active": true,
            "scores": [1, 2, 3, 5.0, 42.0, -10.5, 0.0],
            "null": null,
            "object": {
                "key": "value"
            }
        });

        let runtime_val = ValueView::from(&json_val);
        assert_eq!(runtime_val.get_type(), MistQLValueType::Object);
        let runtime_name = runtime_val.as_json().unwrap().get_key("name").unwrap();
        let runtime_active = runtime_val.as_json().unwrap().get_key("active").unwrap();
        let runtime_scores = runtime_val.as_json().unwrap().get_key("scores").unwrap();
        let runtime_null = runtime_val.as_json().unwrap().get_key("null").unwrap();
        let runtime_object = runtime_val.as_json().unwrap().get_key("object").unwrap();

        // Test that the conversion preserves the structure and values
        assert_eq!(runtime_name.as_value(), &json_val["name"]);
        assert_eq!(runtime_active.as_value(), &json_val["active"]);
        assert_eq!(runtime_scores.as_value(), &json_val["scores"]);
        assert_eq!(runtime_null.as_value(), &json_val["null"]);
        assert_eq!(runtime_object.as_value(), &json_val["object"]);
    }

    #[test]
    fn test_null_type() {
        let null = ValueView::from(json!(null));
        assert_eq!(null.get_type(), MistQLValueType::Null);
        assert!(!null.truthy());
        assert!(!null.comparable());
        assert!(null.to_string_display() == "null");
        assert!(null.to_string_serialize() == "null");
        assert_eq!(null.clone(), null.clone());
    }

    #[test]
    fn test_boolean_type() {
        let true_val = ValueView::from(json!(true));
        let false_val = ValueView::from(json!(false));
        assert_eq!(true_val.get_type(), MistQLValueType::Boolean);
        assert_eq!(false_val.get_type(), MistQLValueType::Boolean);
        assert!(true_val.to_string_display() == "true");
        assert!(false_val.to_string_display() == "false");
        assert!(true_val.to_string_serialize() == "true");
        assert!(false_val.to_string_serialize() == "false");
        assert_eq!(true_val.clone(), true_val.clone());
        assert_ne!(true_val.clone(), false_val.clone());
    }

    #[test]
    fn test_number_type() {
        let num1 = ValueView::from(json!(1.0));
        assert_eq!(num1.get_type(), MistQLValueType::Number);
        assert!(num1.truthy());
        assert!(num1.comparable());
        assert!(num1.to_string_display() == "1.0");
        assert!(num1.to_string_serialize() == "1.0");
        let num2 = ValueView::from(json!(1e-49));
        assert!(num2.to_string_display() == "1e-49");
        assert_eq!(num1.clone(), num1.clone());
        assert_ne!(num1.clone(), num2.clone());
    }

    #[test]
    fn test_string_type() {
        let str = ValueView::from(json!("test"));
        assert_eq!(str.get_type(), MistQLValueType::String);
        assert!(str.truthy());
        assert!(str.comparable());
        assert!(str.to_string_display() == "test");
        assert!(str.to_string_serialize() == "test");
        assert_eq!(str.clone(), str.clone());
        assert_ne!(str.clone(), ValueView::from(json!("test2")));
    }

    #[test]
    fn test_object_type() {
        let obj = ValueView::from(json!({"a": 1.0, "b": "test"}));
        assert_eq!(obj.get_type(), MistQLValueType::Object);
        assert!(!obj.truthy());
        assert!(!obj.comparable());
        assert_eq!(obj.as_json().unwrap().keys(), vec!["a", "b"]);
        assert!(obj.to_string_display() == "{a: 1.0, b: test}");
        assert!(obj.to_string_serialize() == "{\"a\": 1.0, \"b\": \"test\"}");
        assert_eq!(obj.clone(), obj.clone());
        assert_ne!(obj.clone(), ValueView::from(json!({"a": 1.0, "b": "test2"})));
    }

    #[test]
    fn test_array_type() {
        let arr = ValueView::from(json!([1.0, "test"]));
        assert_eq!(arr.get_type(), MistQLValueType::Array);
        assert!(!arr.truthy());
        assert!(!arr.comparable());
        assert_eq!(arr.as_json().unwrap().keys(), vec!["0", "1"]);
        assert!(arr.to_string_display() == "[1.0, test]");
        assert!(arr.to_string_serialize() == "[1.0, \"test\"]");
        assert_eq!(arr.clone(), arr.clone());
        assert_ne!(arr.clone(), ValueView::from(json!([1.0, "test2"])));
    }

    #[test]
    fn test_function_type() {
        let func = ValueView::from(json!("test_func"));

        assert_eq!(func.get_type(), MistQLValueType::Function);
        assert!(func.truthy());
        assert!(!func.comparable());
        assert!(func.to_string_display() == "test_func");
        assert!(func.to_string_serialize() == "test_func");
        assert_eq!(func.clone(), func.clone());
        assert_ne!(func.clone(), ValueView::from(json!("test_func2")));
    }

    #[test]
    fn test_regex_type() {
        let regex = MistQLRegex::new("test", "").unwrap();
        let v = ValueView::from(&regex);

        assert_eq!(v.get_type(), MistQLValueType::Regex);
        assert!(v.truthy());
        assert!(!v.comparable());
        assert!(v.to_string_display() == "test");
        assert!(v.to_string_serialize() == "test");
        assert_eq!(v.clone(), v.clone());
        assert_ne!(v.clone(), ValueView::from(&MistQLRegex::new("test2", "").unwrap()));
    }

    #[test]
    fn test_boolean_comparison() {
        let true_val = ValueView::from(json!(true));
        let false_val = ValueView::from(json!(false));

        assert_eq!(false_val.compare(&true_val).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(true_val.compare(&false_val).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(true_val.compare(&true_val).unwrap(), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_number_comparison() {
        let small = ValueView::from(json!(10.0));
        let large = ValueView::from(json!(20.0));
        let same = ValueView::from(json!(10.0));

        assert_eq!(small.compare(&large).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(large.compare(&small).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(small.compare(&same).unwrap(), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_string_comparison() {
        let a = ValueView::from(json!("apple"));
        let b = ValueView::from(json!("banana"));
        let same = ValueView::from(json!("apple"));

        assert_eq!(a.compare(&b).unwrap(), std::cmp::Ordering::Less);
        assert_eq!(b.compare(&a).unwrap(), std::cmp::Ordering::Greater);
        assert_eq!(a.compare(&same).unwrap(), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_unicode_string_comparison() {
        let emoji1 = ValueView::from(json!("🚀"));
        let emoji2 = ValueView::from(json!("🌟"));

        // Unicode comparison should work
        let result = emoji1.compare(&emoji2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_incomparable_types() {
        let null = ValueView::from(json!(null));
        let obj = ValueView::from(json!({}));
        let arr = ValueView::from(json!([]));

        let func = ValueView::from(json!("test"));
        let regex_obj = MistQLRegex::new("test", "").unwrap();
        let regex = ValueView::from(&regex_obj);

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
        let bool_val = ValueView::from(json!(true));
        let num = ValueView::from(json!(1.0));
        let str_val = ValueView::from(json!("1"));

        assert!(bool_val.compare(&num).is_err());
        assert!(num.compare(&str_val).is_err());
        assert!(str_val.compare(&bool_val).is_err());
    }

    #[test]
    fn test_object_access() {
        let json_val = json!({
            "name": "John",
            "age": 30.0,
            "active": true
        });
        let runtime_obj = ValueView::from(&json_val);
        let runtime_name = runtime_obj.as_json().unwrap().get_key("name").unwrap();
        let runtime_age = runtime_obj.as_json().unwrap().get_key("age").unwrap();
        let runtime_active = runtime_obj.as_json().unwrap().get_key("active").unwrap();
        let runtime_missing = runtime_obj.as_json().unwrap().get_key("missing").unwrap();

        assert_eq!(runtime_name.as_value(), &json_val["name"]);
        assert_eq!(runtime_age.as_value(), &json_val["age"]);
        assert_eq!(runtime_active.as_value(), &json_val["active"]);
        assert_eq!(runtime_missing.as_value(), &json_val["missing"]);

        assert_eq!(runtime_obj.as_json().unwrap().keys(), vec!["name", "age", "active"]);
    }

    #[test]
    fn test_empty_object() {
        let empty_obj = ValueView::from(json!({}));
        let runtime_missing = empty_obj.as_json().unwrap().get_key("missing").unwrap();

        assert_eq!(runtime_missing.as_value(), &json!(null));
        assert_eq!(empty_obj.as_json().unwrap().keys(), Vec::<String>::new());
        assert!(!empty_obj.clone().truthy());
    }

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

    #[test]
    fn test_unicode_strings() {
        let unicode_str = ValueView::from(json!("Hello 世界 🌍"));
        assert_eq!(unicode_str.get_type(), MistQLValueType::String);
        assert!(unicode_str.truthy());
        assert_eq!(unicode_str.to_string_serialize(), "\"Hello 世界 🌍\"");
    }

    #[test]
    fn test_empty_structures() {
        let empty_str = ValueView::from(json!(""));
        let empty_arr = ValueView::from(json!([]));
        let empty_obj = ValueView::from(json!({}));

        assert!(!empty_str.truthy());
        assert!(!empty_arr.truthy());
        assert!(!empty_obj.truthy());
    }

    #[test]
    fn test_nested_empty_structures() {
        let nested_empty = ValueView::from(json!(vec![json!([]), json!({}), json!("")]));

        // The array itself is truthy because it's not empty
        assert!(nested_empty.truthy());

        // But its elements are falsy
        let arr = nested_empty.as_json().unwrap().as_value().as_array().unwrap();
        assert!(!ValueView::from(&arr[0]).truthy()); // Empty array
        assert!(!ValueView::from(&arr[1]).truthy()); // Empty object
        assert!(!ValueView::from(&arr[2]).truthy()); // Empty string
    }

    #[test]
    fn test_special_string_values() {
        let newline_str = ValueView::from(json!("\n"));
        let tab_str = ValueView::from(json!("\t"));
        let space_str = ValueView::from(json!(" "));

        // All non-empty strings are truthy
        assert!(newline_str.truthy());
        assert!(tab_str.truthy());
        assert!(space_str.truthy());
    }

    #[test]
    fn test_number_edge_cases() {
        let zero = ValueView::from(json!(0.0));
        let negative_zero = ValueView::from(json!(-0.0));

        assert!(!zero.truthy());
        assert!(!negative_zero.truthy());

        // -0.0 should equal 0.0
        assert_eq!(zero, negative_zero);
    }
}
