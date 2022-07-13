//! Provides a parselet for struct initializations.

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


/// Provides a prefix parselet for struct initializations.
pub struct StructInitParselet;

impl PrefixParselet for StructInitParselet {
    /// Parses a struct initialization into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        if token.check(TokenType::OpenBrace) {
            let mut variables: HashMap<String, Expression> = HashMap::new();

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
                variables,
            }
        } else {
            throw(Error::ExpectedOpenBrace (token.get_value()));
        }
    }
}