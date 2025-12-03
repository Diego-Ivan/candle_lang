use std::{error::Error, fmt::Display};

use crate::token::Token;

#[derive(Debug)]
pub enum ParserErrorType {
    ExpectedId,
    ExpectedColon,
    ExpectedSemicolon,
    NotADictionary,
    ExpectedNumber,
    UnexpectedEof,
    InvalidStatement,
    NonTerminatedDictionary,
    InvalidSelect,
    NoStringAfterFrom,
}

#[derive(Debug)]
pub struct ParserError {
    pub token: Token,
    pub error_type: ParserErrorType,
}

pub type ParserResult<T> = Result<T, ParserError>;

impl ParserError {
    pub fn new(token: Token, error_type: ParserErrorType) -> Self {
        Self { token, error_type }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.error_type {
            ParserErrorType::ExpectedId => {
                format!("Expected ID, but found {:?} instead", self.token.get_type())
            }
            ParserErrorType::NotADictionary => {
                format!(
                    "Expected dictionary, but found {:?} intead",
                    self.token.get_type()
                )
            }
            ParserErrorType::ExpectedNumber => {
                format!(
                    "Expected number, but found {:?} instead",
                    self.token.get_type()
                )
            }
            ParserErrorType::UnexpectedEof => String::from("Unexpected End Of File"),
            ParserErrorType::InvalidStatement => {
                format!(
                    "A statement cannot begin with the token {:?}",
                    self.token.get_type()
                )
            }
            ParserErrorType::ExpectedColon => {
                format!("Expected colon, found {:?}", self.token.get_type())
            }
            ParserErrorType::ExpectedSemicolon => {
                format!(
                    "Expected semicolon to terminate statement, found {:?}",
                    self.token.get_type()
                )
            }
            ParserErrorType::NonTerminatedDictionary => {
                format!(
                    "Dictionary has to be terminated by '}}', but found {:?}",
                    self.token.get_type()
                )
            }
            ParserErrorType::InvalidSelect => {
                format!(
                    "SELECT can only be followed by FROM or an string, but found {:?}",
                    self.token.get_type()
                )
            }
            ParserErrorType::NoStringAfterFrom => {
                format!(
                    "Expected string after FROM, but found {:?}",
                    self.token.get_type()
                )
            }
        };
        write!(
            f,
            "[line {}:{}] {message}",
            self.token.get_line(),
            self.token.get_column()
        )
    }
}

impl Error for ParserError {}
