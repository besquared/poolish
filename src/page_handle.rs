mod page_swip;

use std::sync::atomic::{
  AtomicUsize, Ordering
};

use crate::{ Frame };

pub use page_swip::*;

#[derive(Debug)]
pub struct PageHandle<'a>(&'a AtomicUsize);

impl<'a> From<&'a usize> for PageHandle<'a> {
  fn from(handle_ref: &'a usize) -> Self {
    Self(Self::make_handle(handle_ref))
  }
}

impl<'a> PageHandle<'a> {
  pub fn swip(&self) -> PageSWIP {
    PageSWIP::new(self.value())
  }

  pub fn swizzle(&mut self, address: usize) {
    match self.swip() {
      PageSWIP::Swizzled(_) => (),
      PageSWIP::Fizzled(_) => self.handle().swap(address, Ordering::SeqCst)
    }
  }

  // Private Accessors + Helpers

  fn handle(&self) -> &AtomicUsize {
    &self.0
  }

  fn value(&self) -> usize {
    self.handle().load(Ordering::Acquire)
  }

  fn make_handle(handle_ref: &usize) -> &AtomicUsize {
    unsafe {
      &(*(handle_ref as *const usize as *const AtomicUsize))
    }
  }
}
