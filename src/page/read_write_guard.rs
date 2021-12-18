use anyhow::Result;
use std::io::Read;
use crate::{Frame, Page};

#[derive(Debug)]
pub struct ReadWriteGuard<'a>(&'a mut Page);

impl<'a> ReadWriteGuard<'a> {
  pub fn new(page: &'a mut Page) -> Self {
    Self(page)
  }

  pub fn frame(&self) -> &Frame {
    self.0.frame()
  }

  pub fn write<R: Read>(&mut self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.0.frame_mut().write(offset, len, data)?)
  }
}