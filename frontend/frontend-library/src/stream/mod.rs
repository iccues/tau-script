use peeker::Peeker;

use crate::error::FrontendResult as Result;

pub mod peeker;

pub trait Stream {
    type Item: Clone;

    fn next(&mut self) -> Result<Self::Item>;
    fn nexts(&mut self, n: usize) -> Result<Vec<Self::Item>> {
        let mut items = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(self.next()?);
        }
        Ok(items)
    }

    fn peeker(self) -> Peeker<Self::Item> where Self: Sized + 'static {
        Peeker::new(self)
    }
}
