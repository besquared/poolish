use std::sync::atomic::{AtomicI64};

//
// A handle to a logical page that may reside either in-memory or on-disk
//

pub struct PageHandle(AtomicI64);

impl PageHandle {
  pub fn new(pid: u64) -> Self {
    Self(AtomicI64::from(pid as i64))
  }
}
