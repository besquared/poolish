use std::{
  collections::VecDeque,
  sync::atomic::{ AtomicUsize, Ordering }
};

pub struct PageIdent(AtomicUsize, VecDeque<usize>);

impl PageIdent {
  pub fn next(&mut self) -> usize {
    match self.free_ids_mut().pop_front() {
      Some(free_id) => free_id,
      None => self.generate_id()
    }
  }

  pub fn free(&mut self, pid: usize) {
    self.free_ids_mut().push_back(pid)
  }

  pub fn new() -> Self {
    Self(AtomicUsize::from(1), VecDeque::new())
  }

  // Private Helpers

  fn counter_mut(&mut self) -> &mut AtomicUsize {
    &mut self.0
  }

  fn free_ids_mut(&mut self) -> &mut VecDeque<usize> {
    &mut self.1
  }

  fn generate_id(&mut self) -> usize {
    self.counter_mut().fetch_add(2, Ordering::SeqCst)
  }
}