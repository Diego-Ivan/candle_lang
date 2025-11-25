use std::io::BufRead;
mod error;

pub use crate::{token::Token, tokenizer::error::TokenizerResult};

pub struct Tokenizer<R> {
    source: R,
}

impl<R> Tokenizer<R>
where
    R: BufRead,
{
    pub fn next_token() -> TokenizerResult<Token> {
        todo!()
    }
}

impl<R> Iterator for Tokenizer<R> {
    type Item = TokenizerResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
