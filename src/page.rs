mod page_latch;
mod read_guard;
mod read_opt_guard;
mod read_write_guard;

use anyhow::{ Result };
use core::hint::spin_loop;

use std::{
  io::{ Cursor, Write }
};

use crate::{ Frame, PageHandle };

//
// RwPage
// RsPage
// RoPage
//

pub use page_latch::*;
pub use read_guard::*;
pub use read_opt_guard::*;
pub use read_write_guard::*;

//
// A page is the entry point for reading and writing to virtual memory
//
// It consists of three parts
//
// A logical page identifier
// A versioned latch with a 48 bit version and a 16 bit state
// A page frame with the pointer and length of the memory allocation
//

#[derive(Debug)]
pub struct Page(u64, Frame);

impl Page {
  pub fn pid(&self) -> u64 {
    self.0
  }

  pub fn frame(&self) -> &Frame {
    &self.1
  }

  pub fn frame_mut(&mut self) -> &mut Frame {
    &mut self.1
  }

  pub fn latch(&self) -> Result<PageLatch> {
    let frame = self.frame();
    Ok(PageLatch::new(frame.latch_ref()?))
  }

  //
  // Three kinds of accessors
  //
  // try_read
  // try_read_opt
  // try_read_write
  //

  pub fn try_read_write(&mut self) -> Result<ReadWriteGuard> {
    let latch = self.latch()?;

    loop {
      let mut value = latch.value();
      let mut state = PageLatch::state(value);

      if PageLatch::is_open(state) {
        match latch.set_state(value, 1u8) {
          Err(_) => continue,
          Ok(_) => return Ok(ReadWriteGuard::new(self))
        }
      }

      while !PageLatch::is_open(state) {
        spin_loop();
        value = latch.value();
        state = PageLatch::state(value);
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

  pub fn try_alloc(cid: u8, pid: u64, mut frame: Frame) -> Result<Self> {
    let mut cursor = Cursor::new(frame.as_mut());

    cursor.write(&cid.to_le_bytes())?;
    cursor.write(&pid.to_le_bytes())?;
    cursor.write(&1u8.to_le_bytes())?;  // Dirty page
    cursor.write(&0u64.to_le_bytes())?; // Open latch

    Ok(Self(pid, frame))
  }

  pub fn try_fetch(handle: &PageHandle, frame: Frame) -> Result<Self> {
    Ok(Self(handle.pid(), frame))
  }
}