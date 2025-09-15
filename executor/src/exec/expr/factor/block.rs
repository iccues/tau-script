use object::core::prelude::*;
use analyzer::parser::expr::factor::block::Block;
use object::types::unit::undefined::Undefined;

use crate::{error::ExecutorResult, exec::Exec};

impl Exec for Block {
    fn exec(&self, env: &Object) -> ExecutorResult<Object> {
        for stmt in self.statments.iter() {
            stmt.exec(env)?;
        }
        if let Some(end_value) = self.end_value.as_ref() {
            end_value.exec(env)
        } else {
            Ok(Undefined::new())
        }
    }
}
