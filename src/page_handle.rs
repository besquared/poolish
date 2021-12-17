use std::sync::atomic::{
  AtomicI64, Ordering
};

/**
 *
 * A handle to a page
 *
 * When a handle is allocated we store the logical pid, the size class,
 *  and an atomic i64 ("handle") which stores either the pid if the page is on disk,
 *  or the virtual memory address of the page if it is in memory
 *
 * If a page is uninitialized then the handle will be zero
 * If a page is on disk (fizzled) then the handle will be negative
 * If a page is in memory (swizzled) then the handle will be positive
 *
 */

#[derive(Debug)]
pub struct PageHandle(i64, usize, AtomicI64);

impl PageHandle {
  pub fn pid(&self) -> i64 {
    self.0
  }

  pub fn class(&self) -> usize {
    self.1
  }

  pub fn handle(&self) -> &AtomicI64 {
    &self.2
  }

  pub fn value(&self) -> i64 {
    self.handle().load(Ordering::Acquire)
  }

  pub fn state(&self) -> PageHandleState {
    PageHandleState::new(&self)
  }

  pub fn fizzle(&mut self, pid: i64) -> i64 {
    self.handle().swap(pid, Ordering::SeqCst)
  }

  pub fn swizzle(&mut self, address: u64) -> i64 {
    self.handle().swap(address as i64, Ordering::SeqCst)
  }

  pub fn new(size: usize, pid: i64) -> Self {
    Self(pid, size, AtomicI64::from(pid))
  }
}

pub enum PageHandleState {
  UnInit,
  Fizzled(i64),
  Swizzled(i64)
}

impl PageHandleState {
  pub fn new(handle: &PageHandle) -> Self {
    let value = handle.value();

    if value < 0 {
      PageHandleState::Fizzled(value)
    } else if value > 0 {
      PageHandleState::Swizzled(value)
    } else {
      PageHandleState::UnInit
    }
  }
}

