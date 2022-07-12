//! Provides a parselet for function declarations.

use std::collections::HashMap;

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


/// Provides a prefix parselet for function declarations.
pub struct FnDeclarationParselet;

impl InfixParselet for FnDeclarationParselet {
    /// Parses a function declaration into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        // Get the function name
        let name = match left {
            Expression::Identifier (s) => s,
            _ => throw(Error::ExpectedIdentifier ("before function declaration".to_string())),
        };

        // Parse each argument to the function
        let mut args: HashMap<String, String> = HashMap::new();
        while let Some(t) = tokenizer.peek() {
            if t.get_type() == TokenType::FnReturnType
            || t.get_type() == TokenType::OpenBrace
            {
                tokenizer.next();
                break;
            }

            // Parse each type and variable name

            // Parse the type
            let option_argtype = match tokenizer.next() {
                Some(t) => t,
                None => throw(Error::UnexpectedEof (token.get_value())),
            };
            let argtype = match option_argtype.get_type() {
                TokenType::Type
                | TokenType::Identifier => option_argtype.get_value(),
                _ => throw(Error::ExpectedIdentifier (option_argtype.get_value())),
            };

            // Parse the variable name
            let option_arg = match tokenizer.next() {
                Some(t) => t,
                None => throw(Error::UnexpectedEof (argtype)),
            };
            let arg = match option_arg.get_type() {
                TokenType::Identifier => option_arg.get_value(),
                _ => throw(Error::ExpectedIdentifier (option_arg.get_value())),
            };

            args.insert(arg, argtype);
        }

        // Parse the return type (if there is one)
        let peek = match tokenizer.peek() {
            Some(n) => n,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        let return_type: String = match peek.get_type() {
            TokenType::Type
            | TokenType::Identifier => {
                tokenizer.next();
                peek.get_value()
            },
            TokenType::OpenBrace => {
                tokenizer.next();
                "nul".to_string()
            },
            _ => throw(Error::ExpectedDatatypeKeyword (peek.get_value())),
        };
        
        // Parse the function body
        let next = match tokenizer.peek() {
            Some(t) => t,
            None => throw(Error::UnexpectedEof (token.get_value())),
        };
        match next.get_type() {
            TokenType::OpenBrace => tokenizer.next(),
            _ => throw(Error::ExpectedOpenBrace (next.get_value())),
        };

        let mut body: Vec<Expression> = Vec::new();
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
            body.push(expr);
        }

        Expression::FnDeclaration {
            identifier: name,
            arguments: args,
            return_type,
            body,
        }
    }
}