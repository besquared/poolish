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
pub struct WriteGuard<'a>(&'a mut Page, FrameVLDS<'a>);

// impl<'a> Drop for WriteGuard<'a> {
//   fn drop(&mut self) {
//     // bump version
//     // release latch
//   }
// }

impl<'a> AsRef<Page> for WriteGuard<'a> {
  fn as_ref(&self) -> &Page {
    self.0.deref()
  }
}

impl<'a> AsMut<Page> for WriteGuard<'a> {
  fn as_mut(&mut self) -> &mut Page {
    self.0
  }
}

impl<'a> Deref for WriteGuard<'a> {
  type Target = Page;

  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<'a> WriteGuard<'a> {
  pub fn read<W: Write>(&self, offset: usize, len: usize, dest: &mut W) -> Result<usize> {
    Ok(self.frame().try_data()?.try_read(offset, len, dest)?)
  }

  pub fn write<R: Read>(&self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.frame().try_data()?.try_write(offset, len, data)?)
  }

  pub fn try_new(page: &'a mut Page, vlds: FrameVLDS<'a>) -> Result<Self> {
    let mut value = vlds.value();
    let mut latch = FrameVLDS::latch(value);

    loop {
      if FrameVLDS::is_open(latch) {
        match latch.lock_write(value) {
          Err(_) => continue,
          Ok(_) => return Ok(Self(page, latch))
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