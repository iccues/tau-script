use error::Result;
use stream::peeker::Peeker;
use token::{operator::Operator, TokenBox};

use crate::expr::expr::Expr;

#[derive(Debug)]
pub struct Block {
    pub statments: Vec<Box<Expr>>,
    pub end_value: Option<Box<Expr>>,
}

impl Block {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        peeker.eat_eq(&Operator::OpenBrace)?;

        let mut statments = Vec::new();
        let mut end_value = None;

        while peeker.eat_eq(&Operator::CloseBrace).is_err() {
            let statment = Expr::parse(peeker)?;
            if peeker.eat_eq(&Operator::Semi).is_err() {
                end_value = Some(statment);
                peeker.eat_eq(&Operator::CloseBrace)?;
                break;
            }
            statments.push(statment);
        }

        Ok(Box::new(Expr::Block(Block {
            statments,
            end_value,
        })))
    }
}
