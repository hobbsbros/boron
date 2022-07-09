//! A simple compiler for the Boron programming language.
//! 
//! The Boron compiler emits standard C code, which can subsequently
//! be compiled by a compiler of your choice.  The authors of the
//! Boron compiler recommends GCC.  Integration with a C compiler (so
//! that the Boron compiler emits machine code) may be included in
//! a future update.
//! 
//! One benefit of emitting standard C code is that the Boron compiler
//! can remain light and portable.


use std::{
    fs::{read_to_string, OpenOptions},
    env,
    io::Write,
};

pub mod tokenizer;
pub mod parser;
pub mod emitter;
pub mod version;
pub mod error;

use tokenizer::Tokenizer;
use parser::Parser;
use emitter::Emitter;
use error::{Error, throw};


fn main() {
    // Get the input filename.
    let filename: String = match env::args().nth(1) {
        Some(f) => f,
        None => throw(Error::NoFileProvided),
    };

    let code = match read_to_string(&filename) {
        Ok(c) => c,
        Err(_) => throw(Error::CouldNotReadFile (filename.to_owned())),
    };

    let mut tokenizer = Tokenizer::new(code);

    let parser = Parser::new();
    let expressions = parser.parse_all(&mut tokenizer);

    let mut emitter = Emitter::new();
    let code = emitter.compile(expressions);

    let mut output_filename = filename.clone();
    output_filename.truncate(filename.len() - 4);
    output_filename.push_str(".c");

    // Open a file for output
    let mut output = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_filename.to_owned())
    {
        Ok(f) => f,
        Err(_) => throw(Error::CouldNotCreate (output_filename.to_owned())),
    };

    match output.write_all(code.as_bytes()) {
        Ok(_) => (),
        Err(_) => throw(Error::CouldNotWriteFile (output_filename.to_owned())),
    }
}