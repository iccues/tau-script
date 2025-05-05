use error::{try_parse, NoneError};
use lexer::stream::peekable::cursor::Cursor;
use lexer::token::{ComplexBox, TokenBox};
use lexer::token::identifier::Identifier;
use lexer::token::operator::Operator;
use crate::expr::binary_expr::BinaryExpr;
use crate::expr::factor::block::Block;
use crate::expr::factor::if_expr::IfExpr;
use crate::expr::factor::literal::Literal;
use crate::expr::factor::tuple::TupleExpr;
use crate::expr::factor::while_expr::WhileExpr;
use crate::expr::postfix::call_expr::CallExpr;
use crate::expr::postfix::dot_expr::DotExpr;

#[derive(Debug)]
pub enum Expr {

    /// Postfix expression
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
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> error::Result<Box<Expr>> {
        Self::parse_postfix(cursor)
    }

    fn parse_postfix(cursor: &mut Cursor<TokenBox>) -> error::Result<Box<Expr>> {
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

    fn parse_binary(cursor: &mut Cursor<TokenBox>) -> error::Result<Box<Expr>> {
        BinaryExpr::parse(cursor)
    }

    pub fn parse_factor(cursor: &mut Cursor<TokenBox>) -> error::Result<Box<Expr>> {
        try_parse!(Block::parse(cursor));
        try_parse!(IfExpr::parse(cursor));
        try_parse!(WhileExpr::parse(cursor));
        try_parse!(Literal::parse(cursor));
        try_parse!(TupleExpr::parse_or_group(cursor));
        try_parse!(Self::parse_id(cursor));
        try_parse!(Self::parse_unary(cursor));
        Err(NoneError.into())
    }

    fn parse_id(cursor: &mut Cursor<TokenBox>) -> error::Result<Box<Expr>> {
        let id = cursor.eat_type::<Identifier>()?;
        Ok(Box::new(Expr::Identifier(id.name())))
    }

    fn parse_unary(cursor: &mut Cursor<TokenBox>) -> error::Result<Box<Expr>> {
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