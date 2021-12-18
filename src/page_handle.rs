use std::sync::atomic::{AtomicUsize, Ordering};
use crate::Frame;

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

#[derive(Debug)]
pub struct PageHandle(u8, u64, AtomicUsize);

impl PageHandle {
  pub fn pid(&self) -> u64 {
    self.1
  }

  pub fn class(&self) -> u8 {
    self.0
  }

  pub fn handle(&self) -> &AtomicUsize {
    &self.2
  }

  pub fn value(&self) -> usize {
    self.handle().load(Ordering::Acquire)
  }

  pub fn state(&self) -> PageHandleState {
    PageHandleState::new(&self)
  }

  pub fn fizzle(&mut self) -> usize {
    self.handle().swap(0, Ordering::SeqCst)
  }

  pub fn swizzle(&mut self, frame: &Frame) -> usize {
    let address = frame.as_ref().as_ptr();
    self.handle().swap(address as usize, Ordering::SeqCst)
  }

  // Constructors

  pub fn new(class: u8, pid: u64) -> Self {
    Self(class, pid, AtomicUsize::from(0))
  }
}

pub enum PageHandleState {
  Fizzled(u64),
  Swizzled(usize)
}

impl PageHandleState {
  pub fn new(handle: &PageHandle) -> Self {
    let pid = handle.pid();
    let value = handle.value();

    if value == 0 {
      PageHandleState::Fizzled(pid)
    } else {
      PageHandleState::Swizzled(value)
    }
  }
}

