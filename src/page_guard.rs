mod read_guard;
mod share_guard;
mod write_guard;

use anyhow::{ Result };
use crate::{ Page };

pub use read_guard::*;
pub use share_guard::*;
pub use write_guard::*;

#[derive(Debug)]
pub struct PageGuard<'a>(Page<'a>);

impl<'a> PageGuard<'a> {
  pub fn new(page: Page<'a>) -> Self {
    Self(page)
  }

  //
  // try_read
  // try_share
  // try_write
  //

  pub fn try_write(&'a mut self) -> Result<WriteGuard<'a>> {
    WriteGuard::try_new(self.page_mut())
  }

  fn page_mut(&mut self) -> &mut Page<'a> {
    &mut self.0
  }
}