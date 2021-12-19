use anyhow::Result;
use std::io::Read;
use crate::{Frame, Page};

#[derive(Debug)]
pub struct ReadWriteGuard<'a>(&'a mut Page);

impl<'a> ReadWriteGuard<'a> {
  pub fn frame(&self) -> &Frame {
    self.0.frame()
  }

  pub fn frame_mut(&mut self) -> &mut Frame {
    self.0.frame_mut()
  }

  pub fn write<R: Read>(&mut self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.frame_mut().write(offset, len, data)?)
  }

  pub fn new(page: &'a mut Page) -> Self {
    Self(page)
  }
}