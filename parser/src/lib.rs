//! A simple Pratt-based parser for the Boron compiler.


pub mod infix;
pub mod prefix;

pub mod identifier_parselet;
pub mod datatype_parselet;
pub mod declaration_parselet;
pub mod assignment_parselet;
pub mod literal_parselet;


use std::collections::HashMap;

use infix::InfixParselet;
use prefix::PrefixParselet;

use identifier_parselet::IdentifierParselet;
use datatype_parselet::DatatypeParselet;
use declaration_parselet::DeclarationParselet;
use assignment_parselet::AssignmentParselet;
use literal_parselet::LiteralParselet;

pub use tokenizer::{
    Token,
    TokenType,
    Tokenizer,
};


/// Defines possible expressions in Boron.
#[derive(Debug)]
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
        prefix_parselets.insert(TokenType::Type, Box::new(DatatypeParselet {}));
        prefix_parselets.insert(TokenType::Identifier, Box::new(IdentifierParselet {}));
        prefix_parselets.insert(TokenType::Int, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::Float, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::Bool, Box::new(LiteralParselet {}));
        infix_parselets.insert(TokenType::Identifier, Box::new(DeclarationParselet {}));
        infix_parselets.insert(TokenType::Assignment, Box::new(AssignmentParselet {}));

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

        dbg!(&token);

        // Get the proper prefix parselet from the type of the given token.
        let parselet: &Box<dyn PrefixParselet> = match self.prefix_parselets.get(&token.get_type()) {
            Some(p) => p,
            None => return None,
        };

        let left = parselet.parse(self, tokenizer, token);

        dbg!(&left);

        let token = match tokenizer.next() {
            Some(t) => t,
            None => return Some(left),
        };

        dbg!(&token);

        // Get the proper infix parselet from the type of the given token,
        // or return the current expression.
        let parselet: &Box<dyn InfixParselet> = match self.infix_parselets.get(&token.get_type()) {
            Some(p) => p,
            None => return Some(left),
        };

        Some(parselet.parse(self, tokenizer, left, token))
    }
}