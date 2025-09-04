use object::core::prelude::*;
use object::types::primitive::bool::ObjBool;
use object::types::primitive::float::ObjF64;
use object::types::primitive::integer::ObjI64;
use frontend::parser::expr::factor::literal::Literal;
use object::ext::core_type::string::ObjString;
use crate::error::ExecutorResult;
use crate::exec::Exec;


impl Exec for Literal {
    fn exec(&self, _env: &Object) -> ExecutorResult<Object> {
        match self {
            Literal::String(value) => {
                Ok(ObjString::new(value.clone()))
            }
            Literal::Integer(value) => {
                Ok(ObjI64::new(*value))
            }
            Literal::Float(value) => {
                Ok(ObjF64::new(*value))
            }
            Literal::Bool(value) => {
                Ok(ObjBool::new(*value))
            }
        }
    }
}
