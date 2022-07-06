//! Provides an interface for prefix parselets.


use crate::{
    Parser,
    Expression,
    Token,
};


/// Defines shared behavior for prefix parselets.
pub trait PrefixParselet {
    fn parse(&self, parser: &Parser, token: Token) -> Expression;
}