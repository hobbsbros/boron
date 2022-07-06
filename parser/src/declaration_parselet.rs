//! Provides a parselet for variable declarations.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    infix::InfixParselet,
};


/// Provides a prefix parselet for variable declarations.
pub struct DeclarationParselet;

impl InfixParselet for DeclarationParselet {
    /// Parses a variable declaration into an expression.
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        // Make sure the left token is a type keyword
        if let Expression::Type (t) = left {
            // Make sure the current token is an identifier
            if token.get_type() == TokenType::Identifier {
                Expression::Declaration {
                    datatype: t,
                    identifier: token.get_value(),
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}