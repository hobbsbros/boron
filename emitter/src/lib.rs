//! A simple C emitter for the Boron compiler.


use parser::{
    Expression,
};


pub struct Emitter {
    code: String,
}

impl Emitter {
    /// Constructs a new emitter.
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    /// Emits an expression.
    fn emit(expr: &Expression) -> String {
        // Emit the given expression as a string
        let value: String = match expr {
            Expression::Int (i) => format!("{}", i),
            Expression::Float (f) => format!("{}", f),
            Expression::Bool (b) => format!("{}", b),
            Expression::Identifier (s) => format!("{}", s),
            Expression::Type (_) => todo!(),
            Expression::Declaration {
                datatype: d,
                identifier: i,
            } => format!("{} {}", d, i),
            Expression::Assignment {
                datatype: d,
                identifier: i,
                value: e,
            } => format!("{} {} = {}", d, i, Emitter::emit(&*e)),
            Expression::FnCall {
                name: n,
                args: a,
            } => {
                let mut emitted = format!("{}(", n).to_string();
                // Emit each argument recursively
                for (idx, arg) in a.iter().enumerate() {
                    emitted.push_str(&format!("{}", Emitter::emit(arg)));
                    if idx < a.len() - 1 {
                        emitted.push('\n');
                    }
                }
                emitted.push(')');
                emitted.to_owned()
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

    /// Compiles a list of expressions into a string of C code.
    pub fn compile(&mut self, expressions: Vec<Expression>) -> String {
        // Emit header file information
        self.writeln("#include <stdio.h>");
        self.writeln("int main(void) {");

        for expression in expressions {
            let statement = Emitter::emit(&expression);
            self.writescln(&statement);
        }

        self.writeln("}");

        self.code.to_owned()
    }
}