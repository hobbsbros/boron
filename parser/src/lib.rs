//! A simple Pratt-based parser for the Boron compiler.


pub mod infix;
pub mod prefix;

pub mod identifier_parselet;
pub mod datatype_parselet;
pub mod declaration_parselet;


use std::collections::HashMap;

use infix::InfixParselet;
use prefix::PrefixParselet;

use identifier_parselet::IdentifierParselet;

pub use tokenizer::{
    Token,
    TokenType,
};

use tokenizer::{
    Tokenizer,
};


/// Defines possible expressions in Boron.
pub enum Expression {
    // 32-bit integer
    Int (i32),
    // 32-bit floating-point
    Float (f32),
    // Boolean
    Bool (bool),
    // Variable or function name
    Identifier (String),
    // Datatype keyword
    Type (String),
    // Variable declaration
    Declaration {
        datatype: String,
        identifier: String,
    },
    // Variable assignment
    Assignment {
        datatype: String,
        identifier: String,
        value: Box<Expression>,
    },
    // Function call
    FnCall {
        name: String,
        args: Vec<Expression>,
    }
}


/// Creates an abstraction over parsing behaviors.
pub struct Parser {
    prefix_parselets: HashMap<TokenType, Box<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenType, Box<dyn InfixParselet>>,
}

impl Parser {
    /// Constructs a new parser from an instance of `Tokenizer`.
    pub fn new() -> Self {
        let mut prefix_parselets: HashMap<TokenType, Box<dyn PrefixParselet>> = HashMap::new();
        let mut infix_parselets: HashMap<TokenType, Box<dyn InfixParselet>> = HashMap::new();

        // Declarative grammar begins here.
        prefix_parselets.insert(TokenType::Identifier, Box::new(IdentifierParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
        }
    }

    /// Parses the token stream and returns an expression, if possible.
    pub fn parse(&self, tokenizer: &mut Tokenizer) -> Option<Expression> {
        // Get the next token from the token stream.
        let token = match tokenizer.next() {
            Some(t) => t,
            None => return None,
        };

        // Get the proper prefix parselet from the type of the given token.
        let parselet: &Box<dyn PrefixParselet> = match self.prefix_parselets.get(&token.get_type()) {
            Some(p) => p,
            None => return None,
        };

        let expr = parselet.parse(self, token);

        Some(expr)
    }
}