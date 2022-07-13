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


/// Provides an abstraction over CLI arguments.
pub struct Args {
    filename: Option<String>,
    lib: bool,
}

impl Args {
    /// Constructs a new instance of CLI arguments.
    pub fn new() -> Self {
        Self {
            filename: None,
            lib: false,
        }
    }

    /// Sets the filename.
    pub fn set_filename(&mut self, f: String) {
        self.filename = Some(f);
    }

    /// Marks this as a library (rather than an executable).
    pub fn mark_lib(&mut self) {
        self.lib = true;
    }

    /// Marks this as an executable.
    pub fn mark_exe(&mut self) {
        self.lib = false;
    }

    /// Gets whether or not this is a library.
    pub fn is_lib(&self) -> bool {
        self.lib
    }

    /// Gets the filename from the CLI args.
    pub fn get_filename(&self) -> String {
        match &self.filename {
            Some(f) => f.to_owned(),
            None => throw(Error::NoFileProvided),
        }
    }

    /// Gets the bare filename (without extension) from the CLI args.
    pub fn get_bare_filename(&self) -> String {
        let mut f = match &self.filename {
            Some(f) => f.to_owned(),
            None => throw(Error::NoFileProvided),
        };
        f.truncate(f.len() - 4);
        f.to_owned()
    }
}


fn main() {
    // Sets up a CLI args struct.
    let mut args = Args::new();

    for arg in env::args() {
        if arg.starts_with("--") {
            if arg == "--lib" {
                args.mark_lib();
            } else if arg == "--exe" {
                args.mark_exe();
            } else {
                throw(Error::UnexpectedCliFlag (arg));
            }
        } else {
            args.set_filename(arg);
        }
    }
    
    let code = match read_to_string(&args.get_filename()) {
        Ok(c) => c,
        Err(_) => throw(Error::CouldNotReadFile (args.get_filename())),
    };

    let mut tokenizer = Tokenizer::new(code);

    let parser = Parser::new();
    let expressions = parser.parse_all(&mut tokenizer);

    let mut emitter = Emitter::new();

    let mut output_filename = args.get_filename();
    output_filename.truncate(output_filename.len() - 4);

    let code = match args.is_lib() {
        true => {
            output_filename.push_str(".h");
            emitter.compile_lib(args.get_bare_filename(), expressions)
        },
        false => {
            output_filename.push_str(".c");
            emitter.compile_exe(expressions)
        },
    };

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