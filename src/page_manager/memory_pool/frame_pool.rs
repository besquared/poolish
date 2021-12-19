use std::collections::VecDeque;

use crate::{ Frame };

#[derive(Clone, Debug)]
pub struct FramePool(usize, VecDeque<Frame>);

impl FramePool {
  pub fn new() -> Self {
    Self(0, VecDeque::new())
  }

  pub fn size(&self) -> usize {
    self.0
  }

  pub fn pop_front(&mut self) -> Option<Frame> {
    match self.1.pop_front() {
      None => None,
      Some(frame) => {
        self.0 -= frame.len();
        Some(frame)
      }
    }
  }

  pub fn push_back(&mut self, frame: Frame) -> () {
    self.0 += frame.len();
    self.1.push_back(frame)
  }

  pub fn push_front(&mut self, frame: Frame) -> () {
    self.0 += frame.len();
    self.1.push_front(frame)
  }

  pub fn remove(&mut self, idx: usize) -> Option<Frame> {
    match self.1.remove(idx) {
      None => None,
      Some(frame) => {
        self.0 -= frame.len();
        Some(frame)
      }
    }
  }
}