# The Boron Programming Language

Boron is a simple, light, readable, and efficient programming language.

# Boron Syntax

More information about Boron syntax is continuously forthcoming.  `FORMAL-GRAMMAR.md` provides an introduction to Boron grammar.

# The Boron Compiler

This repository holds the source code for the Boron compiler, `boron`.  `boron` compiles Boron source code (`.brn` files) to output C source files (`.c` files).

# Building `boron`

The Boron compiler is written fully in the Rust programming language and can be compiled with an up-to-date version of `cargo`.

## From Crates.io

Boron can be installed from the (crates.io)[https://crates.io] registry.

```
$ cargo install boron-lang
$ boron [input].brn
```

## From Source

Boron can also be built and executed from source using the following commands.

```
$ git clone https://github.com/hobbsbros/boron.git
$ cd boron
$ cargo build --release
$ ./boron [input].brn
```
