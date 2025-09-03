pub mod expr;
pub mod stmt;


use object::core::prelude::*;
use crate::error::ExecutorResult;


pub trait Exec {
    fn exec(&self, env: &Object) -> ExecutorResult<Object>;
}
