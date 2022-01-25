use crate::error::Result;
use crate::parse::Expression;

impl Expression {
    pub fn evaluate(&self, context: &serde_json::Value) -> Result<serde_json::Value> {
        match self {
            Self::At => Ok(context.clone()),
        }
    }
}

#[cfg(test)]
mod tests {}
