use std::fmt::Display;

#[derive(Debug)]
pub enum TokenizerError {
    UnknownCharacter(u8),
}

pub type TokenizerResult<T> = Result<T, TokenizerError>;

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter(c) => write!(f, "Unknown byte: {c}"),
        }
    }
}

impl std::error::Error for TokenizerError {}
