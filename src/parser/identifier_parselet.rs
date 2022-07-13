//! Provides a parselet for identifiers.


use std::collections::HashMap;

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
        if !(token.check(TokenType::Identifier) || token.check(TokenType::Ref)) {
            throw(Error::ExpectedIdentifier (token.get_value()));
        }

        let next = match tokenizer.peek() {
            Some(n) => n,
            None => return Expression::Identifier (token.get_value()),
        };

        // Look ahead two tokens
        // We're expecting this to be an identifier for a struct initialization
        let two = match tokenizer.look_ahead(1) {
            Some(t) => t,
            None => return Expression::Identifier (token.get_value()),
        };

        // Look ahead three tokens
        // We're expecting this to be an opening brace for a struct initialization
        let three = match tokenizer.look_ahead(2) {
            Some(t) => t,
            None => return Expression::Identifier (token.get_value()),
        };

        // If the next token is an identifier and the token after that is assignment, return a struct initialization
        // Otherwise, return the identifier
        if next.get_type() == TokenType::Identifier
            && two.get_type() == TokenType::Assignment
            && three.get_type() == TokenType::OpenBrace
        {
            // It's ok to use `unwrap` here because we just checked that there is
            // at least three more tokens in the tokenizer.
            let instance_name = tokenizer.next().unwrap().get_value();
            let mut variables: HashMap<String, Expression> = HashMap::new();
            // Consume the assignment operator and opening brace
            tokenizer.next().unwrap();
            tokenizer.next().unwrap();

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

                variables.insert(varname, expr);
            }

            Expression::StructInit {
                identifier: token.get_value(),
                name: instance_name,
                variables,
            }
        } else {
            match token.get_value().as_bytes()[0] as char {
                '&' => Expression::Reference (token.get_value()),
                _ => Expression::Identifier (token.get_value()),
            }
        }
    }
}