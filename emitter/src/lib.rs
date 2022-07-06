//! A simple C emitter for the Boron compiler.


use std::collections::HashMap;

use chrono::{
    Datelike,
    Timelike,
    Local,
};

use version::VERSION;

use parser::{
    Expression,
    TokenType,
};


pub struct Emitter {
    variables: HashMap<String, String>,
    code: String,
}

impl Emitter {
    /// Constructs a new emitter.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            code: String::new(),
        }
    }

    /// Emits an operation symbol based on a token type.
    fn match_op(&self, op: TokenType) -> String {
        let op_str = match op {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Multiply => "*",
            TokenType::Divide => "/",
            _ => todo!(),
        };

        op_str.to_owned()
    }

    /// Emits a `printf` expression.
    fn emit_printf(&self, args: Vec<Expression>) -> String {
        // "print" is a special case due to idiosyncracies of C & Boron
        let mut emitted = String::new();
        for arg in args {
            if let Expression::Identifier (id) = arg {
                emitted.push_str("printf(");
                // TODO: don't use unwrap here.
                match self.variables.get(&id).unwrap().as_str() {
                    "int" => {
                        emitted.push_str("\"%d\\n\", ");
                        emitted.push_str(&id);
                    },
                    "float" => {
                        emitted.push_str("\"%f\\n\", ");
                        emitted.push_str(&id);
                    },
                    "bool" => {
                        emitted.push_str(&id);
                        emitted.push_str(" ? \"true\\n\" : \"false\\n\"");
                    }
                    _ => todo!(),
                }
                emitted.push_str(");\n\t");
            }
        }
        emitted.pop();
        emitted.pop();
        emitted.pop();
        emitted
    }

    /// Emits an expression.
    fn emit(&self, expr: &Expression) -> String {
        // Emit the given expression as a string
        let value: String = match expr {
            Expression::Int (i) => format!("{}", i),
            Expression::Float (f) => format!("{}", f),
            Expression::Bool (b) => format!("{}", b),
            Expression::Identifier (s) => format!("{}", s),
            Expression::Type (_) => todo!(),
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => format!("{} {} {}", self.emit(&*l), self.match_op(*o), self.emit(&*r)),
            Expression::Declaration {
                datatype: d,
                identifier: i,
            } => format!("{} {}", d, i),
            Expression::Assignment {
                datatype: d,
                identifier: i,
                value: e,
            } => format!("{} {} = {}", d, i, self.emit(&*e)),
            Expression::Reassignment {
                identifier: i,
                value: e,
            } => format!("{} = {}", i, self.emit(&*e)),
            Expression::FnCall {
                name: n,
                args: a,
            } => {
                match n.as_str() {
                    "print" => {
                        self.emit_printf(a.clone())
                    },
                    _ => {
                        let mut emitted = format!("{}(", n).to_string();
                        // Emit each argument recursively
                        for (idx, arg) in a.iter().enumerate() {
                            emitted.push_str(&format!("{}", self.emit(arg)));
                            if idx < a.len() - 1 {
                                emitted.push('\n');
                            }
                        }
                        emitted.push(')');
                        emitted.to_owned()
                    }
                }
            },
        };

        value.to_owned()
    }

    /// Emits a section of code and inserts a semicolon and a new line by concatenating to the C program.
    fn writescln(&mut self, s: &str) {
        self.code.push_str(s);
        self.code.push(';');
        self.code.push('\n');
    }

    /// Emits a section of code and inserts a new line by concatenating to the C program.
    fn writeln(&mut self, s: &str) {
        self.code.push_str(s);
        self.code.push('\n');
    }

    /// Emits a section of code by concatenating to the C program.
    fn write(&mut self, s: &str) {
        self.code.push_str(s);
    }

    /// Compiles a list of expressions into a string of C code.
    pub fn compile(&mut self, expressions: Vec<Expression>) -> String {
        // Create a list of variables
        for expression in &expressions {
            if let Expression::Assignment {
                datatype: d,
                identifier: i,
                value: _,
            } = expression {
                self.variables.insert(i.clone(), d.clone());
            }
        }

        // Get current time
        let now = Local::now();
        let time: String = format!(
            "// Created on {:02}/{:02}/{:02} at {:02}:{:02}:{:02} local time",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
        );

        // Get current version
        let version: String = format!("// Version {}", VERSION);

        // Emit file metadata
        self.writeln("// Autogenerated by the Boron compiler");
        self.writeln(&version);
        self.writeln(&time);
        self.writeln("");
        self.writeln("");

        // Emit header information
        self.writeln("#include <stdio.h>");
        self.writeln("#include <stdbool.h>");
        self.writeln("");
        self.writeln("int main(void) {");

        for expression in expressions {
            let statement = self.emit(&expression);
            self.write("\t");
            self.writescln(&statement);
        }

        self.writeln("}");

        self.code.to_owned()
    }
}