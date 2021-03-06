//! Provides abstractions over tokens.


/// Enumerates token types available.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SingleQuote,
    Int,
    Float,
    Bool,
    Let,
    Assignment,
    FnDeclaration,
    FnReturnType,
    Return,
    Identifier,
    Plus,
    Minus,
    Multiply,
    Divide,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    Not,
    Type,
    Struct,
    While,
    If,
    Else,
    TernaryIf,
    TernaryElse,
    Use,
    Unknown,
}


/// Defines an abstraction over tokens.
#[derive(Clone, Debug)]
pub struct Token {
    val: String,
    t: TokenType,
}


/// Provides functions for the `Token` struct.
impl Token {
    /// Constructs a new token from a string and a token type.
    pub fn new(token_value: String, token_type: TokenType) -> Self {
        Self {
            val: token_value,
            t: token_type,
        }
    }

    /// Checks if this token is of the given type.
    pub fn check(&self, token_type: TokenType) -> bool {
        self.t == token_type
    }

    /// Gets the string associated with this token.
    pub fn get_value(&self) -> String {
        self.val.to_owned()
    }

    /// Gets the type associated with this token.
    pub fn get_type(&self) -> TokenType {
        self.t
    }
}