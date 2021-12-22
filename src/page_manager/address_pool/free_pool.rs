use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct FreePool(VecDeque<usize>);

impl FreePool {
  pub fn pop_front(&mut self) -> Option<usize> {
    self.0.pop_front()
  }

  pub fn push_back(&mut self, address: usize) -> () {
    self.0.push_back(address)
  }

  pub fn push_front(&mut self, address: usize) -> () {
    self.0.push_front(address)
  }
}

impl Default for FreePool {
  fn default() -> Self {
    Self(Default::default())
  }
}