use std::{
  collections::VecDeque,
  sync::atomic::{ AtomicUsize, Ordering }
};

use parking_lot::{Mutex, MutexGuard, RawMutex};

pub struct PageIdent(Mutex<AtomicUsize>, VecDeque<usize>);

impl PageIdent {
  pub fn next(&mut self) -> usize {
    let guard = self.counter().lock();
    match self.free_ids_mut().pop_front() {
      Some(free_id) => free_id,
      None => generate_id(&guard)
    }
  }

  pub fn free(&mut self, pid: usize) {
    self.counter().lock();
    self.free_ids_mut().push_back(pid)
  }

  pub fn new() -> Self {
    Self(AtomicUsize::from(1), VecDeque::new())
  }

  // Private Helpers

  fn counter(&self) -> &Mutex<AtomicUsize> {
    &self.0
  }

  fn free_ids_mut(&mut self) -> &mut VecDeque<usize> {
    &mut self.1
  }

  fn generate_id(guard: &MutexGuard<AtomicUsize>) -> usize {
    guard.fetch_add(2, Ordering::SeqCst)
  }
}