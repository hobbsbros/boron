# Defining a Formal Grammar for Boron

## Literals and Identifiers

`DIGIT := '0' ... '9'`

`ALPHA := 'A' | 'B' | ... 'Z' | 'a' | 'b' | ... | 'z' | '_'`

`INT := DIGIT+`

`FLOAT := DIGIT+ "." DIGIT*`

`NUMBER := INT | FLOAT`

`BOOLEAN := "true" | "false"`

`IDENTIFIER := ALPHA ( ALPHA | DIGIT )*`

`LITERAL := NUMBER | BOOLEAN`

## Operators

`BINARY-OPERATOR := '+' | '-' | '*' | '/'`

`UNARY-OPERATOR := '!' | '-'`

## Statements and Expressions

`BINARY-OP := EXPRESSION BINARY-OPERATOR EXPRESSION`

`UNARY-OP := UNARY-OPERATOR EXPRESSION`

`ASSIGNMENT := IDENTIFIER ":" ( LITERAL | BINARY-OP | UNARY-OP )`

`FN-CALL := IDENTIFIER "(" ( EXPRESSION "," )* ")"`

`EXPRESSION := IDENTIFIER | ASSIGNMENT | LITERAL | FN-CALL | BINARY-OP | UNARY-OP`