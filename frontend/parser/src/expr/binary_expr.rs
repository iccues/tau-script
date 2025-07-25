use frontend_library::stream::peeker::Peeker;
use frontend_library::token::{operator::Operator, TokenBox};

use crate::expr::expr::Expr;
use frontend_library::error::FrontendResult as Result;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: TokenBox<Operator>,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        let mut factors = Vec::new();
        let mut operators: Vec<TokenBox<Operator>> = Vec::new();

        factors.push(Expr::parse_postfix(peeker)?);

        while peeker
            .peek()?
            .downcast::<Operator>()
            .is_ok_and(|o| o.priority() >= 0)
        {

            let operator = peeker.eat_type::<Operator>()?;

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
            factors.push(Expr::parse_postfix(peeker)?);
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
