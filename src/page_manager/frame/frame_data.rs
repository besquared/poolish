// This is like the others bug has a reference to the data instead

use anyhow::{
  Result
};

use std::{
  io::{ Read, Write }
};

pub struct FrameData<'a>(&'a mut [u8]);

impl<'a> From<&'a mut [u8]> for FrameData<'a> {
  fn from(data: &'a mut [u8]) -> Self {
    Self(data)
  }
}

impl<'a> FrameData<'a> {
  fn as_ref(&self) -> &[u8] {
    self.0
  }

  fn as_mut(&mut self) -> &mut [u8] {
    self.0
  }

  // todo: if dest is longer than the frame then only write up to the end of the frame
  pub fn try_read<D: Write>(&self, offset: usize, len: usize, dst: &mut D) -> Result<usize> {
    Ok(dst.write(&self.as_ref()[offset..offset + len])?)
  }

  // todo: if src is longer than the frame then only read up to the end of the frame
  pub fn try_write<S: Read>(&mut self, offset: usize, len: usize, src: &mut S) -> Result<usize> {
    Ok(src.read(&mut self.as_mut()[offset .. offset + len])?)
  }
}