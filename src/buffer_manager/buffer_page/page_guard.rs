// An optimistic read guard around a page

use anyhow::Result;
use std::io::Write;

use crate::BufferPage;

#[derive(Debug)]
pub enum PageGuard<'a> {
  Shared(SharedPageGuard<'a>),
  Exclusive(ExclusivePageGuard<'a>),
  Optimistic(OptimisticPageGuard<'a>)
}

#[derive(Debug)]
pub struct SharedPageGuard<'a>(&'a mut BufferPage<'a>);

#[derive(Debug)]
pub struct ExclusivePageGuard<'a>(&'a mut BufferPage<'a>);

#[derive(Debug)]
pub struct OptimisticPageGuard<'a>(&'a mut BufferPage<'a>, u64);

impl<'a> OptimisticPageGuard<'a> {
  pub fn page(&'a self) -> &BufferPage {
    &self.0
  }

  pub fn version(&self) -> u64 {
    self.1
  }

  pub fn unlock(self) -> () {
    todo!()
    // returns the new version number
    // update version here once we unlock
  }

  // Returns None if a read couldn't be performed due to a version mismatch
  //  Otherwise returns Some(usize) which is the number of bytes written/read
  pub fn try_read<W: AsRef<[u8]> + Write>(&'a mut self, offset: usize, dest: &mut W) -> Result<Option<usize>> {
    if self.page().version() != self.version() {
      return Ok(None)
    }

    loop {
      if self.page().latch().is_exclusive() {
        // Wait on access to this page
        while self.page().latch().is_exclusive() {
          core::hint::spin_loop();
        }
      }  else {
        // Read into dest buffer
        let bytes_written = self.page().read(offset, dest)?;

        // Recheck version
        return if self.page().version() != self.version() {
          Ok(None)
        } else {
          Ok(Some(bytes_written))
        }
      }
    }
  }
}