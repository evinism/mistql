use crate::error::MistQLError;
use crate::parse::Expression;

impl Expression {
    pub fn evaluate(&self, _context: &serde_json::Value) -> Result<serde_json::Value, MistQLError> {
        Err(MistQLError::UnimplementedEvaluation(
            "no evaluate yet".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {}
