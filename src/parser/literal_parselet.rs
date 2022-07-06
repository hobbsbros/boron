//! Provides a parselet for literals.


use crate::parser::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    prefix::PrefixParselet,
};


/// Provides a prefix parselet for literals.
pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    /// Parses a literal into an expression.
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
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
                    _ => todo!(),
                };
                Expression::Bool (bln)
            },
            _ => todo!(),
        }
    }
}