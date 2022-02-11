use crate::parse::Query;
use crate::{Error, Result};

impl Query {
    pub fn evaluate(&self, _context: &serde_json::Value) -> Result<serde_json::Value> {
        Err(Error::eval("unimplemented".to_string()))
    }
}
