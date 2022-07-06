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
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.get_type() == TokenType::Type {
            // Wait to discard the token... maybe it's important
            let identifier = match tokenizer.peek() {
                Some(i) => i,
                None => todo!(),
            };

            if identifier.get_type() == TokenType::Identifier {
                // Discard the token from the stream
                tokenizer.next();
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