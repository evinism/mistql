use crate::error::{Error, Result};
use crate::parse::Expression;

impl Expression {
    pub fn evaluate(&self, _context: &serde_json::Value) -> Result<serde_json::Value> {
        Err(Error::unimplemented_evaluation("no parse yet".to_string()))
    }
}

#[cfg(test)]
mod tests {}
