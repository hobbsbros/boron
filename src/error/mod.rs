//! Provides error handling for the Boron compiler.

use std::process::exit;

use colored::*;

use crate::version::VERSION;

/// Outlines different errors thrown by the Boron compiler.
pub enum Error {
    NoFileProvided,
    CouldNotReadFile (String),
    CouldNotCreate (String),
    CouldNotWriteFile (String),
    CouldNotEmit (String),
    CouldNotParse (String),
    InvalidOperator (String),
    UnexpectedEof (String),
    ExpectedIdentifier (String),
    ExpectedDatatypeKeyword (String),
    ExpectedBoolean (String),
    ExpectedLiteral (String),
    ExpectedOpenParen (String),
    ExpectedCloseParen (String),
    ExpectedOpenBrace (String),
}


pub fn throw(e: Error) -> ! {
    println!("{}", "The Boron Compiler".truecolor(102, 153, 204).bold());
    println!("Version {}", VERSION);
    println!("");

    match e {
        Error::NoFileProvided => {
            println!("{}: No input file specified.", "Error".bold().red());
        },
        Error::CouldNotReadFile (s) => {
            println!("{}: Could not read input file {}", "Error".bold().red(), s);
        },
        Error::CouldNotCreate (s) => {
            println!("{}: Could not open output file {}", "Error".bold().red(), s);
        },
        Error::CouldNotWriteFile (s) => {
            println!("{}: Could not write to output file {}", "Error".bold().red(), s);
        },
        Error::CouldNotEmit (s) => {
            println!("{}: Could not emit code near {}", "Error".bold().red(), s);
        },
        Error::CouldNotParse (s) => {
            println!("{}: Could not parse code near token {}", "Error".bold().red(), s);
        },
        Error::InvalidOperator (s) => {
            println!("{}: Could not parse code near invalid operator {}", "Error".bold().red(), s);
        },
        Error::UnexpectedEof (s) => {
            println!("{}: File unexpectedly terminates near token {}", "Error".bold().red(), s);
        },
        Error::ExpectedIdentifier (s) => {
            println!("{}: Expected identifier, got token {}", "Error".bold().red(), s);
        },
        Error::ExpectedDatatypeKeyword (s) => {
            println!("{}: Expected datatype, got token {}", "Error".bold().red(), s);
        },
        Error::ExpectedBoolean (s) => {
            println!("{}: Expected boolean type, got token {}", "Error".bold().red(), s);
        },
        Error::ExpectedLiteral (s) => {
            println!("{}: Expected literal, got token {}", "Error".bold().red(), s);
        },
        Error::ExpectedOpenParen (s) => {
            println!("{}: Expected opening parenthesis, got token {}", "Error".bold().red(), s);
        },
        Error::ExpectedCloseParen (s) => {
            println!("{}: Expected closing parenthesis, got token {}", "Error".bold().red(), s);
        },
        Error::ExpectedOpenBrace (s) => {
            println!("{}: Expected open curly brace '{{', got token {}", "Error".bold().red(), s);
        },
    };
    
    println!("Compiler exiting");

    exit(0);
}