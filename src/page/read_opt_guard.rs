use anyhow::Result;
use std::io::Write;

use crate::{ Page, PageLatch };

#[derive(Debug)]
pub struct ReadOptGuard<'a>(&'a Page, u64);

impl<'a> ReadOptGuard<'a> {
  pub fn page(&self) -> &Page {
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
    let page = self.page();
    let latch = page.latch()?;

    let mut value = latch.value();
    let mut state = PageLatch::state(value);
    let mut version = PageLatch::version(value);

    if version != self.version() {
      return Ok(None)
    }

    loop {
      if PageLatch::is_exclusive(state) {
        // Wait on access to this page
        while PageLatch::is_exclusive(state) {
          core::hint::spin_loop();

          value = latch.value();
          state = PageLatch::state(value);
          version = PageLatch::version(value);
        }
      }  else {
        // Read into dest buffer
        let dest_bytes = page.frame().read(offset, len, dest)?;

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