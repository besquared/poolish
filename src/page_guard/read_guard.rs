use anyhow::Result;

use std::{
  io::{ Write },
  ops::Deref
};

use crate::{PageVLDS, Page };

#[derive(Debug)]
pub struct ReadGuard<'a>(&'a Page<'a>, usize);

impl<'a> Deref for ReadGuard<'a> {
  type Target = Page<'a>;
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

// Methods

impl<'a> ReadGuard<'a> {
  fn version(&self) -> usize {
    self.1
  }

  // pub fn lock_optimistic(&mut self) -> Option<OptimisticPageGuard<'a>> {
  //   // If the latch is open then return page guard
  //   // If the latch is shared then return page guard
  //   // If the latch is exclusive then wait for it to be unlocked, loop
  //
  //   None
  // }
}

// Associated

impl<'a> ReadGuard<'a> {
  // Returns None if a read couldn't be performed due to a version mismatch
  //  Otherwise returns Some(usize) which is the number of bytes written/read
  pub fn try_read<D: AsRef<[u8]> + Write>(&'a mut self, offset: usize, len: usize, dest: &mut D) -> Result<Option<usize>> {
    let vlds = self.vlds();
    let data = self.data();

    if PageVLDS::version(vlds.value()) != self.version() {
      return Ok(None)
    }

    loop {
      while PageVLDS::is_exclusive(PageVLDS::latch(vlds.value())) {
        core::hint::spin_loop();
      }

      // Read into dest buffer
      let bytes_read = data.try_read(offset, len, dest)?;

      // Recheck version
      return if PageVLDS::version(vlds.value()) != self.version() {
        Ok(None)
      } else {
        Ok(Some(bytes_read))
      }
    }
  }
}