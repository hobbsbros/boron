//! Provides a parselet for binary operations.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    infix::InfixParselet,
};


/// Provides a prefix parselet for binary operations.
pub struct BinOpParselet;

impl InfixParselet for BinOpParselet {
    /// Parses a binary operation into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        let right = match parser.parse(tokenizer) {
            Some(r) => r,
            None => todo!(),
        };

        match token.get_type() {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide 
            | TokenType::Greater
            | TokenType::Less
            | TokenType::Equal
            => {
                // No problem!
            },
            _ => todo!(), // Error: invalid binary operation
        };

        Expression::BinOp {
            left: Box::new(left),
            op: token.get_type(),
            right: Box::new(right),
        }
    }
}