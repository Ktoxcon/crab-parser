use crate::token::Token;

#[derive(Debug)]
pub struct LexerResult {
    tokens: Vec<Token>,
}

impl LexerResult {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}
