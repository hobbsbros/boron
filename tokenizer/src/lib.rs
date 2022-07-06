//! A simple tokenizer for the Boron compiler.


pub mod token;


pub use token::{
    Token,
    TokenType,
};


/// Creates a character stream.
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Tokenizer {
    tokenstream: Vec<Token>,
    index: usize,
}

const WHITESPACE: &str = "\n ,";
const SEPARATORS: &str = "\n ():,";

/// Provides functions for the `Tokenizer` struct.
impl Tokenizer {
    /// Constructs a new token stream from a string.
    pub fn new(string: String) -> Self {
        let mut charstream = CharStream::new(string);

        let mut tokenstream = Vec::new();
        
        while let Some(t) = Self::next_token(&mut charstream) {
            tokenstream.push(t);
        }

        Self {
            tokenstream,
            index: 0,
        }
    }

    /// Skips all whitespace (newlines, spaces, and comments)
    fn skip_whitespace(charstream: &mut CharStream) {
        while let Some(c) = charstream.peek() {
            if WHITESPACE.contains(c) {
                charstream.next();
            } else {
                break;
            }
        }
    }

    /// Yields the next token from the character stream.
    fn next_token(charstream: &mut CharStream) -> Option<Token> {
        // Skip whitespace
        Self::skip_whitespace(charstream);

        if charstream.peek() == Some('#') {
            while let Some(c) = charstream.peek() {
                if c != '\n' {
                    charstream.next();
                } else {
                    break;
                }
            }
        }

        // Skip any whitespace again
        Self::skip_whitespace(charstream);

        let character = match charstream.next() {
            Some(c) => c,
            None => return None,
        };
        
        let token = match character {
            // EOF
            '\0' => return None,
            // Open parenthesis
            '(' => Token::new(character.to_string(), TokenType::OpenParen),
            // Closing parenthesis
            ')' => Token::new(character.to_string(), TokenType::CloseParen),
            // Assignment
            ':' => Token::new(character.to_string(), TokenType::Assignment),
            // Plus
            '+' => Token::new(character.to_string(), TokenType::Plus),
            // Minus
            '-' => Token::new(character.to_string(), TokenType::Minus),
            // Multiply
            '*' => Token::new(character.to_string(), TokenType::Multiply),
            // Divide
            '/' => Token::new(character.to_string(), TokenType::Divide),
            // Not
            '!' => Token::new(character.to_string(), TokenType::Not),
            // Integer or floating-point
            '0'..='9' => {
                let mut sofar = String::from(character);
                while let Some(chr) = charstream.peek() {
                    if !SEPARATORS.contains(chr) {
                        sofar.push(chr);
                        charstream.next();
                    } else {
                        break;
                    }
                }

                let mut token = Token::new(sofar.clone(), TokenType::Unknown);

                if let Ok(_) = str::parse::<i32>(&sofar) {
                    token = Token::new(sofar, TokenType::Int);
                } else if let Ok(_) = str::parse::<f32>(&sofar) {
                    token = Token::new(sofar, TokenType::Float);
                }

                token
            },
            // Identifier or type keyword
            'A'..='z' => {
                let mut sofar = String::from(character);
                while let Some(chr) = charstream.peek() {
                    if !SEPARATORS.contains(chr) {
                        sofar.push(chr);
                        charstream.next();
                    } else {
                        break;
                    }
                }

                let token = match sofar.as_str() {
                    "int" => Token::new(sofar, TokenType::Type),
                    "float" => Token::new(sofar, TokenType::Type),
                    "bool" => Token::new(sofar, TokenType::Type),
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

    /// Gets the next token and advances the stream.
    pub fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.index += 1;
        token
    }

    /// Gets the next token without advancing the stream.
    pub fn peek(&self) -> Option<Token> {
        if self.index >= self.tokenstream.len() {
            None
        } else {
            Some(self.tokenstream[self.index].to_owned())
        }
    }

    /// Yields all tokens in the stream.
    /// This *does not* consume the token stream.
    pub fn collect(&self) -> Vec<Token> {
        self.tokenstream.to_owned()
    }
}