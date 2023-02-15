#[derive(Debug, Clone)]
pub enum JsonSymbol {
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Coma,
    DoubleQuote,
    Literal(String),
}
