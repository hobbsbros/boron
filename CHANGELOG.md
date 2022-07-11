# Changelog

## Version 0.17.1

Fixed a bug causing successive variable assignments to halt parsing.

## Version 0.17.0

Changed struct initialization syntax from `struct MyStruct x {}` to `MyStruct x: {}`. 

## Version 0.16.0

Added support for C-like struct definition and initialization.

## Version 0.15.1

Fixed a bug preventing compilation of if & while statements.

## Version 0.15.0

Fixed variable scoping (bumped from v0.13.0 release).

## Version 0.14.1

Fixed a bug preventing the tokenizer from working on Windows systems due to `CR` characters.

## Version 0.14.0

Modified the emitter to emit dates in 4-digit year format (`2022` vs `22`).

## Version 0.13.2

Fixed an issue causing ternary expressions not to parse properly.

## Version 0.13.1

Fixed an issue causing debug information to be printed to stdout.

## Version 0.13.0

Added support for ternary operator `? |`.

~~Fixed variable scoping.~~ (bumped to a later release)

Expanded changelog back to version 0.6.0.

## Version 0.12.0

Added README.md and installation instructions.

## Version 0.11.0

Fixed a bug in the emitter causing semicolons after while loops and if statements.

## Version 0.10.0

Added a precedence table to the parser to correct for errors in parsing binary operations.

## Version 0.9.0

Added if/else statements.

## Version 0.8.0

Added comparison and unary operators

## Version 0.7.0

Implemented proper error handling.

## Version 0.6.0

Original release of compiler; overhauled previous code and implemented tokenizer, parser, and emitter.