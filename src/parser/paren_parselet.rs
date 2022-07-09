//! Provides a parselet for parenthetical expressions.


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


/// Provides a prefix parselet for parenthetical expressions.
pub struct ParenParselet;

impl PrefixParselet for ParenParselet {
    /// Parses a parenthetical into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.check(TokenType::OpenParen) {
            let expr = match parser.parse(token.get_type().into(), tokenizer) {
                Some(e) => e,
                None => throw(Error::CouldNotParse (token.get_value())),
            };
                        
            let next = tokenizer.peek();

            match next {
                Some(t) => {
                    if t.get_type() == TokenType::CloseParen {
                        // Consume the token
                        // It's ok to use `unwrap` here because we just checked that
                        // the tokenizer has at least one more token to yield
                        tokenizer.next().unwrap();
                    } else {
                        throw(Error::ExpectedCloseParen (t.get_value()));
                    }
                },
                None => throw(Error::UnexpectedEof (token.get_value())),
            }

            expr
        } else {
            throw(Error::ExpectedOpenParen (token.get_value()));
        }
    }
}