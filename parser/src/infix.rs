//! Provides an interface for infix parselets.


use crate::{
    Parser,
    Expression,
    Token,
};


/// Defines shared behavior for infix parselets.
pub trait InfixParselet {
    fn parse(&self, parser: &Parser, left: Expression, token: Token) -> Expression;
}