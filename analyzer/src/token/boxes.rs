use std::ops::Deref;
use std::rc::Rc;
use crate::error::{FrontendError, FrontendResult};
use crate::token::traits::Token;

#[derive(Debug)]
pub struct TokenBox<T: 'static + ?Sized + Token = dyn Token> {
    kind: Rc<T>,
}

impl TokenBox {
    pub fn new<T: 'static + Token>(kind: T) -> TokenBox {
        TokenBox { kind: Rc::new(kind) }
    }
}

impl<T: ?Sized + Token> TokenBox<T> {
    pub fn downcast<U: Token>(&self) -> FrontendResult<TokenBox<U>> {
        self.kind.clone().into_any()
            .downcast::<U>()
            .map(|c| TokenBox {
                kind: c,
            })
            .map_err(|_| FrontendError::DowncastFailed)
    }
}

impl<T: ?Sized + Token> Deref for TokenBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<T: ?Sized + Token> Clone for TokenBox<T> {
    fn clone(&self) -> Self {
        TokenBox {
            kind: self.kind.clone(),
        }
    }
}

impl<T: Token + 'static> From<TokenBox<T>> for TokenBox {
    fn from(b: TokenBox<T>) -> Self {
        TokenBox { kind: b.kind }
    }
}
