use anyhow::{ Result };
use core::hint::spin_loop;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::{BufferFrame, ExclusivePageGuard};

//
// A versioned latch with a 48 bit version and a 16 bit state
//

#[derive(Debug)]
pub struct PageLatch<'a>(BufferFrame, &'a AtomicU64);

pub const STATE_BITS: u64 = 12u64;
pub const STATE_MASK: u64 = 0x0000_0000_0000_0FFF;

impl<'a> PageLatch<'a> {
  pub fn frame(&self) -> &BufferFrame {
    &self.0
  }

  pub fn frame_mut(&mut self) -> &mut BufferFrame {
    &mut self.0
  }

  pub fn value(&self) -> &AtomicU64 {
    self.1
  }

  pub fn load(&self) -> u64 {
    self.value().load(Ordering::Acquire)
  }

  pub fn acquire_exclusive(&'a mut self) -> Result<ExclusivePageGuard> {
    loop {
      let mut value = self.load();
      let mut state = Self::state(value);

      if Self::is_open(state) {
        match self.set_state(value, 1u8) {
          Err(_) => continue,
          Ok(_) => return Ok(ExclusivePageGuard::new(self))
        }
      }

      while !Self::is_open(state) {
        spin_loop();
        value = self.load();
        state = Self::state(value);
      }
    }
  }

  // pub fn lock_shared(&mut self) -> Option<SharedPageGuard<'a>> {
  //   // If the latch is open then cas(0, 2)
  //   // If the latch is shared then cas(shared_count, shared_count + 1)
  //   // If the latch is exclusive then wait for it to be unlocked, loop
  //   None
  // }

  // pub fn lock_optimistic(&mut self) -> Option<OptimisticPageGuard<'a>> {
  //   // If the latch is open then return page guard
  //   // If the latch is shared then return page guard
  //   // If the latch is exclusive then wait for it to be unlocked, loop
  //
  //   None
  // }

  // Constructors

  pub fn new(mut frame: BufferFrame) -> Self {
    let value = unsafe {
      let pointer = frame.latch_ref();
      Self::make_atomic_u64(std::mem::transmute(pointer))
    };

    Self(frame, value)
  }

  // Self methods

  pub fn state(value: u64) -> u64 {
    value & STATE_MASK
  }

  pub fn version(value: u64) -> u64 {
    value >> STATE_BITS
  }

  pub fn with_state(value: u64, state: u64) -> u64 {
    (value & !STATE_MASK) | (state & STATE_MASK)
  }

  pub fn with_version(value: u64, version: u64) -> u64 {
    (value & STATE_MASK) | (version << STATE_BITS)
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

  fn make_atomic_u64(src: &mut u64) -> &AtomicU64 {
    unsafe {
      &*(src as *mut u64 as *const AtomicU64)
    }
  }

  fn set_state(&self, value: u64, state: u8) -> Result<u64, u64> {
    let new_value = Self::with_state(value, u64::from(state));
    self.value().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }
}