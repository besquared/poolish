use std::sync::atomic::{
  AtomicUsize, Ordering
};

use crate::{ Frame, PageClass };

/**
 *
 * A handle to a page
 *
 * When a handle is allocated we store the logical pid, the size class,
 *  and an atomic i64 ("handle") which stores either the pid if the page is on disk,
 *  or the virtual memory address of the page if it is in memory
 *
 * If a page is on disk (fizzled) then the handle will be zero
 * If a page is in memory (swizzled) then the handle will be positive
 *
 */

//
// todo: We should make this work with tagged pointers instead
//  on 64 bit systems the least significant (right-most) 3 bits aren't used
//  when a pointer is de-referenced those bytes must be 0 but at rest they can
//  store some type information.
//
// For our system we'll use the least significant bit set to 1 to indicate that
//  the value is a pid and not a pointer. This means that pids are only ever odd
//  numbers. If a value is odd it's a pid (value & 1 == 1) otherwise we know that
//  it's a pointer and can be safely de-referenced.
//

#[derive(Debug)]
pub struct PageHandle(u64, PageClass, AtomicUsize);

impl PageHandle {
  pub fn pid(&self) -> u64 {
    self.0
  }

  pub fn cid(&self) -> u8 {
    self.class().id()
  }

  pub fn class(&self) -> &PageClass {
    &self.1
  }

  pub fn handle(&self) -> &AtomicUsize {
    &self.2
  }

  pub fn value(&self) -> usize {
    self.handle().load(Ordering::Acquire)
  }

  pub fn state(&self) -> PageHandleState {
    PageHandleState::new(self.pid(), self.value())
  }

  pub fn fizzle(&mut self) -> usize {
    self.handle().swap(0, Ordering::SeqCst)
  }

  pub fn swizzle(&mut self, frame: &Frame) -> usize {
    let frame_ptr = frame.as_ref().as_ptr();
    self.handle().swap(frame_ptr as usize, Ordering::SeqCst)
  }

  // Constructors

  pub fn new(pid: u64, class: PageClass) -> Self {
    Self(pid, class, AtomicUsize::from(0))
  }
}

pub enum PageHandleState {
  Fizzled(u64),
  Swizzled(usize)
}

impl PageHandleState {
  pub fn new(pid: u64, handle: usize) -> Self {
    if handle == 0 {
      PageHandleState::Fizzled(pid)
    } else {
      PageHandleState::Swizzled(handle)
    }
  }
}

