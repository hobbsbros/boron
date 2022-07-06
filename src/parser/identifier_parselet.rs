//! Provides a parselet for identifiers.


use crate::parser::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    prefix::PrefixParselet,
};


/// Provides a prefix parselet for identifiers.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    /// Parses an identifier into an expression.
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.check(TokenType::Identifier) {
            Expression::Identifier (token.get_value())
        } else {
            todo!();
        }
    }
}