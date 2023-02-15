use crate::symbols::JsonSymbol;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: JsonSymbol,
    pub token_index: usize,
}
