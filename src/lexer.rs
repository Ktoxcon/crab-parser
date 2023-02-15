use crate::lexer_result::LexerResult;
use crate::symbols::JsonSymbol;
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

    pub fn build_token_struct(&self, token_type: JsonSymbol) -> Token {
        Token {
            token_type,
            token_index: self.current_position,
        }
    }

    pub fn is_not_at_end(&mut self) -> bool {
        self.current_position != self.file_content.len()
    }

    pub fn get_current_char(&mut self) -> char {
        let current_char = self
            .file_content
            .chars()
            .nth(self.current_position)
            .unwrap();

        current_char
    }

    pub fn pick_next(&mut self) -> char {
        let next_char: char = self
            .file_content
            .chars()
            .nth(self.current_position + 1)
            .unwrap();

        next_char
    }

    pub fn advance(&mut self) {
        self.current_position += 1
    }

    pub fn get_string_literal(&mut self) -> String {
        let mut string_literal = String::from("");

        while self.get_current_char() != '"' {
            string_literal.push(self.get_current_char());
            self.advance();
        }

        string_literal
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut scanned_tokens = Vec::new();

        while self.is_not_at_end() {
            match self.get_current_char() {
                ':' => {
                    let new_colon_token = self.build_token_struct(JsonSymbol::Colon);
                    scanned_tokens.push(new_colon_token);
                    self.advance()
                }
                '"' => {
                    let new_quote_token = self.build_token_struct(JsonSymbol::DoubleQuote);
                    scanned_tokens.push(new_quote_token);
                    self.advance()
                }
                '{' => {
                    let new_lcurlyb_token = self.build_token_struct(JsonSymbol::LeftCurlyBracket);
                    scanned_tokens.push(new_lcurlyb_token);
                    self.advance()
                }
                '}' => {
                    let new_rcurlyb_token = self.build_token_struct(JsonSymbol::RightCurlyBracket);
                    scanned_tokens.push(new_rcurlyb_token);
                    self.advance()
                }
                '[' => {
                    let new_lsquareb_token = self.build_token_struct(JsonSymbol::LeftSquareBracket);
                    scanned_tokens.push(new_lsquareb_token);
                    self.advance()
                }
                ']' => {
                    let new_rsquareb_token =
                        self.build_token_struct(JsonSymbol::RightSquareBracket);
                    scanned_tokens.push(new_rsquareb_token);
                    self.advance()
                }
                ',' => {
                    let new_coma_token = self.build_token_struct(JsonSymbol::Coma);
                    scanned_tokens.push(new_coma_token);
                    self.advance()
                }
                '\n' => {
                    self.current_line += 1;
                    self.advance();
                }
                '\t' | '\r' | ' ' => self.advance(),
                _ => {
                    let literal = self.get_string_literal();
                    let new_literal_token = self.build_token_struct(JsonSymbol::Literal(literal));
                    scanned_tokens.push(new_literal_token);
                    self.advance()
                }
            }
        }

        scanned_tokens
    }

    pub fn lex(&mut self) -> LexerResult {
        let tokens = self.scan_tokens();

        LexerResult::new(tokens)
    }
}
