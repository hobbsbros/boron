//! A simple Pratt-based parser for the Boron compiler.


pub mod infix;
pub mod prefix;

pub mod identifier_parselet;
pub mod datatype_parselet;
pub mod assignment_parselet;
pub mod literal_parselet;
pub mod openparen_parselet;
pub mod binop_parselet;
pub mod paren_parselet;
pub mod while_parselet;


use std::collections::HashMap;

use infix::InfixParselet;
use prefix::PrefixParselet;

use identifier_parselet::IdentifierParselet;
use datatype_parselet::DatatypeParselet;
use assignment_parselet::AssignmentParselet;
use literal_parselet::LiteralParselet;
use openparen_parselet::OpenParenParselet;
use binop_parselet::BinOpParselet;
use paren_parselet::ParenParselet;
use while_parselet::WhileParselet;

pub use tokenizer::{
    Token,
    TokenType,
    Tokenizer,
};


/// Defines possible expressions in Boron.
#[derive(Clone, Debug)]
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
    // Binary operation
    BinOp {
        left: Box<Expression>,
        op: TokenType,
        right: Box<Expression>,
    },
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
    // Variable reassignment
    Reassignment {
        identifier: String,
        value: Box<Expression>,
    },
    // Function call
    FnCall {
        name: String,
        args: Vec<Expression>,
    },
    // While loop
    While {
        condition: Box<Expression>,
        body: Vec<Expression>,
    },
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
        prefix_parselets.insert(TokenType::While, Box::new(WhileParselet {}));
        prefix_parselets.insert(TokenType::Int, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::Float, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::Bool, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::OpenParen, Box::new(ParenParselet {}));
        infix_parselets.insert(TokenType::Assignment, Box::new(AssignmentParselet {}));
        infix_parselets.insert(TokenType::OpenParen, Box::new(OpenParenParselet {}));
        infix_parselets.insert(TokenType::Plus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Minus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Multiply, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Divide, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Greater, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Less, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Equal, Box::new(BinOpParselet {}));

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

        let left = parselet.parse(self, tokenizer, token);

        let token = match tokenizer.peek() {
            Some(t) => t,
            None => return Some(left),
        };

        // Get the proper infix parselet from the type of the given token,
        // or return the current expression.
        let parselet: &Box<dyn InfixParselet> = match self.infix_parselets.get(&token.get_type()) {
            Some(p) => p,
            None => return Some(left),
        };

        tokenizer.next();

        Some(parselet.parse(self, tokenizer, left, token))
    }

    /// Parses the program into a list of expressions.
    pub fn parse_all(&self, tokenizer: &mut Tokenizer) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while let Some(e) = self.parse(tokenizer) {
            expressions.push(e);
        }

        expressions
    }
}