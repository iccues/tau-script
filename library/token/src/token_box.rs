use std::ops::Deref;
use std::rc::Rc;
use error::NoneError;
use crate::token_trait::Token;

#[derive(Debug)]
pub enum TokenBox<T: 'static + ?Sized + Token = dyn Token> {
    Rc(Rc<T>),
    Ref(&'static T),
}

impl<T: ?Sized + Token> TokenBox<T> {
    pub fn downcast<U: Token>(&self) -> error::Result<TokenBox<U>> {
        match self {
            TokenBox::Rc(r) => {
                r.clone().into_any()
                    .downcast::<U>()
                    .ok()
                    .map(|c| TokenBox::Rc(c))
            }
            TokenBox::Ref(r) => {
                r.as_any()
                    .downcast_ref::<U>()
                    .map(|r| TokenBox::Ref(r))
            }
        }.ok_or(NoneError.into())
    }
}

impl<T: ?Sized + Token> Deref for TokenBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            TokenBox::Rc(b) => b,
            TokenBox::Ref(r) => r,
        }
    }
}

impl<T: ?Sized + Token> Clone for TokenBox<T> {
    fn clone(&self) -> Self {
        match self {
            TokenBox::Rc(b) => TokenBox::Rc(b.clone()),
            TokenBox::Ref(r) => TokenBox::Ref(r),
        }
    }
}
