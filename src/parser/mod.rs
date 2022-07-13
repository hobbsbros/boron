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
pub mod unaryop_parselet;
pub mod ifelse_parselet;
pub mod ternary_parselet;
pub mod struct_parselet;
pub mod fndeclaration_parselet;
pub mod return_parselet;


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
use unaryop_parselet::UnaryOpParselet;
use ifelse_parselet::IfElseParselet;
use ternary_parselet::TernaryParselet;
use struct_parselet::StructParselet;
use fndeclaration_parselet::FnDeclarationParselet;
use return_parselet::ReturnParselet;

pub use crate::tokenizer::{
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
    // Character
    Char (char),
    // Variable or function name
    Identifier (String),
    // Reference
    Reference (String),
    // Datatype keyword
    Type (String),
    // Unary operation
    UnaryOp {
        op: TokenType,
        expr: Box<Expression>,
    },
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
    // Struct declaration
    Struct {
        identifier: String,
        variables: HashMap<String, String>,
    },
    // Struct initialization
    StructInit {
        identifier: String,
        name: String,
        variables: HashMap<String, Expression>,
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
    // If statement
    If {
        condition: Box<Expression>,
        body: Vec<Expression>,
    },
    // If/else statement
    IfElse {
        condition: Box<Expression>,
        body_true: Vec<Expression>,
        body_false: Vec<Expression>,
    },
    // Ternary if/else statement
    TernaryIfElse {
        condition: Box<Expression>,
        body_true: Box<Expression>,
        body_false: Box<Expression>,
    },
    // Function declaration
    FnDeclaration {
        identifier: String,
        arguments: HashMap<String, String>,
        return_type: String,
        body: Vec<Expression>,
    },
    // Return statement
    Return (Box<Expression>),
}


/// Converts a token type into a precedence value.
impl From<TokenType> for u8 {
    fn from(t: TokenType) -> u8 {
        match t {
            TokenType::Assignment => 1,
            TokenType::FnDeclaration => 1,
            TokenType::While => 1,
            TokenType::Plus => 3,
            TokenType::Minus => 3,
            TokenType::Multiply => 4,
            TokenType::Divide => 4,
            TokenType::OpenParen => 5,
            TokenType::Equal => 7,
            TokenType::Greater => 7,
            TokenType::GreaterEqual => 7,
            TokenType::Less => 7,
            TokenType::LessEqual => 7,
            TokenType::TernaryIf => 8,
            TokenType::TernaryElse => 8,
            _ => 0,
        }
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
        prefix_parselets.insert(TokenType::Ref, Box::new(IdentifierParselet {}));
        prefix_parselets.insert(TokenType::While, Box::new(WhileParselet {}));
        prefix_parselets.insert(TokenType::If, Box::new(IfElseParselet {}));
        prefix_parselets.insert(TokenType::Int, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::Float, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::Bool, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::SingleQuote, Box::new(LiteralParselet {}));
        prefix_parselets.insert(TokenType::OpenParen, Box::new(ParenParselet {}));
        prefix_parselets.insert(TokenType::Minus, Box::new(UnaryOpParselet {}));
        prefix_parselets.insert(TokenType::Not, Box::new(UnaryOpParselet {}));
        prefix_parselets.insert(TokenType::Struct, Box::new(StructParselet {}));
        prefix_parselets.insert(TokenType::Return, Box::new(ReturnParselet {}));
        infix_parselets.insert(TokenType::Assignment, Box::new(AssignmentParselet {}));
        infix_parselets.insert(TokenType::OpenParen, Box::new(OpenParenParselet {}));
        infix_parselets.insert(TokenType::Plus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Minus, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Multiply, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Divide, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Greater, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Less, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::GreaterEqual, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::LessEqual, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::Equal, Box::new(BinOpParselet {}));
        infix_parselets.insert(TokenType::TernaryIf, Box::new(TernaryParselet {}));
        infix_parselets.insert(TokenType::FnDeclaration, Box::new(FnDeclarationParselet {}));

        Self {
            prefix_parselets,
            infix_parselets,
        }
    }

    /// Gets the precedence of the given token.
    fn get_precedence(&self, tokenizer: &mut Tokenizer) -> u8 {
        let token = match tokenizer.peek() {
            Some(t) => t,
            None => return 0,
        };

        token.get_type().into()
    }

    /// Parses the token stream and returns an expression, if possible.
    pub fn parse(&self, precedence: u8, tokenizer: &mut Tokenizer) -> Option<Expression> {
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

        let mut left: Expression = parselet.parse(self, tokenizer, token);

        while precedence < self.get_precedence(tokenizer) {
            let token = match tokenizer.peek() {
                Some(t) => t,
                None => break,
            };
    
            // Get the proper infix parselet from the type of the given token,
            // or return the current expression.
            let parselet: &Box<dyn InfixParselet> = match self.infix_parselets.get(&token.get_type()) {
                Some(p) => p,
                None => break,
            };
    
            tokenizer.next();
    
            left = parselet.parse(self, tokenizer, left, token);
        }

        Some(left)
    }

    /// Parses the program into a list of expressions.
    pub fn parse_all(&self, tokenizer: &mut Tokenizer) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while let Some(e) = self.parse(0, tokenizer) {
            expressions.push(e);
        }

        expressions
    }
}