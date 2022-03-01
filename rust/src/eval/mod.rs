use crate::parse::Node;
use crate::{Error, Result};

impl Node {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::At => Ok(context.clone()),
            Self::Value(val) => Ok(val.clone().into()),
            Self::Array(arr) => Ok(arr
                .iter()
                .map(|elt| elt.evaluate(context))
                .collect::<Result<Vec<serde_json::Value>>>()?
                .into()),
        }
    }
}
