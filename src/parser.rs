mod error;
mod statement;
#[cfg(test)]
mod tests;
use std::collections::HashMap;

use crate::{
    parser::error::{ParserError, ParserErrorType, ParserResult},
    token::{Token, TokenType},
};

pub use statement::{Select, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_token {
    ($parser: ident, $pattern: pat) => {{
        match $parser.peek() {
            Some(next_token) => {
                if matches!(next_token.get_type(), $pattern) {
                    $parser.advance();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }};
}

macro_rules! expect_identifier {
    ($parser: ident) => {{
        match $parser.peek() {
            Some(next_token) => match next_token.get_type() {
                TokenType::Id(id) => {
                    let id = String::from(id);
                    $parser.advance();
                    id
                }
                _ => {
                    return Err(ParserError::new(
                        $parser.peek().unwrap().clone(),
                        ParserErrorType::ExpectedId,
                    ));
                }
            },
            None => panic!(""),
        }
    }};
}
macro_rules! expect_semicolon {
    ($parser: ident) => {
        if !match_token!($parser, TokenType::Semicolon) {
            return Err(ParserError::new(
                $parser.peek().unwrap().clone(),
                ParserErrorType::ExpectedSemicolon,
            ));
        }
    };
}

macro_rules! expect_colon {
    ($parser: ident) => {
        if !match_token!($parser, TokenType::Colon) {
            return Err(ParserError::new(
                $parser.peek().unwrap().clone(),
                ParserErrorType::ExpectedColon,
            ));
        }
    };
}
macro_rules! expect_number {
    ($parser: ident) => {
        match $parser.peek() {
            Some(next_token) => match next_token.get_type() {
                TokenType::Number(num) => {
                    let num = *num;
                    $parser.advance();
                    num
                }
                _ => {
                    return Err(ParserError::new(
                        $parser.peek().unwrap().clone(),
                        ParserErrorType::ExpectedNumber,
                    ));
                }
            },
            None => panic!(""),
        }
    };
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn program(&mut self) -> ParserResult<Vec<Statement>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
            expect_semicolon!(self);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> ParserResult<Statement> {
        match self.peek().unwrap().get_type() {
            TokenType::Load => {
                self.advance();
                self.parse_load()
            }
            TokenType::Predict => {
                self.advance();
                self.parse_predict()
            }
            TokenType::Analyze => {
                self.advance();
                self.parse_analyze()
            }
            TokenType::Evaluate => {
                self.advance();
                self.parse_evaluate()
            }
            TokenType::Train => {
                self.advance();
                self.parse_train()
            }
            TokenType::Select => {
                self.advance();
                self.parse_select()
            }
            TokenType::Init => {
                self.advance();
                self.parse_init()
            }
            TokenType::Split => {
                self.advance();
                self.parse_split()
            }
            _ => Err(ParserError::new(
                self.peek().unwrap().clone(),
                ParserErrorType::InvalidStatement,
            )),
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn parse_load(&mut self) -> ParserResult<Statement> {
        let identifier = expect_identifier!(self);
        Ok(Statement::Load(String::from(identifier)))
    }

    fn parse_predict(&mut self) -> ParserResult<Statement> {
        let identifier = expect_identifier!(self);
        Ok(Statement::Predict(String::from(identifier)))
    }

    fn parse_analyze(&mut self) -> ParserResult<Statement> {
        let arg_list = self.parse_argument_list()?;
        Ok(Statement::Analyze(arg_list))
    }

    fn parse_evaluate(&mut self) -> ParserResult<Statement> {
        let arg_list = self.parse_argument_list()?;
        Ok(Statement::Evaluate(arg_list))
    }

    fn parse_argument_list(&mut self) -> ParserResult<Vec<String>> {
        let mut arguments = Vec::new();

        arguments.push(expect_identifier!(self));

        while match_token!(self, TokenType::Comma) {
            let ident = expect_identifier!(self);
            arguments.push(ident);
        }

        Ok(arguments)
    }

    fn parse_split(&mut self) -> ParserResult<Statement> {
        let dictionary = self.parse_dictionary()?;
        Ok(Statement::Split(dictionary))
    }

    fn parse_init(&mut self) -> ParserResult<Statement> {
        let dictionary = self.parse_dictionary()?;
        Ok(Statement::Init(dictionary))
    }

    fn parse_dictionary(&mut self) -> ParserResult<HashMap<String, f64>> {
        let mut result_map = HashMap::new();
        if !match_token!(self, TokenType::LeftBrace) {
            return Err(ParserError::new(
                self.peek().unwrap().clone(),
                ParserErrorType::NotADictionary,
            ));
        }

        let id = expect_identifier!(self);
        expect_colon!(self);
        let num = expect_number!(self);

        result_map.insert(id, num);

        while match_token!(self, TokenType::Comma) {
            let id = expect_identifier!(self);
            expect_colon!(self);
            let num = expect_number!(self);
            result_map.insert(id, num);
        }

        if !match_token!(self, TokenType::RightBrace) {
            return Err(ParserError::new(
                self.peek().unwrap().clone(),
                ParserErrorType::NonTerminatedDictionary,
            ));
        }

        Ok(result_map)
    }

    fn parse_train(&mut self) -> ParserResult<Statement> {
        Ok(Statement::Train)
    }

    fn parse_select(&mut self) -> ParserResult<Statement> {
        match self.advance().unwrap().get_type() {
            TokenType::Id(id) => Ok(Statement::Select(Select::Identifier(String::from(id)))),
            TokenType::From => match self.advance().unwrap().get_type() {
                TokenType::String(str) => Ok(Statement::Select(Select::From(String::from(str)))),
                _ => Err(ParserError::new(
                    self.previous().unwrap().clone(),
                    ParserErrorType::NoStringAfterFrom,
                )),
            },
            _ => Err(ParserError::new(
                self.previous().unwrap().clone(),
                ParserErrorType::InvalidSelect,
            )),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        if self.current == 0 {
            None
        } else {
            self.tokens.get(self.current - 1)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
