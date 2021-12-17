use crate::{ PageLatch };

#[derive(Debug)]
pub struct ExclusivePageGuard<'a>(&'a PageLatch<'a>);

impl<'a> ExclusivePageGuard<'a> {
  pub fn new(latch: &'a PageLatch) -> Self {
    Self(latch)
  }
}