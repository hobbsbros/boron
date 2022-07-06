//! Provides error handling for the Boron compiler.

use std::process::exit;

use colored::*;

/// Outlines different errors thrown by the Boron compiler.
pub enum Error {
    NoFileProvided,
}


pub fn throw(e: Error) -> ! {
    match e {
        Error::NoFileProvided => {
            println!("{}: No input files.", "Error".bold().red());
            println!("Compiler exiting.");
        },
    };
    
    exit(0);
}