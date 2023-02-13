use crate::types::JsonValue;

#[derive(Debug, Clone)]
pub struct Token {
    children: Option<JsonValue>,
    token_type: JsonValue,
}
