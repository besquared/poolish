mod shared_page_guard;
mod exclusive_page_guard;
mod optimistic_page_guard;

use anyhow::{ Result };
use core::hint::spin_loop;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{Frame};

//
// RwPage
// RsPage
// RoPage
//

pub use shared_page_guard::*;
pub use exclusive_page_guard::*;
pub use optimistic_page_guard::*;

//
// A page is the entry point for reading and writing to virtual memory
//
// It consists of three parts
//
// A logical page identifier
// A page frame with the loc/len of the page allocation
// A versioned latch with a 48 bit version and a 16 bit state
//

pub const STATE_BITS: u64 = 16u64;
pub const STATE_MASK: u64 = 0x0000_0000_0000_FFFF;

#[derive(Debug)]
pub struct Page(i64, Frame, AtomicU64);

impl Page {
  pub fn pid(&self) -> i64 {
    self.0
  }

  pub fn frame(&self) -> &Frame {
    &self.1
  }

  pub fn frame_mut(&mut self) -> &mut Frame {
    &mut self.1
  }

  pub fn latch(&self) -> &AtomicU64 {
    &self.2
  }

  pub fn latch_value(&self) -> u64 {
    self.latch().load(Ordering::Acquire)
  }

  pub fn acquire_exclusive(&mut self) -> Result<ExclusivePageGuard> {
    loop {
      let mut value = self.latch_value();
      let mut state = Self::state(value);

      if Self::is_open(state) {
        match self.set_state(value, 1u8) {
          Err(_) => continue,
          Ok(_) => return Ok(ExclusivePageGuard::new(self))
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

  pub fn new(pid: i64, mut frame: Frame) -> Self {
    let value = unsafe {
      let pointer = frame.latch_ref();
      Self::make_atomic_u64(std::mem::transmute(pointer))
    };

    Self(pid, frame, value)
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
  fn make_atomic_u64(src: &mut u64) -> AtomicU64 {
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