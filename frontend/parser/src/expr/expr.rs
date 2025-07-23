use frontend_library::try_parse;
use frontend_library::error::{FrontendError, FrontendResult as Result};
use frontend_library::stream::peeker::Peeker;
use frontend_library::token::TokenBox;
use frontend_library::token::identifier::Identifier;
use frontend_library::token::operator::Operator;
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

    /// Binary expression
    Binary(BinaryExpr),

    /// Postfix expression
    Call(CallExpr),
    Dot(DotExpr),

    /// Factors
    Block(Block),
    If(IfExpr),
    While(WhileExpr),
    Literal(Literal),
    Tuple(TupleExpr),

    Identifier(String),
    UnaryExpr {
        operator: TokenBox<Operator>,
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn parse(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        Self::parse_binary(peeker)
    }

    fn parse_binary(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        BinaryExpr::parse(peeker)
    }

    pub fn parse_postfix(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        let mut first = Self::parse_factor(peeker)?;
        let mut changed = true;
        let mut t;
        while changed {
            (t, first) = CallExpr::try_parse(first, peeker)?;
            changed = t;
            (t, first) = DotExpr::try_parse(first, peeker)?;
            changed = changed || t;
        }
        Ok(first)
    }

    pub fn parse_factor(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        try_parse!(Block::parse(peeker));
        try_parse!(IfExpr::parse(peeker));
        try_parse!(WhileExpr::parse(peeker));
        try_parse!(Literal::parse(peeker));
        try_parse!(TupleExpr::parse_or_group(peeker));
        try_parse!(Self::parse_id(peeker));
        try_parse!(Self::parse_unary(peeker));
        Err(FrontendError::None)
    }

    fn parse_id(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        let id = peeker.eat_type::<Identifier>()?;
        Ok(Box::new(Expr::Identifier(id.name())))
    }

    fn parse_unary(peeker: &mut Peeker<TokenBox>) -> Result<Box<Expr>> {
        if peeker.peek()?
            .downcast::<Operator>()
            .is_ok_and(|o| o.is_unary())
        {
            let operator = peeker.eat_type::<Operator>()?;
            let expr = Self::parse(peeker)?;
            Ok(Box::new(Expr::UnaryExpr { operator, expr }))
        } else {
            Err(FrontendError::None)
        }
    }
}
