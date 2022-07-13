//! Provides a parselet for identifiers.

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


/// Provides a prefix parselet for identifiers.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    /// Parses an identifier into an expression.
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if !token.check(TokenType::Identifier) {
            throw(Error::ExpectedIdentifier (token.get_value()));
        }
        Expression::Identifier (token.get_value())
    }
}