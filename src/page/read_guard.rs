use anyhow::Result;
use std::io::Write;

use crate::{Page, FrameState};

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

  // pub fn lock_optimistic(&mut self) -> Option<OptimisticPageGuard<'a>> {
  //   // If the latch is open then return page guard
  //   // If the latch is shared then return page guard
  //   // If the latch is exclusive then wait for it to be unlocked, loop
  //
  //   None
  // }


  // Returns None if a read couldn't be performed due to a version mismatch
  //  Otherwise returns Some(usize) which is the number of bytes written/read
  pub fn try_read<W: AsRef<[u8]> + Write>(&'a mut self, offset: usize, len: usize, dest: &mut W) -> Result<Option<usize>> {
    let page = self.page();
    let latch = page.state()?;

    let mut value = latch.value();
    let mut state = FrameState::latch(value);
    let mut version = FrameState::version(value);

    if version != self.version() {
      return Ok(None)
    }

    loop {
      if FrameState::is_exclusive(state) {
        // Wait on access to this page
        while FrameState::is_exclusive(state) {
          core::hint::spin_loop();

          value = latch.value();
          state = FrameState::latch(value);
          version = FrameState::version(value);
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