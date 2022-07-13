//! A simple C emitter for the Boron compiler.


use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
};

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
            structs: HashMap::new(),
            id: self.scopes.len(),
            parent,
        };
        let id = new.get_id();
        self.scopes.push(new);
        id
    }

    /// Registers a variable within the given scope.
    pub fn register(&mut self, id: usize, varname: String, variable: Variable) {
        self.scopes[id].register(varname, variable);
    }

    /// Registers a structure within the given scope.
    pub fn register_struct(&mut self, id: usize, varname: String, structure: HashMap<String, Variable>) {
        self.scopes[id].register_struct(varname, structure);
    }

    /// Looks up a variable in the given scope.
    pub fn lookup(&self, id: usize, varname: &String) -> Variable {
        match self.scopes[id].get(varname) {
            Some(s) => s.to_owned(),
            None => match self.scopes[id].get_parent() {
                Some(p) => self.lookup(p, varname),
                None => throw(Error::UndeclaredVariable (varname.to_string())),
            }
        }
    }

    /// Checks if a variable exists in the given scope.
    pub fn check(&self, id: usize, varname: &String) -> bool {
        match self.scopes[id].get(varname) {
            Some(_) => true,
            None => match self.scopes[id].get_parent() {
                Some(p) => self.check(p, varname),
                None => false,
            }
        }
    }

    /// Looks up a structure in the given scope.
    pub fn lookup_struct(&self, id: usize, varname: &String) -> HashMap<String, Variable> {
        match self.scopes[id].get_struct(varname) {
            Some(s) => s.to_owned(),
            None => match self.scopes[id].get_parent() {
                Some(p) => self.lookup_struct(p, varname),
                None => throw(Error::UndeclaredVariable (varname.to_string())),
            }
        }
    }

    /// Checks if a structure exists in the given scope.
    pub fn check_struct(&self, id: usize, varname: &String) -> bool {
        match self.scopes[id].get_struct(varname) {
            Some(_) => true,
            None => match self.scopes[id].get_parent() {
                Some(p) => self.check_struct(p, varname),
                None => false,
            }
        }
    }
}


/// Represents the types of variables to be stored in a scope.
#[derive(Clone, Debug)]
pub enum Variable {
    Int,
    Float,
    Bool,
    Char,
    Void,
    Struct (String),
}

/// Provides an interface for functions on variable types.
impl Variable {
    /// Converts a Boron variable type into a `Variable` variant.
    pub fn from(string: &String) -> Self {
        match string.as_str() {
            "int" => Variable::Int,
            "flt" => Variable::Float,
            "bln" => Variable::Bool,
            "chr" => Variable::Char,
            "nul" => Variable::Void,
            _ => Variable::Struct (string.to_string()),
        }
    }
    
    /// Generates C code to print a variable.
    pub fn print(var: Self, id: &String) -> String {
        let mut emitted = String::new();
        match var {
            Variable::Int => {
                emitted.push_str("printf(\"%d\\n\", ");
                emitted.push_str(id);
                emitted.push_str(");\n");
            },
            Variable::Float => {
                emitted.push_str("printf(\"%f\\n\", ");
                emitted.push_str(id);
                emitted.push_str(");\n");
            },
            Variable::Bool => {
                emitted.push_str("printf(");
                emitted.push_str(id);
                emitted.push_str(" ? \"true\\n\" : \"false\\n\");\n");
            },
            Variable::Char => {
                emitted.push_str("printf(\"%c\\n\", ");
                emitted.push_str(id);
                emitted.push_str(");\n");
            },
            Variable::Void => {
                emitted.push_str("printf(\"\\n\")");
            },
            Variable::Struct (_) => {
                unreachable!()
            },
        }
        emitted.to_owned()
    }
}


/// Abstracts over variable scopes.
#[derive(Clone, Debug)]
pub struct Scope {
    variables: HashMap<String, Variable>,
    structs: HashMap<String, HashMap<String, Variable>>,
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
    pub fn register(&mut self, varname: String, variable: Variable) {
        self.variables.insert(varname, variable);
    }

    /// Registers a structure within the scope.
    pub fn register_struct(&mut self, varname: String, structure: HashMap<String, Variable>) {
        self.structs.insert(varname, structure);
    }

    /// Looks up a variable in the given scope.
    pub fn get(&self, varname: &String) -> Option<&Variable> {
        self.variables.get(varname)
    }

    /// Looks up a structure in the given scope.
    pub fn get_struct(&self, varname: &String) -> Option<&HashMap<String, Variable>> {
        self.structs.get(varname)
    }
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
    fn match_type(&self, datatype: String, lhs: bool) -> String {
        let type_str = match datatype.as_str() {
            "int" => "int".to_string(),
            "flt" => "float".to_string(),
            "bln" => "bool".to_string(),
            "chr" => "char".to_string(),
            "nul" => "void".to_string(),
            _ => if lhs {
                format!("struct {}", datatype.as_str())
            } else {
                format!("struct {} *", datatype.as_str())
            }
        };

        type_str
    }

    /// Emits a variable name, dereferencing a struct field if necessary.
    fn match_var(&self, var: String, in_fn: bool) -> String {
        if in_fn {
            var.replace(".", "->")
        } else {
            var
        }
    }

    /// Aliases imports from the standard library if necessary.
    fn match_module(&self, module: String) -> String {
        let top = module.split("/").collect::<Vec<&str>>();
        match top[0] {
            "std" => {
                // Get the current working directory
                let current_path = match env::current_dir() {
                    Ok(p) => p.as_path().to_owned(),
                    Err(_) => throw(Error::CouldNotGetCurrentDir),
                };
                for path in current_path.ancestors() {
                    if path.to_path_buf().join(".boron-std").is_dir() {
                        // Get the rest of the path (everything after `.boron-std`)
                        let mut rest_of_path = top[1..].iter().map(|x| OsStr::new(x)).collect::<Vec<&OsStr>>();
                        rest_of_path.insert(0, &OsStr::new(".boron-std"));
                        
                        let output =  path
                            .iter()
                            .chain(rest_of_path)
                            .map(|x| match x.to_str() {
                                Some(s) => s.to_string(),
                                None => throw(Error::CouldNotEmit ("use".to_string())),
                            })
                            .collect::<Vec<String>>();
                        if output[0] == "/" {
                            return format!("/{}", output[1..].join("/"));
                        } else {
                            return output.join("/");
                        }
                    }
                }
                throw(Error::CouldNotReadFile (module));          
            },
            _ => module.to_owned()
        }
    }

    /// Emits a `printf` expression.
    fn emit_printf(&self, args: Vec<Expression>, scope: usize, in_fn: bool) -> String {
        // "print" is a special case due to idiosyncracies of C & Boron
        let mut emitted = String::new();
        for arg in args {
            if let Expression::Identifier (id) = arg {
                let var: Variable = self.environment.lookup(scope, &id);
                emitted.push_str(&Variable::print(var, &self.match_var(id.to_string(), in_fn)));
            }
        }
        emitted.pop();
        emitted.pop();
        emitted
    }

    /// Emits a block of code.
    fn emit_block(&mut self, block: Vec<Expression>, parent: Option<usize>, in_fn: bool) -> (String, String, String, String) {
        let scope = self.environment.add(parent);
        let mut code = String::new();
        let mut functions = String::new();
        let mut structs = String::new();
        let mut includes = String::new();

        for expression in block {
            let line = &self.emit(&expression, scope, in_fn);
            match expression {
                Expression::Struct {
                    identifier: _,
                    variables: _,
                } => {
                    structs.push_str(line);
                    structs.push_str(";\n");
                },
                Expression::FnDeclaration {
                    identifier: _,
                    arguments: _,
                    return_type: _,
                    body: _,
                } => {
                    functions.push_str(line);
                    functions.push_str("\n");
                },
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
                } => {
                    code.push_str(line);
                    code.push_str("\n");
                },
                Expression::Use (_) => {
                    includes.push_str(line);
                    includes.push_str("\n");
                }
                _ => {
                    code.push_str(line);
                    code.push_str(";\n");
                }
            };
        }

        (includes.to_owned(), structs.to_owned(), functions.to_owned(), code.to_owned())
    }

    /// Emits an expression.
    fn emit(&mut self, expr: &Expression, scope: usize, in_fn: bool) -> String {
        // Emit the given expression as a string
        let value: String = match expr {
            Expression::Int (i) => format!("{}", i),
            Expression::Float (f) => format!("{}", f),
            Expression::Bool (b) => format!("{}", b),
            Expression::Char (c) => format!("'{}'", c),
            Expression::Identifier (s) => format!("{}", self.match_var(s.to_string(), in_fn)),
            Expression::Type (t) => throw(Error::CouldNotEmit (t.to_string())),
            Expression::UnaryOp {
                op: o,
                expr: e,
            } => format!("{}{}", self.match_op(*o), self.emit(&*e, scope, in_fn)),
            Expression::BinOp {
                left: l,
                op: o,
                right: r,
            } => format!("({} {} {})", self.emit(&*l, scope, in_fn), self.match_op(*o), self.emit(&*r, scope, in_fn)),
            Expression::Declaration {
                datatype: d,
                identifier: i,
            } => format!("{} {}", self.match_type(d.to_string(), false), i),
            Expression::Struct {
                identifier: i,
                variables: v,
            } => {
                let mut emitted = "struct ".to_string();
                emitted.push_str(&i);
                emitted.push_str(" {\n");
                let mut variables = HashMap::new();
                // Push each variable in the structure
                for (varname, vartype) in v.iter() {
                    emitted.push_str(&self.match_type(vartype.to_string(), false));
                    emitted.push(' ');
                    emitted.push_str(varname);
                    emitted.push_str(";\n");
                    variables.insert(varname.to_owned(), Variable::from(vartype));
                }
                // Register this structure in the scope
                self.environment.register_struct(scope, i.to_string(), variables);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::StructInit {
                variables: _,
            } => {
                throw(Error::FoundBareStruct);
            },
            Expression::Assignment {
                datatype: d,
                identifier: i,
                value: e,
            } => {
                match &**e {
                    Expression::StructInit {
                        variables: v,
                    } => {
                        // First, declare the struct
                        let mut emitted = format!("struct {} {};\n", &d, &i);
                        self.environment.register(scope, i.clone(), Variable::from(&d));
                        let structure = self.environment.lookup_struct(scope, &d);
                        for (index, (varname, variable)) in v.iter().enumerate() {
                            // `scoped_name` takes the form `struct.field`
                            let scoped_name = format!("{}.{}", &i, &varname);
                            let field = format!("{} = {}", &scoped_name, self.emit(variable, scope, in_fn));
                            let vartype = match structure.get(varname) {
                                Some(v) => v,
                                None => throw(Error::UndeclaredVariable (scoped_name.to_owned())),
                            };
                            // Register this field as a variable in the current environment
                            self.environment.register(scope, scoped_name.to_owned(), vartype.to_owned());
                            emitted.push_str(&field);
                            if index < v.len() - 1 {
                                emitted.push_str(";\n");
                            }
                        }
                        emitted.to_owned()
                    },
                    _ => {
                        let vartype = Variable::from(&d);
                        self.environment.register(scope, i.clone(), vartype.to_owned());

                        // If this is a struct, register each field in the given scope
                        if let Variable::Struct (s) = vartype {
                            let structure: HashMap<String, Variable> = self.environment.lookup_struct(scope, &s);
                            for (varname, variable) in structure.iter() {
                                // `scoped_name` takes the form `struct.field`
                                let scoped_name = format!("{}.{}", &i, &varname);
                                // Register this field as a variable in the current environment
                                self.environment.register(scope, scoped_name.to_owned(), variable.to_owned());
                            }
                        }

                        format!("{} {} = {}", self.match_type(d.to_string(), true), self.match_var(i.to_string(), in_fn), self.emit(&*e, scope, in_fn))
                    }
                }
            },
            Expression::Reassignment {
                identifier: i,
                value: e,
            } => match &**e {
                Expression::StructInit {
                    variables: v,
                } => {
                    // No need to declare the struct as this is a *reassignment*
                    let mut emitted = String::new();
                    for (index, (varname, variable)) in v.iter().enumerate() {
                        // `scoped_name` takes the form `struct.field`
                        let scoped_name = format!("{}.{}", &i, &varname);
                        let field = format!("{} = {}", &scoped_name, self.emit(variable, scope, in_fn));
                        emitted.push_str(&field);
                        if index < v.len() - 1 {
                            emitted.push_str(";\n");
                        }
                    }
                    emitted.to_owned()
                },
                _ => format!("{} = {}", self.match_var(i.to_string(), in_fn), self.emit(&*e, scope, in_fn))
            },
            Expression::FnCall {
                name: n,
                args: a,
            } => {
                match n.as_str() {
                    "print" => self.emit_printf(a.clone(), scope, in_fn),
                    _ => {
                        let names = n.split(".").collect::<Vec<&str>>();
                        let mut emitted = match names.len() {
                            1 => format!("{}(", names[0]),
                            2 => if a.len() != 0 {
                                format!("{}(&{}, ", names[1], names[0])
                            } else {
                                format!("{}(&{}", names[1], names[0])
                            },
                            _ => throw(Error::TooManyLeadingKeywords (n.to_string())),
                        };

                        // Emit each argument recursively
                        for (idx, arg) in a.iter().enumerate() {
                            let argument = if let Expression::Identifier (s) = arg {
                                format!("&{}", s)
                            } else {
                                format!("{}", self.emit(arg, scope, in_fn))
                            };
                            emitted.push_str(&argument);
                            if idx < a.len() - 1 {
                                emitted.push_str(", ");
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
                let mut emitted = "while (".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope, in_fn));
                emitted.push_str(") {\n");
                // Emit each expression in the while loop
                let block = self.emit_block(b.to_vec(), Some(scope), in_fn).3;
                emitted.push_str(&block);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::If {
                condition: c,
                body: b,
            } => {
                let mut emitted = "if (".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope, in_fn));
                emitted.push_str(") {\n");
                // Emit each expression in the if statement
                let block = self.emit_block(b.to_vec(), Some(scope), in_fn).3;
                emitted.push_str(&block);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::IfElse {
                condition: c,
                body_true: t,
                body_false: f,
            } => {
                let mut emitted = "if (".to_string();
                // Emit the condition
                emitted.push_str(&self.emit(&*c, scope, in_fn));
                emitted.push_str(") {\n");
                // Emit each expression in the if statement
                let block_true = self.emit_block(t.to_vec(), Some(scope), in_fn).3;
                emitted.push_str(&block_true);
                emitted.push_str("} else {\n");
                // Emit each expression in the else statement
                let block_false = self.emit_block(f.to_vec(), Some(scope), in_fn).3;
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
                emitted.push_str(&self.emit(&*c, scope, in_fn));
                // Emit the ternary if
                emitted.push_str(" ? ");
                // Emit the first expression
                emitted.push_str(&self.emit(&*t, scope, in_fn));
                // Emit the ternary else
                emitted.push_str(" : ");
                // Emit the second expression
                emitted.push_str(&self.emit(&*f, scope, in_fn));
                // Emit a closing parenthesis
                emitted.push(')');
                
                emitted.to_owned()
            },
            Expression::FnDeclaration {
                identifier: i,
                arguments: a,
                return_type: r,
                body: b,
            } => {
                // Add the return type to the emitted code
                let mut emitted = self.match_type(r.to_string(), true);
                // Add the function name and an opening parenthesis
                emitted.push(' ');
                emitted.push_str(&i);
                emitted.push('(');
                // Add each argument's type and name
                // Also register each argument as a variable in the current scope
                for (index, (arg, argtype)) in a.iter().enumerate() {
                    emitted.push_str(&self.match_type(argtype.to_string(), false));
                    emitted.push(' ');
                    emitted.push_str(arg);
                    if index < a.len() - 1 {
                        emitted.push_str(", ");
                    }

                    let var = Variable::from(&argtype);
                    self.environment.register(scope, arg.clone(), Variable::from(&argtype));

                    // If this is a struct, we need to register each field as well
                    if let Variable::Struct (s) = var {
                        let fields: HashMap<String, Variable> = self.environment.lookup_struct(scope, &s);
                        for (varname, variable) in fields.iter() {
                            let scoped_varname = format!("{}.{}", &arg, &varname);
                            self.environment.register(scope, scoped_varname.to_owned(), variable.to_owned());
                        }
                    }
                }
                emitted.push_str(") {\n");
                // Emit the body
                let block = self.emit_block(b.to_vec(), Some(scope), true).3;
                emitted.push_str(&block);
                emitted.push_str("}");
                emitted.to_owned()
            },
            Expression::Return (v) => {
                let mut emitted = "return ".to_string();
                let expr = self.emit(&*v, scope, true);
                emitted.push_str(&expr);
                emitted.to_owned()
            },
            Expression::Use (m) => {
                let mut emitted = "#include \"".to_string();
                emitted.push_str(&self.match_module(m.to_string()));
                emitted.push_str(".h\"");
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
    pub fn compile_exe(&mut self, expressions: Vec<Expression>) -> String {
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
        self.writeln("// Executable autogenerated by the Boron compiler");
        self.writeln(&version);
        self.writeln(&datetime);
        self.writeln("");

        let (includes, structs, functions, code) = self.emit_block(expressions, None, false);

        // Emit #include statements
        self.writeln("#include <stdio.h>");
        self.writeln("#include <stdbool.h>");
        self.writeln(&includes);
        self.writeln("");
        
        // Emit header (functions + structs)
        self.writeln("// Structs begin here");
        self.writeln(&structs);
        self.writeln("// Functions begin here");
        self.writeln(&functions);

        // Emit main function
        self.writeln("int main(void) {");
        self.writeln(&code);
        self.writeln("return 0;\n}");

        self.code.to_owned()
    }

    /// Compiles a list of expressions into a C header file.
    pub fn compile_lib(&mut self, name: String, expressions: Vec<Expression>) -> String {
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
        self.writeln("// Library autogenerated by the Boron compiler");
        self.writeln(&version);
        self.writeln(&datetime);
        self.writeln("");

        // Only emit structs and functions
        let (includes, structs, functions, _) = self.emit_block(expressions, None, false);
        let upper_name = name.to_ascii_uppercase().replace("/", "_");

        let header_guard_start = format!("#ifndef {}\n#define {}", upper_name, upper_name);
        let header_guard_end = "#endif".to_string();

        // Emit header guard
        self.writeln(&header_guard_start);
        self.writeln("");

        // Emit #include statements
        self.writeln("#include <stdio.h>");
        self.writeln("#include <stdbool.h>");
        self.writeln("");
        self.writeln("// Additional includes begin here");
        self.writeln(&includes);
        self.writeln("");
        
        // Emit header (functions + structs)
        self.writeln("// Structs begin here");
        self.writeln(&structs);
        self.writeln("// Functions begin here");
        self.writeln(&functions);

        // Emit header guard
        self.writeln(&header_guard_end);

        self.code.to_owned()
    }
}