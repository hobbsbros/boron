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
    ffi::OsStr,
};

use walkdir::WalkDir;

pub mod tokenizer;
pub mod parser;
pub mod emitter;
pub mod version;
pub mod error;

use tokenizer::Tokenizer;
use parser::Parser;
use emitter::Emitter;
use error::{Error, throw};


/// Enumerates the types of processes that the Boron compiler can execute.
#[derive(Copy, Clone)]
pub enum Process {
    Lib,
    Exe,
    Build,
}


/// Provides an abstraction over CLI arguments.
pub struct Args {
    filename: Option<String>,
    process: Process,
}

impl Args {
    /// Constructs a new instance of CLI arguments.
    pub fn new() -> Self {
        Self {
            filename: None,
            process: Process::Exe,
        }
    }

    /// Sets the filename.
    pub fn set_filename(&mut self, f: String) {
        self.filename = Some(f);
    }

    /// Marks this as a header file (rather than an executable).
    pub fn mark_lib(&mut self) {
        self.process = Process::Lib;
    }

    /// Marks this as an executable.
    pub fn mark_exe(&mut self) {
        self.process = Process::Exe;
    }

    /// Marks this as a directory build.
    pub fn mark_build(&mut self) {
        self.process = Process::Build;
    }

    /// Gets whether or not this is a library.
    pub fn get_process(&self) -> Process {
        self.process
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
            match arg.as_str() {
                "--lib" => args.mark_lib(),
                "--exe" => args.mark_exe(), // NOTE: this is marked by default
                "--build" => args.mark_build(),
                _ => throw(Error::UnexpectedCliFlag (arg)),
            }
        } else {
            args.set_filename(arg);
        }
    }
    
    

    match args.get_process() {
        Process::Lib => compile_lib(args),
        Process::Exe => compile_exe(args),
        Process::Build => build(args),
    };
}


fn compile_lib(args: Args) {
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
    output_filename.push_str(".h");

    let output = emitter.compile_lib(args.get_bare_filename(), expressions);

    // Open a file for output
    let mut output_file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_filename.to_owned())
    {
        Ok(f) => f,
        Err(_) => throw(Error::CouldNotCreate (output_filename.to_owned())),
    };

    match output_file.write_all(output.as_bytes()) {
        Ok(_) => (),
        Err(_) => throw(Error::CouldNotWriteFile (output_filename.to_owned())),
    }
}


fn compile_exe(args: Args) {
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
    output_filename.push_str(".c");

    let output = emitter.compile_exe(expressions);

    // Open a file for output
    let mut output_file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_filename.to_owned())
    {
        Ok(f) => f,
        Err(_) => throw(Error::CouldNotCreate (output_filename.to_owned())),
    };

    match output_file.write_all(output.as_bytes()) {
        Ok(_) => (),
        Err(_) => throw(Error::CouldNotWriteFile (output_filename.to_owned())),
    }
}


fn build(args: Args) {
    // Walk the given directory
    let mut filenames: Vec<String> = Vec::new();
    for entry in WalkDir::new(&args.get_filename()) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => throw(Error::CouldNotReadFile (args.get_filename())),
        };
        if entry.path().extension() == Some(OsStr::new("brn")) {
            filenames.push(entry.path().display().to_string());
        }
    }

    for filename in filenames {
        let mut filename_args = Args::new();
        filename_args.set_filename(filename);
        filename_args.mark_lib();
        compile_lib(filename_args);
    }
}