//! Provides error handling for the Boron compiler.

use std::process::exit;

use colored::*;

/// Outlines different errors thrown by the Boron compiler.
pub enum Error {
    NoFileProvided,
    CouldNotReadFile (String),
    CouldNotCreate (String),
    CouldNotWriteFile (String),
    CouldNotEmit (String),
}


pub fn throw(e: Error) -> ! {
    match e {
        Error::NoFileProvided => {
            println!("{}: No input files.", "Error".bold().red());
            println!("Compiler exiting.");
        },
        Error::CouldNotReadFile (s) => {
            println!("{}: Could not read input file {}", "Error".bold().red(), s);
            println!("Compiler exiting.");
        },
        Error::CouldNotCreate (s) => {
            println!("{}: Could not open output file {}", "Error".bold().red(), s);
            println!("Compiler exiting.");
        },
        Error::CouldNotWriteFile (s) => {
            println!("{}: Could not write to output file {}", "Error".bold().red(), s);
            println!("Compiler exiting.");
        },
        Error::CouldNotEmit (s) => {
            println!("{}: Could not emit code near {}", "Error".bold().red(), s);
            println!("Compiler exiting.");
        },
    };
    
    exit(0);
}