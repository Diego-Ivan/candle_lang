use std::io::BufRead;
mod error;
#[cfg(test)]
mod tests;

pub use crate::{
    token::{Token, TokenType},
    tokenizer::error::TokenizerResult,
};

pub struct Tokenizer<R> {
    _source: R,
    current_char: char,
    peeked: Option<char>,
    line: usize,
    column: usize,
    input_buffer: String,
    buffer_pos: usize,
}

impl<R> Tokenizer<R>
where
    R: BufRead,
{
    pub fn new(mut reader: R) -> Self {
        let mut input_buffer = String::new();
        // Read entire input as UTF-8 string
        let _ = std::io::Read::read_to_string(&mut reader, &mut input_buffer);

        let mut tokenizer = Self {
            _source: reader,
            current_char: '\0',
            peeked: None,
            line: 1,
            column: 0,
            input_buffer,
            buffer_pos: 0,
        };
        tokenizer.advance(); // Load first character
        tokenizer
    }

    pub fn next_token(&mut self) -> TokenizerResult<Token> {
        // Skip whitespace
        self.skip_whitespace();

        let start_line = self.line;
        let start_col = self.column;

        // DFA state transitions
        match self.current_char {
            '\0' => Ok(Token::new(TokenType::Eof, start_col, start_line)),
            'a'..='z' | 'A'..='Z' => self.read_identifier(start_col, start_line),
            '0'..='9' => self.read_number(start_col, start_line),
            '\'' => self.read_string('\'', start_col, start_line),
            '{' => {
                self.advance();
                Ok(Token::new(TokenType::LeftBrace, start_col, start_line))
            }
            '}' => {
                self.advance();
                Ok(Token::new(TokenType::RightBrace, start_col, start_line))
            }
            ';' => {
                self.advance();
                Ok(Token::new(TokenType::Semicolon, start_col, start_line))
            }
            ',' => {
                self.advance();
                Ok(Token::new(TokenType::Comma, start_col, start_line))
            }
            ':' => {
                self.advance();
                Ok(Token::new(TokenType::Colon, start_col, start_line))
            }
            _ => {
                let err_char = self.current_char;
                let err_byte = self.current_char as u8;
                self.advance(); // Skip the bad character to avoid infinite loop
                Err(error::TokenizerError::UnknownCharacter {
                    character: err_char,
                    byte: err_byte,
                    line: start_line,
                    column: start_col,
                })
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == ' '
            || self.current_char == '\t'
            || self.current_char == '\n'
            || self.current_char == '\r'
        {
            self.advance();
        }
    }

    fn advance(&mut self) {
        // Check if the character is a LINE BREAK
        if self.current_char == '\n' {
            self.line += 1;
            self.column = 1;
        } else if self.current_char != '\0' {
            self.column += 1;
        }
        self.current_char = self.read_next_char();
    }

    fn read_next_char(&mut self) -> char {
        if let Some(ch) = self.peeked.take() {
            // When returning peeked char, we must advance buffer_pos
            self.buffer_pos += ch.len_utf8();
            return ch;
        }

        self.input_buffer[self.buffer_pos..]
            .chars()
            .next()
            .inspect(|ch| {
                self.buffer_pos += ch.len_utf8();
            })
            .unwrap_or('\0')
    }

    fn peek(&mut self) -> char {
        if self.peeked.is_none() {
            self.peeked = self.input_buffer[self.buffer_pos..].chars().next();
        }

        self.peeked.unwrap_or('\0')
    }

    fn read_identifier(&mut self, start_col: usize, start_line: usize) -> TokenizerResult<Token> {
        let mut identifier = String::new();
        while self.current_char.is_alphanumeric() {
            identifier.push(self.current_char);
            self.advance();
        }

        let token_type = match identifier.as_str() {
            "LOAD" => TokenType::Load,
            "PREDICT" => TokenType::Predict,
            "TRAIN" => TokenType::Train,
            "INIT" => TokenType::Init,
            "SPLIT" => TokenType::Split,
            "ANALYZE" => TokenType::Analyze,
            "EVALUATE" => TokenType::Evaluate,
            "SELECT" => TokenType::Select,
            "FROM" => TokenType::From,
            _ => TokenType::Id(identifier),
        };

        Ok(Token::new(token_type, start_col, start_line))
    }

    fn read_number(&mut self, start_col: usize, start_line: usize) -> TokenizerResult<Token> {
        let mut number_str = String::new();
        while self.current_char.is_numeric() {
            number_str.push(self.current_char);
            self.advance();
        }

        // Check for decimal point
        if self.current_char == '.' && self.peek().is_numeric() {
            number_str.push(self.current_char);
            self.advance();
            while self.current_char.is_numeric() {
                number_str.push(self.current_char);
                self.advance();
            }
        }

        match number_str.parse::<f64>() {
            Ok(number) => Ok(Token::new(TokenType::Number(number), start_col, start_line)),
            Err(e) => Err(error::TokenizerError::InvalidNumber {
                value: number_str,
                line: start_line,
                column: start_col,
                reason: e.to_string(),
            }),
        }
    }

    fn read_string(
        &mut self,
        quote: char,
        start_col: usize,
        start_line: usize,
    ) -> TokenizerResult<Token> {
        let mut string = String::new();
        self.advance(); // Skip opening quote

        while self.current_char != quote && self.current_char != '\0' && self.current_char != '\n' {
            string.push(self.current_char);
            self.advance();
        }

        if self.current_char == quote {
            self.advance(); // Skip closing quote
            Ok(Token::new(TokenType::String(string), start_col, start_line))
        } else {
            Err(error::TokenizerError::UnterminatedString {
                line: start_line,
                column: start_col,
            })
        }
    }
}

impl<R> Iterator for Tokenizer<R>
where
    R: BufRead,
{
    type Item = TokenizerResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) => {
                if matches!(token.get_type(), TokenType::Eof) {
                    None
                } else {
                    Some(Ok(token))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}
