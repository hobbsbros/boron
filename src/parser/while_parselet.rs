//! Provides a parselet for while loops.


use crate::parser::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    prefix::PrefixParselet,
};


/// Provides a prefix parselet for while loops.
pub struct WhileParselet;

impl PrefixParselet for WhileParselet {
    /// Parses a while loop into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _token: Token) -> Expression {
        let condition: Expression = match parser.parse(tokenizer) {
            Some(c) => c,
            None => todo!(),
        };

        let next = match tokenizer.peek() {
            Some(t) => t,
            None => todo!(),
        };

        match next.get_type() {
            TokenType::OpenBrace => tokenizer.next(),
            _ => todo!(),
        };

        let mut body: Vec<Expression> = Vec::new();

        // Until we find a closing parenthesis, parse each expression in the loop
        while let Some(t) = tokenizer.peek() {
            if t.get_type() == TokenType::CloseBrace {
                break;
            }

            let expr: Expression = match parser.parse(tokenizer) {
                Some(e) => e,
                None => {
                    todo!();
                },
            };
            body.push(expr);
        }

        Expression::While {
            condition: Box::new(condition),
            body,
        }
    }
}