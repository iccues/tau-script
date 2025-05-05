use lexer::{stream::peekable::cursor::Cursor, token::{operator::Operator, ComplexBox, TokenBox}};

use super::expr::Expr;
use error::Result;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: ComplexBox<Operator>,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let mut factors = Vec::new();
        let mut operators: Vec<ComplexBox<Operator>> = Vec::new();

        factors.push(Expr::parse_factor(cursor)?);

        while cursor
            .peek()?
            .downcast::<Operator>()
            .is_some_and(|o| o.priority() >= 0)
        {

            let operator = cursor.eat_type::<Operator>()?;

            while !operators.is_empty()
                && operator.priority() <= operators.last().unwrap().priority()
            {
                let right = factors.pop().unwrap();
                let left = factors.pop().unwrap();
                let operator = operators.pop().unwrap();
                factors.push(Box::new(Expr::Binary(BinaryExpr {
                    left,
                    operator,
                    right,
                })));
            }

            operators.push(operator);
            factors.push(Expr::parse_factor(cursor)?);
        }

        while !operators.is_empty() {
            let right = factors.pop().unwrap();
            let left = factors.pop().unwrap();
            let operator = operators.pop().unwrap();
            factors.push(Box::new(Expr::Binary(BinaryExpr {
                left,
                operator,
                right,
            })));
        }

        Ok(factors.pop().unwrap())
    }
}
