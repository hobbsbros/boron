//! Provides a parselet for identifiers.


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


/// Provides a prefix parselet for identifiers.
pub struct IdentifierParselet;

impl PrefixParselet for IdentifierParselet {
    /// Parses an identifier into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if !token.check(TokenType::Identifier) {
            throw(Error::ExpectedIdentifier (token.get_value()));
        }

        let next = match tokenizer.peek() {
            Some(n) => n,
            None => return Expression::Identifier (token.get_value()),
        };

        // Look ahead two tokens
        let two = match tokenizer.look_ahead(1) {
            Some(t) => t,
            None => return Expression::Identifier (token.get_value()),
        };

        // If the next token is an identifier and the token after that is assignment, return a struct initialization
        // Otherwise, return the identifier
        if next.get_type() == TokenType::Identifier && two.get_type() == TokenType::Assignment {
            // It's ok to use `unwrap` here because we just checked that there is
            // at least one more token in the tokenizer.
            let instance_name = tokenizer.next().unwrap().get_value();
            let mut variables: Vec<(String, Expression)> = Vec::new();

            match tokenizer.next() {
                Some(t) => if t.get_type() != TokenType::Assignment {
                    throw(Error::ExpectedAssignment (t.get_value()));
                },
                None => throw(Error::UnexpectedEof (instance_name)),
            };
            match tokenizer.next() {
                Some(t) => if t.get_type() != TokenType::OpenBrace {
                    throw(Error::ExpectedOpenBrace (t.get_value()));
                },
                None => throw(Error::UnexpectedEof (instance_name)),
            };

            // Until we find a closing curly brace, parse each variable
            while let Some(t) = tokenizer.peek() {
                if t.get_type() == TokenType::CloseBrace {
                    tokenizer.next();
                    break;
                }

                // It's ok to use `unwrap` here because we just checked that there is
                // at least one more token left in the tokenizer.
                let varname: String = tokenizer.next().unwrap().get_value();
                let expr: Expression = match parser.parse(t.get_type().into(), tokenizer) {
                    Some(e) => e,
                    None => {
                        dbg!(&t);
                        throw(Error::CouldNotParse (t.get_value()));
                    },
                };

                variables.push((varname, expr));
            }

            Expression::StructInit {
                identifier: token.get_value(),
                name: instance_name,
                variables,
            }
        } else {
            Expression::Identifier (token.get_value())
        }
    }
}