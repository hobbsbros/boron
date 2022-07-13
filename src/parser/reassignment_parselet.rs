//! Provides a parselet for reassignments.


use crate::parser::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    infix::InfixParselet,
};

use crate::error::{
    throw,
    Error,
};


/// Provides a prefix parselet for reassignments.
pub struct ReassignmentParselet;

impl InfixParselet for ReassignmentParselet {
    /// Parses an reassignment into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        if token.get_type() != TokenType::Assignment {
            throw(Error::CouldNotParse (token.get_value()));
        }

        if let Expression::Identifier (id) = left {
            // This is a declaration
        
            // Evaluate the right hand side of the assignment
            let right_hand_side: Expression = match parser.parse(token.get_type().into(), tokenizer) {
                Some(r) => r,
                None => throw(Error::CouldNotParse (id)),
            };
            // Place the right hand side into an instance of `Expression`
            Expression::Reassignment {
                identifier: id,
                value: Box::new(right_hand_side),
            }
        } else {
            throw(Error::ExpectedIdentifier (token.get_value()));
        }
    }
}