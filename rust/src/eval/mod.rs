use crate::parse::Node;
use crate::{Error, Result};

impl Node {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::At => Ok(context.clone()),
            Self::Bool(val) => Ok(val.clone().into()),
            Self::Int(val) => Ok(val.clone().into()),
            Self::Float(val) => Ok(val.clone().into()),
        }
    }
}
