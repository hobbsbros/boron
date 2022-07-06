//! Provides an interface for infix parselets.


use crate::parser::{
    Parser,
    Expression,
    Token,
    Tokenizer,
};


/// Defines shared behavior for infix parselets.
pub trait InfixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression;
}