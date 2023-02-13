use crate::lexer_result::LexerResult;
use crate::token::Token;

#[derive(Debug, Default)]
pub struct Lexer {
    current_position: usize,
    current_line: usize,
    file_content: String,
}

impl Lexer {
    pub fn new(file_content: String) -> Self {
        Self {
            current_position: 0,
            current_line: 0,
            file_content,
        }
    }

    pub fn scan_tokens(self) -> Vec<Token> {
        let mut scanned_tokens = Vec::new();

        scanned_tokens
    }

    pub fn lex(self) -> LexerResult {
        let tokens = self.scan_tokens();

        LexerResult::new(tokens)
    }
}
