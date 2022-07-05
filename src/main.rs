//! A simple compiler for the Boron programming language.

use std::{
    fs,
    env,
};

use tokenizer::{
    Tokenizer,
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

    dbg!(tokenizer.collect());
}