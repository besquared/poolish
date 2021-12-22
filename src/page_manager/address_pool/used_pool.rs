use std::collections::{ BTreeSet };

#[derive(Clone, Debug)]
pub struct UsedPool(BTreeSet<usize>);

impl UsedPool {
  pub fn insert(&mut self, address: usize) -> bool {
    self.0.insert(address)
  }

  pub fn remove(&mut self, address: usize) -> Option<usize> {
    if self.0.remove(&address) {
      Some(address)
    } else {
      None
    }
  }
}

impl Default for UsedPool {
  fn default() -> Self {
    Self(Default::default())
  }
}