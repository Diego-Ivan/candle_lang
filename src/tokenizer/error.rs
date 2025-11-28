use std::fmt::Display;

#[derive(Debug)]
pub enum TokenizerError {
    UnknownCharacter {
        character: char,
        byte: u8,
        line: usize,
        column: usize,
    },
    UnterminatedString {
        line: usize,
        column: usize,
    },
    InvalidNumber {
        value: String,
        line: usize,
        column: usize,
        reason: String,
    },
    IoError(std::io::Error),
}

pub type TokenizerResult<T> = Result<T, TokenizerError>;

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCharacter { character, byte, line, column } => {
                if character.is_ascii_graphic() {
                    write!(f, "Unknown character '{}' (byte {}) at line {}, column {}", 
                           character, byte, line, column)
                } else {
                    write!(f, "Unknown character (byte {}) at line {}, column {}", 
                           byte, line, column)
                }
            }
            Self::UnterminatedString { line, column } => {
                write!(f, "Unterminated string starting at line {}, column {}", line, column)
            }
            Self::InvalidNumber { value, line, column, reason } => {
                write!(f, "Invalid number '{}' at line {}, column {}: {}", 
                       value, line, column, reason)
            }
            Self::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for TokenizerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TokenizerError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
