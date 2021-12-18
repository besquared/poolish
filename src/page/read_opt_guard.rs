use anyhow::Result;
use std::io::Write;

use crate::{Page};

#[derive(Debug)]
pub struct ReadOptGuard<'a>(&'a Page, u64);

impl<'a> ReadOptGuard<'a> {
  pub fn latch(&self) -> &Page {
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
  pub fn try_read<W: AsRef<[u8]> + Write>(&'a mut self, offset: usize, len: usize, dest: &mut W) -> Result<Option<usize>> {
    let mut value = self.latch().latch_value();
    let mut state = Page::state(value);
    let mut version = Page::version(value);

    if version != self.version() {
      return Ok(None)
    }

    loop {
      if Page::is_exclusive(state) {
        // Wait on access to this page
        while Page::is_exclusive(state) {
          core::hint::spin_loop();
          value = self.latch().latch_value();
          state = Page::state(value);
          version = Page::version(value);
        }
      }  else {
        // Read into dest buffer
        let dest_bytes = self.latch().frame().read(offset, len, dest)?;

        // Recheck version
        return if version != self.version() {
          Ok(None)
        } else {
          Ok(Some(dest_bytes))
        }
      }
    }
  }
}