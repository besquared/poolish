mod page_ident;
mod page_state;
mod read_guard;
mod share_guard;
mod write_guard;

use anyhow::{ Result };
use core::hint::spin_loop;

use std::{
  io::{ Cursor, Write }
};

use crate::{
  FizzledSWIP, Frame,
  PageHandle, PageSWIP
};

pub use page_ident::*;
pub use frame_state::*;
pub use read_guard::*;
pub use share_guard::*;
pub use write_guard::*;

#[derive(Debug)]
pub struct Page(Frame);

impl From<Frame> for Page {
  fn from(frame: Frame) -> Self {
    Self(frame)
  }
}

impl Page {
  pub fn read_cid(&self) -> Result<usize> {
    self.frame().read_cid()
  }

  pub fn read_pid(&self) -> Result<usize> {
    self.frame().read_pid()
  }

  //
  // try_read
  // try_share
  // try_write
  //

  pub fn try_write(&mut self) -> Result<WriteGuard> {
    WriteGuard::try_new(self, self.state()?)
  }

  pub fn try_alloc(swip: &FizzledSWIP, state: &PageState, mut frame: Frame) -> Result<Self> {
    let mut cursor = Cursor::new(frame.as_mut());

    cursor.write(&swip.value().to_le_bytes())?;
    cursor.write(&state.value().to_le_bytes())?;

    Ok(Self(pid, frame))
  }

  // Private Accessors

  fn frame(&self) -> &Frame {
    &self.1
  }

  fn frame_mut(&mut self) -> &mut Frame {
    &mut self.1
  }
}