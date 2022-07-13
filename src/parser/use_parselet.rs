//! Provides a parselet for use statements.


use crate::parser::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    prefix::PrefixParselet,
};

use crate::error::{
    throw,
    Error,
};


/// Provides a prefix parselet for use statements.
pub struct UseParselet;

impl PrefixParselet for UseParselet {
    /// Parses a use statement into an expression.
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let next = match tokenizer.next() {
            Some(n) => n,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        let identifier = match next.get_type() {
            TokenType::Identifier => next.get_value(),
            _ => throw(Error::ExpectedIdentifier (next.get_value())),
        };
        
        Expression::Use (identifier.replace(".", "/"))
    }
}