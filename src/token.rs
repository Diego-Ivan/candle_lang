#[derive(Debug, Clone)]
pub enum TokenType {
    Semicolon,
    Comma,
    Colon,
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
    RightBrace,
    LeftBrace,
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

    // --- Getter Methods ---

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}
