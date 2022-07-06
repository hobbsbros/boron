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
    fs,
    env,
};

use tokenizer::{
    Tokenizer,
};

use parser::{
    Parser,
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

    dbg!(&tokenizer.clone().collect());

    let parser = Parser::new();

    dbg!(parser.parse(&mut tokenizer));
    dbg!(parser.parse(&mut tokenizer));
}