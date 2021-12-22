use anyhow::Result;
use core::hint::spin_loop;

use std::{
  io::{ Read, Write },
  ops::{ Deref, DerefMut }
};

use crate::{ FrameVLDS, Page };

#[derive(Debug)]
pub struct WriteGuard<'a>(&'a mut Page<'a>);

// impl<'a> Drop for WriteGuard<'a> {
//   fn drop(&mut self) {
//     // bump version
//     // release latch
//   }
// }

impl<'a> Deref for WriteGuard<'a> {
  type Target = Page<'a>;
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl<'a> DerefMut for WriteGuard<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0.deref_mut()
  }
}

// Methods

impl<'a> WriteGuard<'a> {
  pub fn read<W: Write>(&'a self, offset: usize, len: usize, dest: &mut W) -> Result<usize> {
    Ok(self.data().try_read(offset, len, dest)?)
  }

  pub fn write<R: Read>(&'a mut self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.data().try_write(offset, len, data)?)
  }
}

// Associated

impl<'a> WriteGuard<'a> {
  pub fn try_new(page: &'a mut Page<'a>) -> Result<Self> {
    let vlds = page.vlds();
    let mut value = vlds.value();
    let mut latch = FrameVLDS::latch(value);

    loop {
      if FrameVLDS::is_open(latch) {
        match vlds.latch_write() {
          Err(_) => continue,
          Ok(_) => return Ok(Self(page))
        }
      }

      while !FrameVLDS::is_open(FrameVLDS::latch(vlds.value())) {
        spin_loop();
      }

      value = vlds.value();
      latch = FrameVLDS::latch(value);
    }
  }
}