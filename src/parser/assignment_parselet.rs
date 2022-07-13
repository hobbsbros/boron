//! Provides a parselet for assignments.


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


/// Provides a prefix parselet for assignments.
pub struct AssignmentParselet;

impl PrefixParselet for AssignmentParselet {
    /// Parses an assignment into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.get_type() != TokenType::Let {
            throw(Error::CouldNotParse (token.get_value()));
        }
        // This is an assignment

        // Get the datatype keyword
        let next = match tokenizer.next() {
            Some(n) => n,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        let d = match next.get_type() {
            TokenType::Type | TokenType::Identifier => next.get_value(),
            _ => throw(Error::ExpectedDatatypeKeyword (next.get_value())),
        };

        // Get the identifier name
        let next = match tokenizer.next() {
            Some(n) => n,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        let id = match next.get_type() {
            TokenType::Identifier => next.get_value(),
            _ => throw(Error::ExpectedIdentifier (next.get_value())),
        };

        // Consume the assignment token
        let next = match tokenizer.next() {
            Some(n) => n,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        let _ = match next.get_type() {
            TokenType::Assignment => next.get_value(),
            _ => throw(Error::ExpectedAssignment (next.get_value())),
        };

        // Evaluate the right hand side of the assignment
        let right_hand_side: Expression = match parser.parse(next.get_type().into(), tokenizer) {
            Some(r) => r,
            None => throw(Error::CouldNotParse (id)),
        };

        // Place the right hand side into an instance of `Expression`
        Expression::Assignment {
            datatype: d,
            identifier: id,
            value: Box::new(right_hand_side),
        }
    }
}