//! Provides a parselet for unary operations.


use crate::parser::{
    Parser,
    Expression,
    Token,
    Tokenizer,
    prefix::PrefixParselet,
};

use crate::error::{
    throw,
    Error,
};


/// Provides a prefix parselet for unary operations.
pub struct UnaryOpParselet;

impl PrefixParselet for UnaryOpParselet {
    /// Parses a unary operation into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let expr = match parser.parse(token.get_type().into(), tokenizer) {
            Some(e) => e,
            None => throw(Error::CouldNotParse (token.get_value())),
        };

        Expression::UnaryOp {
            op: token.get_type(),
            expr: Box::new(expr),
        }
    }
}