use std::sync::atomic::{
  AtomicU64, Ordering
};

pub const STATE_BITS: u64 = 16u64;
pub const STATE_MASK: u64 = 0x0000_0000_0000_FFFF;


#[derive(Debug)]
pub struct PageLatch<'a>(&'a AtomicU64);

impl<'a> PageLatch<'a> {
  fn latch(&self) -> &AtomicU64 {
    self.0
  }

  pub fn value(&self) -> u64 {
    self.latch().load(Ordering::Acquire)
  }

  pub fn set_state(&self, value: u64, state: u8) -> Result<u64, u64> {
    let new_value = Self::with_state(value, u64::from(state));
    self.latch().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }

  pub fn new(latch_ref: &'a u8) -> Self {
    Self(make_latch(unsafe { std::mem::transmute(latch_ref) }))
  }
}

impl<'a> PageLatch<'a> {
  pub fn state(value: u64) -> u64 {
    value & STATE_MASK
  }

  pub fn version(value: u64) -> u64 {
    value >> STATE_BITS
  }

  pub fn is_open(state: u64) -> bool {
    state == 0
  }

  pub fn is_shared(state: u64) -> bool {
    state > 1
  }

  pub fn is_exclusive(state: u64) -> bool {
    state == 1
  }

  // Helper methods

  fn with_state(value: u64, state: u64) -> u64 {
    (value & !STATE_MASK) | (state & STATE_MASK)
  }

  fn _with_version(value: u64, version: u64) -> u64 {
    (value & STATE_MASK) | (version << STATE_BITS)
  }
}

fn make_latch(latch_ref: &u64) -> &AtomicU64 {
  unsafe {
    &(*(latch_ref as *const u64 as *const AtomicU64))
  }
}
