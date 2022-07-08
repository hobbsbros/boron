//! Provides a parselet for binary operations.


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


/// Provides a prefix parselet for binary operations.
pub struct BinOpParselet;

impl InfixParselet for BinOpParselet {
    /// Parses a binary operation into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        let right = match parser.parse(token.get_type().into(), tokenizer) {
            Some(r) => r,
            None => throw(Error::CouldNotParse (token.get_value())),
        };

        match token.get_type() {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide 
            | TokenType::Greater
            | TokenType::Less
            | TokenType::Equal
            | TokenType::GreaterEqual
            | TokenType::LessEqual
            => {
                // No problem!
            },
            _ => throw(Error::InvalidOperator (token.get_value())), // Error: invalid binary operation
        };

        Expression::BinOp {
            left: Box::new(left),
            op: token.get_type(),
            right: Box::new(right),
        }
    }
}