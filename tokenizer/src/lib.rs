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
    pub fn peek(&self) -> char {
        // If beyond the end of the source string, return EOF.
        if self.index >= self.source.len() {
            '\0'
        } else {
            self.source[self.index]
        }
    }

    /// Gets the next character in the stream and advances the stream.
    pub fn next(&mut self) -> char {
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

    /// Yields the next token from the token stream.
    pub fn next(&mut self) -> Self {
        todo!();
    }

    /// Consumes the character stream and yields all tokens.
    /// Note: this is only for debugging purposes.
    pub fn collect(&mut self) -> Vec<Token> {
        todo!();
    }
}