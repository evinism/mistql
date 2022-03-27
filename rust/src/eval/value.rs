use std::convert::TryFrom;

use crate::Error;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
}

impl TryFrom<serde_json::Value> for Value {
    type Error = Error;

    fn try_from(val: serde_json::Value) -> Result<Self, Self::Error> {
        match val {
            serde_json::Value::Null => Ok(Value::Null),
            _ => Err(Error::unimplemented(format!("json -> value {:?}", val))),
        }
    }
}

impl TryFrom<Value> for serde_json::Value {
    type Error = Error;

    fn try_from(val: Value) -> Result<Self, Self::Error> {
        match val {
            Value::Null => Ok(serde_json::Value::Null),
            _ => Err(Error::unimplemented(format!("value -> json {:?}", val))),
        }
    }
}
