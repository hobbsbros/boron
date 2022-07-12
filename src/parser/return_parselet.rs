//! Provides a parselet for return statements.


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


/// Provides a prefix parselet for return statements.
pub struct ReturnParselet;

impl PrefixParselet for ReturnParselet {
    /// Parses a return statement into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let expr = match parser.parse(token.get_type().into(), tokenizer) {
            Some(r) => r,
            None => throw(Error::CouldNotParse (token.get_value())),
        };

        Expression::Return (Box::new(expr))
    }
}