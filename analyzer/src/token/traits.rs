use std::fmt::Debug;
use std::any::Any;
use std::rc::Rc;

pub trait Token: TokenExt + Debug {}

pub trait TokenExt {
    fn eq(&self, other: &dyn Token) -> bool;

    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Rc<Self>) -> Rc<dyn Any>;
}

impl<T: PartialEq + Any + Token> TokenExt for T {
    fn eq(&self, other: &dyn Token) -> bool {
        if let Some(other) = other.as_any().downcast_ref() {
            self == other
        }
        else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
}
