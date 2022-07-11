//! A simple C emitter for the Boron compiler.


use std::collections::HashMap;

use chrono::{
    Datelike,
    Timelike,
    Local,
};

use crate::version::VERSION;

use crate::parser::{
    Expression,
    TokenType,
};

use crate::error::{
    throw,
    Error,
};


/// Holds a list of variable scopes.
#[derive(Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

/// Provides an interface for the environment.
impl Environment {
    /// Constructs a new environment.
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
        }
    }

    /// Adds a new scope to the environment and returns the scope's ID.
    pub fn add(&mut self, parent: Option<usize>) -> usize {
        let new = Scope {
            variables: HashMap::new(),
            id: self.scopes.len(),
            parent,
        };
        let id = new.get_id();
        self.scopes.push(new);
        id
    }

    /// Registers a variable within the given scope.
    pub fn register(&mut self, id: usize, varname: String, vartype: String) {
        self.scopes[id].register(varname, vartype);
    }

    /// Looks up a variable in the given scope.
    pub fn lookup(&self, id: usize, varname: &String) -> String {
        match self.scopes[id].get(varname) {
            Some(s) => s.to_string(),
            None => match self.scopes[id].get_parent() {
                Some(p) => self.lookup(p, varname),
                None => throw(Error::UndeclaredVariable (varname.to_string())),
            }
        }
    }
}


/// Abstracts over variable scopes.
#[derive(Clone, Debug)]
pub struct Scope {
    variables: HashMap<String, String>,
    id: usize,
    parent: Option<usize>,
}

/// Provides an interface for interacting with scopes.
impl Scope {
    /// Gets the ID of the given scope.
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// Gets the ID of the given scope's parent.
    pub fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    /// Registers a variable within the scope.
    pub fn register(&mut self, varname: String, vartype: String) {
        self.variables.insert(varname, vartype);
    }

    /// Looks up a variable in the given scope.
    pub fn get(&self, varname: &String) -> Option<&String> {
        self.variables.get(varname)
    }
}

#[test]
fn scoping() {
    // Create a new environment.
    let mut global = Environment::new();
    let s1 = global.add(None);
    let s2 = global.add(Some(s1));
    let s3 = global.add(Some(s2));

    global.get(s1).register("x".to_string(), "int".to_string());
    assert_eq!(global.get(s3).lookup(&global, "x".to_string()), "int".to_string());
}


/// Provides an abstraction over the Boron-to-C emitter.
pub struct Emitter {
    code: String,
    environment: Environment,
}

/// Provides an interface for the Boron-to-C emitter.
impl Emitter {
    /// Constructs a new emitter.
    pub fn new() -> Self {
        Self {
            code: String::new(),
            environment: Environment::new(),
        }
    }

    /// Emits an operation symbol based on a token type.
    fn match_op(&self, op: TokenType) -> String {
        let op_str = match op {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Multiply => "*",
            TokenType::Divide => "/",
            TokenType::Greater => ">",
            TokenType::Less => "<",
            TokenType::Equal => "==",
            TokenType::Not => "!",
            TokenType::GreaterEqual => ">=",
            TokenType::LessEqual => "<=",
            _ => throw(Error::CouldNotEmit ("operation".to_string())),
        };

        op_str.to_owned()
    }

    /// Emits a datatype name based on the C name.
    fn match_type(&self, datatype: String) -> String {
        let type_str = match datatype.as_str() {
            "int" => "int",
            "flt" => "float",
            "bln" => "bool",
            _ => datatype.as_str(),
        };

        type_str.to_owned()
    }

    /// Emits a `printf` expression.
    fn emit_printf(&self, args: Vec<Expression>, scope: usize) -> String {
        // "print" is a special case due to idiosyncracies of C & Boron
        let mut emitted = String::new();
        for arg in args {
            if let Expression::Identifier (id) = arg {
                emitted.push_str("printf(");

                let var = self.environment.lookup(scope, &id);

                match var.as_str() {
                    "int" => {
                        emitted.push_str("\"%d\\n\", ");
                        emitted.push_str(&id);
                    },
                    "flt" => {
                        emitted.push_str("\"%f\\n\", ");
                        emitted.push_str(&id);
                    },
                    "bln" => {
                        emitted.push_str(&id);
                        emitted.push_str(" ? \"true\\n\" : \"false\\n\"");
                    }
                    _ => throw(Error::CouldNotEmit (id.to_string())),
                }
                emitted.push_str(");\n");
            }
        }
        emitted.pop();
        emitted.pop();
        emitted
    }

    /// Emits a block of code.
    fn emit_block(&mut self, block: Vec<Expression>, parent: Option<usize>) -> String {
        let scope = self.environment.add(parent);
        let mut code = String::new();

        for expression in block {
            code.push_str(&self.emit(&expression, scope));
            match expression {
                Expression::While {
                    condition: _,
                    body: _,
                } | Expression::If {
                    condition: _,
                    body: _,
                } | Expression::IfElse {
                    condition: _,
                    body_true: _,
                    body_false: _,
                } => code.push_str("\n"),
                _ => code.push_str(";\n"),
            };
        }

        code.to_owned()
    }

    /// Emits an expression.
    fn emit(&mut self, expr: &Expression, scope: usize) -> String {
        // Emit the given expression as a string
        let value: String = match expr {
            Expression::Int (i) => format!("{}", i),
            Expression::Float (f) => format!("{}", f),
            Expression::Bool (b) => format!("{}", b),
            Expression::Identifier (s) => format!("{}", s),
            Expression::Type (t) => throw(Error::CouldNotEmit (t.to_string())),
            Expression::UnaryOp {
                op: o,
                expr: e,
            } => format!("{}{}", self.match_op(*o), self.emit(&*e, scope)),
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => format!("({} {} {})", self.emit(&*l, scope), self.match_op(*o), self.emit(&*r, scope)),
            Expression::Declaration {
                datatype: d,
                identifier: i,
            } => format!("{} {}", self.match_type(d.to_string()), i),
            Expression::Assignment {
                datatype: d,
                identifier: i,
                value: e,
            } => {
                self.environment.register(scope, i.clone(), d.clone());
                format!("{} {} = {}", self.match_type(d.to_string()), i, self.emit(&*e, scope))
            },
            Expression::Reassignment {
                identifier: i,
                value: e,
            } => format!("{} = {}", i, self.emit(&*e, scope)),
            Expression::FnCall {
                name: n,
                args: a,
            } => {
                match n.as_str() {
                    "print" => self.emit_printf(a.clone(), scope),
                    _ => {
                        let mut emitted = format!("{}(", n);
                        // Emit each argument recursively
                        for (idx, arg) in a.iter().enumerate() {
                            emitted.push_str(&format!("{}", self.emit(arg, scope)));
                            if idx < a.len() - 1 {
                                emitted.push('\n');
                            }
                        }
                        emitted.push(')');
                        emitted.to_owned()
                    }
                }
            },
            Expression::While {
                condition: c,
                body: b,
            } => {
                let mut emitted = "while ".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope));
                emitted.push_str(" {\n");
                // Emit each expression in the while loop
                let block = self.emit_block(b.to_vec(), Some(scope));
                emitted.push_str(&block);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::If {
                condition: c,
                body: b,
            } => {
                let mut emitted = "if ".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope));
                emitted.push_str(" {\n");
                // Emit each expression in the if statement
                let block = self.emit_block(b.to_vec(), Some(scope));
                emitted.push_str(&block);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::IfElse {
                condition: c,
                body_true: t,
                body_false: f,
            } => {
                let mut emitted = "if ".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope));
                emitted.push_str(" {\n");
                // Emit each expression in the if statement
                let block_true = self.emit_block(t.to_vec(), Some(scope));
                emitted.push_str(&block_true);
                emitted.push_str("} else {\n");
                // Emit each expression in the else statement
                let block_false = self.emit_block(f.to_vec(), Some(scope));
                emitted.push_str(&block_false);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::TernaryIfElse {
                condition: c,
                body_true: t,
                body_false: f,
            } => {
                let mut emitted = "(".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope));
                // Emit the ternary if
                emitted.push_str(" ? ");
                // Emit the first expression
                emitted.push_str(&self.emit(&*t, scope));
                // Emit the ternary else
                emitted.push_str(" : ");
                // Emit the second expression
                emitted.push_str(&self.emit(&*f, scope));
                // Emit a closing parenthesis
                emitted.push(')');
                
                emitted.to_owned()
            },
            
        };

        value.to_owned()
    }

    /// Emits a section of code and inserts a new line by concatenating to the C program.
    fn writeln(&mut self, s: &str) {
        self.code.push_str(s);
        self.code.push('\n');
    }

    /// Compiles a list of expressions into a string of C code.
    pub fn compile(&mut self, expressions: Vec<Expression>) -> String {
        // Get current time
        let now = Local::now();
        let datetime: String = format!(
            "// Created on {:04}/{:02}/{:02} at {:02}:{:02}:{:02} local time",
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
        self.writeln(&datetime);
        self.writeln("");

        // Emit header information
        self.writeln("#include <stdio.h>");
        self.writeln("#include <stdbool.h>");
        self.writeln("");
        self.writeln("int main(void) {");

        let code = self.emit_block(expressions, None);
        self.writeln(&code);

        self.writeln("return 0;\n}");

        self.code.to_owned()
    }
}