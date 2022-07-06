//! Provides a parselet for assignments.


use crate::{
    Parser,
    Expression,
    Token,
    TokenType,
    Tokenizer,
    infix::InfixParselet,
};


/// Provides a prefix parselet for assignments.
pub struct AssignmentParselet;

impl InfixParselet for AssignmentParselet {
    /// Parses an assignment into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, left: Expression, token: Token) -> Expression {
        if token.get_type() != TokenType::Assignment {
            todo!()
        }

        if let Expression::Declaration {
            datatype: d,
            identifier: id,
        } = left {
            // Evaluate the right hand side of the assignment
            let right_hand_side: Expression = match parser.parse(tokenizer) {
                Some(r) => r,
                None => todo!(),
            };
            // Place the right hand side into an instance of `Expression`
            Expression::Assignment {
                datatype: d,
                identifier: id,
                value: Box::new(right_hand_side),
            }
        } else {
            dbg!(&left);
            todo!()
        }
    }
}