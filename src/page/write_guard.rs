use anyhow::Result;
use core::hint::spin_loop;

use std::{
  io::{ Read, Write },
  ops::Deref
};

use crate::{
  Frame, FrameVLDS, Page
};

#[derive(Debug)]
pub struct WriteGuard<'a>(&'a Page, FrameVLDS<'a>);

impl<'a> Deref for WriteGuard<'a> {
  type Target = Page;

  fn deref(&self) -> &Self::Target {
    self.0
  }
}

// impl<'a> Drop for WriteGuard<'a> {
//   fn drop(&mut self) {
//     // bump version
//     // release latch
//   }
// }

impl<'a> WriteGuard<'a> {
  pub fn read<W: Write>(&self, offset: usize, len: usize, dest: &mut W) -> Result<usize> {
    Ok(self.frame().try_data()?.try_read(offset, len, dest)?)
  }

  pub fn write<R: Read>(&self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.frame().try_data()?.try_write(offset, len, data)?)
  }

  pub fn try_new(page: &'a Page, vlds: FrameVLDS<'a>) -> Result<Self> {
    let mut value = vlds.value();
    let mut latch = FrameVLDS::latch(value);

    loop {
      if FrameVLDS::is_open(latch) {
        match vlds.latch_write() {
          Err(_) => continue,
          Ok(_) => return Ok(Self(page, vlds))
        }
      }

      while !FrameVLDS::is_open(FrameVLDS::latch(vlds.value())) {
        spin_loop();
      }

      value = vlds.value();
      latch = FrameVLDS::latch(value);
    }
  }

  // Private Accessors

  fn frame(&self) -> &Frame {
    self.0.frame()
  }
}