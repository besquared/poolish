mod read_guard;
mod share_guard;
mod write_guard;

use anyhow::{ Result };

use crate::{
  Frame, PageHandle
};

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
    &self.0
  }

  pub fn try_handle(&self) -> Result<PageHandle> {
    Ok(PageHandle::from(self.frame().try_swip()?))
  }

  pub fn try_write(&self) -> Result<WriteGuard> {
    WriteGuard::try_new(self, self.frame().try_vlds()?)
  }

  // Private Accessors

}