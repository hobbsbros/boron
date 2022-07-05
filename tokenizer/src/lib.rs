//! A simple tokenizer for the Boron compiler.


pub mod token;


use token::{
    Token,
    TokenType,
};


/// Creates a character stream.
pub struct CharStream {
    source: Vec<char>,
    index: usize,
}

/// Provides functions for the `CharStream` struct.
impl CharStream {
    /// Constructs a new character stream from a given string.
    pub fn new(src: String) -> Self {
        Self {
            source: src.chars().collect::<Vec<char>>(),
            index: 0,
        }
    }

    /// Gets the next character in the stream without advancing the stream.
    pub fn peek(&self) -> Option<char> {
        // If beyond the end of the source string, return EOF.
        if self.index >= self.source.len() {
            None
        } else {
            Some(self.source[self.index])
        }
    }

    /// Gets the next character in the stream and advances the stream.
    pub fn next(&mut self) -> Option<char> {
        let character = self.peek();
        self.index += 1;
        character
    }
}


/// Provides an abstraction over tokenization behavior.
pub struct Tokenizer {
    charstream: CharStream,
}

/// Provides functions for the `Tokenizer` struct.
impl Tokenizer {
    /// Constructs a new token stream from a stream of characters.
    pub fn from_charstream(charstream: CharStream) -> Self {
        Self {
            charstream,
        }
    }

    /// Constructs a new token stream from a string.
    pub fn new(string: String) -> Self {
        let charstream = CharStream::new(string);
        Self::from_charstream(charstream)
    }

    /// Gets the next character in the character stream.
    fn next_char(&mut self) -> Option<char> {
        self.charstream.next()
    }

    /// Peeks at the next character in the character stream.
    fn peek_char(&self) -> Option<char> {
        self.charstream.peek()
    }

    /// Yields the next token from the token stream.
    pub fn next(&mut self) -> Option<Token> {
        let character = match self.next_char() {
            Some(c) => c,
            None => return None,
        };
        
        let token = match character {
            // EOF
            '\0' => Token::new('\0'.to_string(), TokenType::Eof),
            // Newline
            '\n' => Token::new('\n'.to_string(), TokenType::Newline),
            // Open parenthesis
            '(' => Token::new('('.to_string(), TokenType::OpenParen),
            // Closing parenthesis
            ')' => Token::new(')'.to_string(), TokenType::CloseParen),
            // Integer or floating-point
            '0'..='9' => {
                let mut sofar = String::from(character);
                while let Some(character) = self.peek_char() {

                }
                Token::new(sofar, TokenType::Int)
            },
            // Identifier or type keyword
            'A'..='z' => {
                let mut sofar = String::from(character);
                while let Some(chr) = self.next_char() {
                    if chr != ' ' {
                        sofar.push(chr);
                    } else {
                        break;
                    }
                }

                let token = match sofar.as_str() {
                    "int" => Token::new(sofar, TokenType::IntType),
                    "float" => Token::new(sofar, TokenType::FloatType),
                    "bool" => Token::new(sofar, TokenType::BoolType),
                    "true" => Token::new(sofar, TokenType::Bool),
                    "false" => Token::new(sofar, TokenType::Bool),
                    _ => Token::new(sofar, TokenType::Identifier),
                };

                token
            },
            // Unknown type
            _ => Token::new(character.to_string(), TokenType::Unknown),
        };

        Some(token)
    }

    /// Consumes the character stream and yields all tokens.
    /// Note: this is only for debugging purposes.
    pub fn collect(&mut self) -> Vec<Token> {
        todo!();
    }
}