use parking_lot::{ Mutex };

use std::{
  collections::VecDeque,
  sync::atomic::{ AtomicUsize, Ordering }
};

#[derive(Debug)]
pub struct PageIdPool(AtomicUsize, Mutex<VecDeque<usize>>);

impl PageIdPool {
  pub fn next(&self) -> usize {
    match self.free_ids().lock().pop_front() {
      Some(free_id) => free_id,
      None => self.generate_id()
    }
  }

  pub fn free(&mut self, pid: usize) {
    self.free_ids().lock().push_back(pid)
  }

  pub fn new() -> Self {
    Self(AtomicUsize::from(1), Mutex::new(VecDeque::new()))
  }

  // Private Helpers

  fn counter(&self) -> &AtomicUsize {
    &self.0
  }

  fn free_ids(&self) -> &Mutex<VecDeque<usize>> {
    &self.1
  }

  fn generate_id(&self) -> usize {
    self.counter().fetch_add(2, Ordering::SeqCst)
  }
}