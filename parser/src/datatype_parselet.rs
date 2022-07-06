//! Provides a parselet for datatype keywords.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    prefix::PrefixParselet,
};


/// Provides a prefix parselet for datatype keywords.
pub struct DatatypeParselet;

impl PrefixParselet for DatatypeParselet {
    /// Parses a datatype keyword into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.get_type() == TokenType::Type {
            let identifier = match tokenizer.next() {
                Some(i) => i,
                None => todo!(),
            };

            if identifier.get_type() == TokenType::Identifier {
                Expression::Declaration {
                    identifier: identifier.get_value(),
                    datatype: token.get_value()
                }
            } else {
                todo!();
            }
        } else {
            todo!()
        }
    }
}