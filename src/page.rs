mod read_guard;
mod read_write_guard;
mod read_opt_guard;

use anyhow::{ Result };
use core::hint::spin_loop;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{Frame};

//
// RwPage
// RsPage
// RoPage
//

pub use read_guard::*;
pub use read_write_guard::*;
pub use read_opt_guard::*;

//
// A page is the entry point for reading and writing to virtual memory
//
// It consists of three parts
//
// A logical page identifier
// A versioned latch with a 48 bit version and a 16 bit state
// A page frame with the pointer and length of the memory allocation
//

pub const STATE_BITS: u64 = 16u64;
pub const STATE_MASK: u64 = 0x0000_0000_0000_FFFF;

#[derive(Debug)]
pub struct Page(u64, AtomicU64, Frame);

impl Page {
  pub fn pid(&self) -> u64 {
    self.0
  }

  pub fn latch(&self) -> &AtomicU64 {
    &self.1
  }

  pub fn latch_value(&self) -> u64 {
    self.latch().load(Ordering::Acquire)
  }

  pub fn frame(&self) -> &Frame {
    &self.2
  }

  pub fn frame_mut(&mut self) -> &mut Frame {
    &mut self.2
  }

  //
  // Three kinds of accessors
  //
  // try_read
  // try_read_opt
  // try_read_write
  //

  pub fn try_read_write(&mut self) -> Result<ReadWriteGuard> {
    loop {
      let mut value = self.latch_value();
      let mut state = Self::state(value);

      if Self::is_open(state) {
        match self.set_state(value, 1u8) {
          Err(_) => continue,
          Ok(_) => return Ok(ReadWriteGuard::new(self))
        }
      }

      while !Self::is_open(state) {
        spin_loop();
        value = self.latch_value();
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

  pub fn new(pid: u64, mut frame: Frame) -> Self {
    let latch = unsafe {
      let latch_ref = frame.latch_ref();
      Self::make_latch(std::mem::transmute(latch_ref))
    };

    Self(pid, latch, frame)
  }

  // Self methods

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
  fn make_latch(src: &mut u64) -> AtomicU64 {
    unsafe {
      (src as *mut u64 as *const AtomicU64).read_unaligned()
    }
  }

  fn with_state(value: u64, state: u64) -> u64 {
    (value & !STATE_MASK) | (state & STATE_MASK)
  }

  fn _with_version(value: u64, version: u64) -> u64 {
    (value & STATE_MASK) | (version << STATE_BITS)
  }

  fn set_state(&self, value: u64, state: u8) -> Result<u64, u64> {
    let new_value = Self::with_state(value, u64::from(state));
    self.latch().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }
}