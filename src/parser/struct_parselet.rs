//! Provides a parselet for structure definitions.


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


/// Provides a prefix parselet for structure definitions.
pub struct StructParselet;

impl PrefixParselet for StructParselet {
    /// Parses a structure definition into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.get_type() != TokenType::Struct {
            throw(Error::CouldNotParse (token.get_value()));
        }

        let name = match tokenizer.peek() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        match name.get_type() {
            TokenType::Identifier => tokenizer.next(),
            _ => throw(Error::ExpectedIdentifier (name.get_value())),
        };

        let next = match tokenizer.peek() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        match next.get_type() {
            // This is a struct declaration.
            TokenType::OpenBrace => {
                tokenizer.next();
                let mut body: HashMap<String, String> = HashMap::new();

                // Until we find a closing curly brace, parse each variable
                while let Some(t) = tokenizer.peek() {
                    if t.get_type() == TokenType::CloseBrace {
                        tokenizer.next();
                        break;
                    }
        
                    let expr: Expression = match parser.parse(t.get_type().into(), tokenizer) {
                        Some(e) => e,
                        None => throw(Error::CouldNotParse (t.get_value())),
                    };
                    if let Expression::Declaration {
                        datatype: d,
                        identifier: i,
                    } = expr {
                        body.insert(i, d);
                    } else {
                        throw(Error::CouldNotParse (t.get_value()));
                    }
                }
        
                Expression::Struct {
                    identifier: name.get_value(),
                    variables: body,
                }
            },
            // If we see an identifier, this is a struct initialization, not declaration.
            TokenType::Identifier => {
                // It's ok to use `unwrap` here because we just checked that there is
                // at least one more token in the tokenizer.
                let instance_name = tokenizer.next().unwrap().get_value();
                let mut variables: Vec<(String, Expression)> = Vec::new();

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
                    identifier: name.get_value(),
                    name: instance_name,
                    variables,
                }
            },
            _ => throw(Error::ExpectedOpenBrace (next.get_value())),
        }
    }
}