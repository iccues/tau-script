use object::core::prelude::*;
use object::{ext::object_ext::ObjectExt, types::{primitive::bool::ObjBool, unit::undefined::Undefined}};
use crate::error::ExecutorResult;
use analyzer::parser::expr::factor::if_expr::IfExpr;
use crate::exec::Exec;

impl Exec for IfExpr {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        let condition = self.condition.exec(env)?;
        if condition.match_downcast::<ObjBool>()?.value {
            self.then_block.exec(env)
        } else {
            if let Some(else_block) = self.else_block.as_ref() {
                else_block.exec(env)
            } else {
                Ok(Undefined::new())
            }
        }
    }
}
