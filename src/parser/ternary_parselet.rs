//! Provides a parselet for the ternary conditional operator.


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


/// Provides a prefix parselet for the ternary conditional.
pub struct TernaryParselet;

impl InfixParselet for TernaryParselet {
    /// Parses a ternary conditional into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        let if_expr: Expression = match parser.parse(token.get_type().into(), tokenizer) {
            Some(e) => e,
            None => {
                throw(Error::CouldNotParse (token.get_value()));
            },
        };

        let ternary_else = match tokenizer.next() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };

        if ternary_else.get_type() != TokenType::TernaryElse {
            throw(Error::CouldNotParse (ternary_else.get_value()));
        }

        let else_expr: Expression = match parser.parse(ternary_else.get_type().into(), tokenizer) {
            Some(e) => e,
            None => {
                throw(Error::CouldNotParse (token.get_value()));
            },
        };

        Expression::TernaryIfElse {
            condition: Box::new(left),
            body_true: Box::new(if_expr),
            body_false: Box::new(else_expr),
        }
    }
}