use anyhow::Result;
use core::hint::spin_loop;
use std::io::Read;
use std::ops::Deref;
use crate::{Frame, Page, FrameState};

#[derive(Debug)]
pub struct WriteGuard<'a>(&'a mut Page, FrameState<'a>);

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
  pub fn read<W: Wite>(&mut self, offset: usize, len: usize, dest: &mut W) -> Result<usize> {
    Ok(self.frame().read(offset, len, dest)?)
  }

  pub fn write<R: Read>(&mut self, offset: usize, len: usize, data: &mut R) -> Result<usize> {
    Ok(self.frame_mut().write(offset, len, data)?)
  }

  pub fn try_new(page: &'a mut Page, state: FrameState<'a>) -> Result<Self> {
    let mut value = state.value();
    let mut latch = FrameState::latch(value);

    loop {
      if FrameState::is_open(latch) {
        match latch.lock_write(value) {
          Err(_) => continue,
          Ok(_) => return Ok(Self(page, latch))
        }
      }

      while !FrameState::is_open(FrameState::latch(latch.value())) {
        spin_loop();
      }

      value = state.value();
      latch = FrameState::latch(value);
    }
  }

  // Private Accessors

  fn frame(&self) -> &Frame {
    self.0.frame()
  }

  fn frame_mut(&mut self) -> &mut Frame {
    self.0.frame_mut()
  }
}