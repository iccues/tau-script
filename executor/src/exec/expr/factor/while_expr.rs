use frontend::parser::expr::factor::while_expr::WhileExpr;
use object::core::prelude::*;
use object::{ext::object_ext::ObjectExt, types::{primitive::bool::ObjBool, unit::undefined::Undefined}};
use crate::error::ExecutorResult;
use crate::exec::Exec;

impl Exec for WhileExpr {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        let mut result: Object = Undefined::new();
        while self.condition.exec(env)?.match_downcast::<ObjBool>()?.value {
            result = self.then_block.exec(env)?;
        }
        Ok(result)
    }
}
