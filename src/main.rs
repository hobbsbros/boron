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
    fs::{
        self,
        OpenOptions,
    },
    env,
    io::Write,
};

use tokenizer::{
    Tokenizer,
};

use parser::{
    Parser,
};

use emitter::{
    Emitter,
};


fn main() {
    println!("Boron Compiler");

    // Get the input filename.
    let filename: String = match env::args().nth(1) {
        Some(f) => f,
        None => todo!(),
    };

    let code = match fs::read_to_string(&filename) {
        Ok(c) => c,
        Err(_) => todo!(),
    };

    let mut tokenizer = Tokenizer::new(code);

    let parser = Parser::new();
    let expressions = parser.parse_all(&mut tokenizer);

    let mut emitter = Emitter::new();
    let code = emitter.compile(expressions);

    // Open a file for output
    let mut output = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("out.c")
    {
        Ok(f) => f,
        Err(_) => todo!(),
    };

    match output.write_all(code.as_bytes()) {
        Ok(_) => println!("Successfully compiled.  Output written to 'out.c'."),
        Err(_) => todo!(),
    }
}