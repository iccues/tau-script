use crate::signal_table::func::{block::Block, if_expr::IfExpr, literal::Literal};

use error::{NoneError, Result};
use lexer::{
    stream::peekable::cursor::Cursor,
    token::{identifier::Identifier, operator::Operator, ComplexBox, TokenBox}
};
use super::{binary_expr::BinaryExpr, call_expr::CallExpr, dot_expr::DotExpr, tuple::TupleExpr, while_expr::WhileExpr};
use error::try_parse;

#[derive(Debug)]
pub enum Expr {

    /// Postfixe expression
    Call(CallExpr),
    Dot(DotExpr),

    /// Binary expression
    Binary(BinaryExpr),

    /// Factors
    Block(Block),
    If(IfExpr),
    While(WhileExpr),
    Literal(Literal),
    Tuple(TupleExpr),

    Identifier(String),
    UnaryExpr {
        operator: ComplexBox<Operator>,
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        Self::parse_postfix(cursor)
    }

    fn parse_postfix(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let mut first = Self::parse_binary(cursor)?;
        let mut changed = true;
        let mut t;
        while changed {
            (t, first) = CallExpr::try_parse(first, cursor)?;
            changed = t;
            (t, first) = DotExpr::try_parse(first, cursor)?;
            changed = changed || t;
        }
        Ok(first)
    }

    fn parse_binary(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        BinaryExpr::parse(cursor)
    }

    pub fn parse_factor(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        try_parse!(Block::parse(cursor));
        try_parse!(IfExpr::parse(cursor));
        try_parse!(WhileExpr::parse(cursor));
        try_parse!(Literal::parse(cursor));
        try_parse!(TupleExpr::parse_or_group(cursor));
        try_parse!(Self::parse_id(cursor));
        try_parse!(Self::parse_unary(cursor));
        Err(NoneError.into())
    }

    fn parse_id(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let id = cursor.eat_type::<Identifier>()?;
        Ok(Box::new(Expr::Identifier(id.name())))
    }

    fn parse_unary(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        if cursor.peek()?
            .downcast::<Operator>()
            .is_some_and(|o| o.is_unary())
        {
            let operator = cursor.eat_type::<Operator>()?;
            let expr = Self::parse(cursor)?;
            Ok(Box::new(Expr::UnaryExpr { operator, expr }))
        } else {
            Err(NoneError.into())
        }
    }
}
