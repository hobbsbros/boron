//! Provides a parselet for while loops.


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


/// Provides a prefix parselet for while loops.
pub struct WhileParselet;

impl PrefixParselet for WhileParselet {
    /// Parses a while loop into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let condition: Expression = match parser.parse(tokenizer) {
            Some(c) => c,
            None => throw(Error::CouldNotParse (token.get_value())),
        };

        let next = match tokenizer.peek() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };

        match next.get_type() {
            TokenType::OpenBrace => tokenizer.next(),
            _ => throw(Error::ExpectedOpenBrace (next.get_value())),
        };

        let mut body: Vec<Expression> = Vec::new();

        // Until we find a closing curly brace, parse each expression in the loop
        while let Some(t) = tokenizer.peek() {
            if t.get_type() == TokenType::CloseBrace {
                tokenizer.next();
                break;
            }

            let expr: Expression = match parser.parse(tokenizer) {
                Some(e) => e,
                None => throw(Error::CouldNotParse (t.get_value())),
            };
            body.push(expr);
        }

        Expression::While {
            condition: Box::new(condition),
            body,
        }
    }
}