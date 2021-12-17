use anyhow::Result;
use std::io::Write;

use crate::{ PageLatch };

#[derive(Debug)]
pub struct OptimisticPageGuard<'a>(&'a PageLatch<'a>, u64);

impl<'a> OptimisticPageGuard<'a> {
  pub fn latch(&self) -> &PageLatch {
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
    let mut value = self.latch().load();
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
          value = self.latch().load();
          state = PageLatch::state(value);
          version = PageLatch::version(value);
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