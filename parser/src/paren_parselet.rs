//! Provides a parselet for parenthetical expressions.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    prefix::PrefixParselet,
};


/// Provides a prefix parselet for parenthetical expressions.
pub struct ParenParselet;

impl PrefixParselet for ParenParselet {
    /// Parses a parenthetical into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.check(TokenType::OpenParen) {
            let expr = match parser.parse(tokenizer) {
                Some(e) => e,
                None => todo!(),
            };
            
            let token = tokenizer.peek();

            match token {
                Some(t) => {
                    if t.get_type() == TokenType::CloseParen {
                        // Consume the token
                        // It's ok to use `unwrap` here because we just checked that
                        // the tokenizer has at least one more token to yield
                        tokenizer.next().unwrap();
                    } else {
                        todo!();
                    }
                },
                None => todo!(),
            }

            expr
        } else {
            todo!();
        }
    }
}