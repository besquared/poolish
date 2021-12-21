mod page_ident;
mod read_guard;
mod share_guard;
mod write_guard;

use anyhow::{ Result };
use core::hint::spin_loop;

use std::{
  io::{ Cursor, Write }
};

use crate::{ Frame, FrameSWIP, FrameVLDS };

pub use page_ident::*;
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
  //
  // try_read
  // try_share
  // try_write
  //

  pub fn frame(&self) -> &Frame {
    &self.1
  }

  pub fn try_write(&mut self) -> Result<WriteGuard> {
    WriteGuard::try_new(self, self.frame().try_vlds()?)
  }

  // Private Accessors

}