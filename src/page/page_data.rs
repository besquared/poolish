// This is like the others bug has a reference to the data instead

use anyhow::{
  Result
};

use std::{
  io::{ Read, Write }
};

#[derive(Debug)]
pub struct PageData<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> From<T> for PageData<T> {
  fn from(data: T) -> Self {
    Self(data)
  }
}

impl<T: AsRef<[u8]>> PageData<T> {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }

  // todo: if dest is longer than the frame then only write up to the end of the frame
  pub fn try_read<D: Write>(&self, offset: usize, len: usize, dst: &mut D) -> Result<usize> {
    Ok(dst.write(&self.as_ref()[offset..offset + len])?)
  }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> PageData<T> {
  fn as_mut(&mut self) -> &mut [u8] {
    self.0.as_mut()
  }

  // todo: if src is longer than the frame then only read up to the end of the frame
  pub fn try_write<S: Read>(&mut self, offset: usize, len: usize, src: &mut S) -> Result<usize> {
    Ok(src.read(&mut self.as_mut()[offset .. offset + len])?)
  }

}