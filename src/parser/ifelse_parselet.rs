//! Provides a parselet for if/else statements.


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


/// Provides a prefix parselet for if/else statements.
pub struct IfElseParselet;

impl PrefixParselet for IfElseParselet {
    /// Parses an if/else statement loop into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        let condition: Expression = match parser.parse(token.get_type().into(), tokenizer) {
            Some(c) => c,
            None => throw(Error::CouldNotParse (token.get_value())),
        };

        let next = match tokenizer.peek() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };

        match next.get_type() {
            TokenType::OpenBrace => tokenizer.next(),
            _ => throw(Error::ExpectedOpenBrace (next.get_value())),
        };

        let mut body_true: Vec<Expression> = Vec::new();

        // Until we find a closing curly brace, parse each expression in the loop
        while let Some(t) = tokenizer.peek() {
            if t.get_type() == TokenType::CloseBrace {
                tokenizer.next();
                break;
            }

            let expr: Expression = match parser.parse(t.get_type().into(), tokenizer) {
                Some(e) => e,
                None => throw(Error::CouldNotParse (t.get_value())),
            };
            body_true.push(expr);
        }

        // Look for the next token (should be `else`)
        // If it does not exist, return the if statement as is
        let else_token = match tokenizer.peek() {
            Some(t) => t,
            None => return Expression::If {
                condition: Box::new(condition),
                body: body_true,
            },
        };

        // The next token should be `else`
        // If it is not, return the if statement as it is
        match else_token.get_type() {
            TokenType::Else => tokenizer.next(),
            _ => return Expression::If {
                condition: Box::new(condition),
                body: body_true,
            },
        };

        // The token after `else` should be a curly brace
        let brace = match tokenizer.peek() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (else_token.get_value()))
        };

        // If this is not a curly brace, throw an error
        match brace.get_type() {
            TokenType::OpenBrace => tokenizer.next(),
            _ => throw(Error::ExpectedOpenBrace (brace.get_value())),
        };

        let mut body_false: Vec<Expression> = Vec::new();
        
        // Until we find a closing curly brace, parse each expression in the loop
        while let Some(t) = tokenizer.peek() {
            if t.get_type() == TokenType::CloseBrace {
                tokenizer.next();
                break;
            }

            let expr: Expression = match parser.parse(t.get_type().into(), tokenizer) {
                Some(e) => e,
                None => throw(Error::CouldNotParse (t.get_value())),
            };
            body_false.push(expr);
        }

        Expression::IfElse {
            condition: Box::new(condition),
            body_true,
            body_false,
        }
    }
}