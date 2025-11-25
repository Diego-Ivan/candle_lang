#[derive(Debug, Clone)]
pub enum TokenType {
    Semicolon,
    Load,
    Predict,
    Train,
    Init,
    Split,
    Analyze,
    Evaluate,
    Select,
    Id(String),
    Number(f64),
    RightParen,
    LeftParen,
    RightBracket,
    LeftBracket,
    Eof,
    From,
    String(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    column: usize,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, column: usize, line: usize) -> Token {
        Self {
            token_type,
            column,
            line,
        }
    }
}
