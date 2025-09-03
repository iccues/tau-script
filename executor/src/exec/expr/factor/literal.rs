use object::core::prelude::*;
use object::types::primitive::numbers::ObjI64;
use frontend::parser::expr::factor::literal::Literal;
use object::ext::core_type::string::ObjString;
use crate::error::ExecutorResult;
use crate::exec::Exec;


impl Exec for Literal {
    fn exec(&self, _env: &Object) -> ExecutorResult<Object> {
        match self {
            Literal::String(literal) => {
                Ok(ObjString::new(literal.clone()))
            }
            Literal::Integer(literal) => {
                Ok(ObjI64::new(literal.parse().unwrap()))
            }
            _ => unimplemented!(),
        }
    }
}
