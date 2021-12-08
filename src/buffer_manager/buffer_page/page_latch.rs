use std::sync::atomic::{AtomicU64, Ordering};

//
// A versioned latch with a 48 bit version and a 16 bit state
//

pub struct PageLatch<'a>(&'a AtomicU64);

pub const STATE_MASK: u64 = 0x0000_0000_0000_00FF;

impl<'a> PageLatch<'a> {
  pub fn version(&self) -> u64 {
    self.0.load(Ordering::SeqCst) >> 8
  }

  pub fn state(&self) -> u64 {
    self.0.load(Ordering::SeqCst) & STATE_MASK
  }

  pub fn is_locked(&self) -> bool {
    self.state() > 0
  }

  pub fn is_shared(&self) -> bool {
    self.state() > 1
  }

  pub fn is_exclusive(&self) -> bool {
    self.state() == 1
  }

  pub fn shared_count(&self) -> Option<u64> {
    if self.is_shared() {
      Some(self.state())
    } else {
      None
    }
  }

  // Ways to basically latch this in various ways

  pub fn new(latch: &'a AtomicU64) -> Self {
    Self(latch)
  }
}