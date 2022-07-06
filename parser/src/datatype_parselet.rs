//! Provides a parselet for datatype keywords.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    prefix::PrefixParselet,
};


/// Provides a prefix parselet for datatype keywords.
pub struct DatatypeParselet;

impl PrefixParselet for DatatypeParselet {
    /// Parses a datatype keyword into an expression.
    fn parse(&self, _parser: &Parser, token: Token) -> Expression {
        if token.get_type() == TokenType::Type {
            Expression::Type (token.get_value())
        } else {
            todo!()
        }
    }
}