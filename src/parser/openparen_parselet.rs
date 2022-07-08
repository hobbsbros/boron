//! Provides a parselet for function calls.


use crate::parser::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    infix::InfixParselet,
};

use crate::error::{
    throw,
    Error,
};


/// Provides a prefix parselet for function calls.
pub struct OpenParenParselet;

impl InfixParselet for OpenParenParselet {
    /// Parses a function call into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        if let Expression::Identifier (i) = left {
            // `i` is the name of the function being called
            let fn_name: String = i;

            let mut args: Vec<Expression> = Vec::new();

            // Until we find a closing parenthesis, parse each expression
            while let Some(t) = tokenizer.peek() {
                if t.get_type() == TokenType::CloseParen {
                    tokenizer.next();
                    break;
                }

                let expr: Expression = match parser.parse(token.get_type().into(), tokenizer) {
                    Some(e) => e,
                    None => {
                        throw(Error::CouldNotParse (token.get_value()));
                    },
                };
                args.push(expr);
            }

            Expression::FnCall {
                name: fn_name,
                args,
            }
        } else {
            throw(Error::ExpectedIdentifier (token.get_value()));
        }
    }
}