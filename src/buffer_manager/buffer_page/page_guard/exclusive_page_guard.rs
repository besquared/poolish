use anyhow::Result;
use std::io::Read;
use crate::{BufferFrame, PageLatch};

#[derive(Debug)]
pub struct ExclusivePageGuard<'a>(&'a mut PageLatch<'a>);

impl<'a> ExclusivePageGuard<'a> {
  pub fn new(latch: &'a mut PageLatch<'a>) -> Self {
    Self(latch)
  }

  pub fn frame(&self) -> &BufferFrame {
    self.0.frame()
  }

  pub fn write<R: Read>(&mut self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.0.frame_mut().write(offset, len, data)?)
  }
}