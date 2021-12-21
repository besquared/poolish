use anyhow::Result;
use std::io::Write;

use crate::{ Page, FrameVLDS };

#[derive(Debug)]
pub struct ReadOptGuard<'a>(&'a Page, usize);

impl<'a> ReadOptGuard<'a> {
  pub fn page(&self) -> &Page {
    &self.0
  }

  pub fn version(&self) -> usize {
    self.1
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
    let vlds = page.frame().try_vlds()?;

    let mut value = vlds.value();

    if FrameVLDS::version(value) != self.version() {
      return Ok(None)
    }

    loop {
      while FrameVLDS::is_exclusive(FrameVLDS::latch(vlds.value())) {
        core::hint::spin_loop();
      }

      // Read into dest buffer
      let bytes_read = page.frame().try_data()?.try_read(offset, len, dest)?;

      // Recheck version
      return if FrameVLDS::version(vlds.value()) != self.version() {
        Ok(None)
      } else {
        Ok(Some(bytes_read))
      }
    }
  }
}