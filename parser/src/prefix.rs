//! Provides an interface for prefix parselets.


use crate::{
    Parser,
    Expression,
    Token,
    Tokenizer,
};


/// Defines shared behavior for prefix parselets.
pub trait PrefixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression;
}