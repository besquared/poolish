mod page_guard;
mod page_handle;
mod page_latch;

use anyhow::{
  anyhow, Result
};

use crate::BufferFrame;

pub use page_guard::*;
pub use page_handle::*;
pub use page_latch::*;


/**
 *
 * We don't need to do reclamation as long as version ALWAYS increments even if new pages are put in
 * What that really means is that we put a page in we increment its version
 *
 */

#[derive(Debug)]
pub struct BufferPage(i64, BufferFrame);

impl BufferPage {
  fn pid(&self) -> i64 {
    self.0
  }

  pub fn frame(&self) -> &BufferFrame {
    &self.1
  }

  pub fn try_load(pid: i64, mut frame: BufferFrame) -> Result<Self> {
    Ok(Self(pid, frame))
  }
}