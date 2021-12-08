use std::sync::atomic::{AtomicI64, Ordering};

//
// A handle to a logical page that may reside either in-memory or on-disk
//

#[derive(Debug)]
pub struct PageHandle(AtomicI64);

impl PageHandle {
  pub fn value(&self) -> i64 {
    self.0.load(Ordering::SeqCst)
  }

  pub fn is_fizzled(&self) -> bool {
    self.value().is_negative()
  }

  pub fn is_swizzled(&self) -> bool {
    self.value().is_positive()
  }

  pub fn fizzle(&mut self, pid: i64) -> i64 {
    self.0.swap(pid, Ordering::SeqCst)
  }

  pub fn swizzle(&mut self, address: &[u8]) -> i64 {
    self.0.swap(address.as_ptr() as i64, Ordering::SeqCst)
  }

  pub fn new(pid: i64) -> Self {
    Self(AtomicI64::from(pid))
  }
}
