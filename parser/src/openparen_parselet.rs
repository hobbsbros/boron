//! Provides a parselet for function calls.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    infix::InfixParselet,
};


/// Provides a prefix parselet for function calls.
pub struct OpenParenParselet;

impl InfixParselet for OpenParenParselet {
    /// Parses a function call into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, _token: Token) -> Expression {
        if let Expression::Identifier (i) = left {
            // `i` is the name of the function being called
            let fn_name: String = i;

            let mut args: Vec<Expression> = Vec::new();

            // Until we find a closing parenthesis, parse each expression
            while let Some(t) = tokenizer.peek() {
                if t.get_type() == TokenType::CloseParen {
                    break;
                }

                let expr: Expression = match parser.parse(tokenizer) {
                    Some(e) => e,
                    None => {
                        dbg!(&tokenizer.peek());
                        todo!();
                    },
                };
                args.push(expr);
            }

            Expression::FnCall {
                name: fn_name,
                args,
            }
        } else {
            Expression::None
        }
    }
}