//! Provides a parselet for literals.


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


/// Provides a prefix parselet for literals.
pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    /// Parses a literal into an expression.
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::Int => {
                // It's ok to use `unwrap` here because our tokenizer checked that this could
                // be parsed into an `i32`.
                let int: i32 = str::parse::<i32>(&token.get_value()).unwrap();
                Expression::Int (int)
            },
            TokenType::Float => {
                // It's ok to use `unwrap` here because our tokenizer checked that this could
                // be parsed into an `f32`.
                let float: f32 = str::parse::<f32>(&token.get_value()).unwrap();
                Expression::Float (float)
            },
            TokenType::Bool => {
                let bln: bool = match token.get_value().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => throw(Error::ExpectedBoolean (token.get_value())),
                };
                Expression::Bool (bln)
            },
            TokenType::SingleQuote => {
                let next = match tokenizer.next() {
                    Some(n) => n,
                    None => throw(Error::UnexpectedEof (token.get_value())),
                };
                if next.get_value().as_str().len() != 1 {
                    throw(Error::ExpectedLiteral (next.get_value()));
                }

                // It's ok to use `unwrap` here because we just checked that there is exactly
                // one character in the string.
                let chr: char = next.get_value().chars().nth(0).unwrap();
                
                let next = match tokenizer.peek() {
                    Some(n) => n,
                    None => throw(Error::UnexpectedEof (token.get_value())),
                };
                match next.get_type() {
                    TokenType::SingleQuote => tokenizer.next(),
                    _ => throw(Error::ExpectedSingleQuote (token.get_value())),
                };
                Expression::Char (chr)
            },
            _ => throw(Error::ExpectedLiteral (token.get_value())),
        }
    }
}