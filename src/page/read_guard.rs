use anyhow::Result;
use std::io::{ Write };
use crate::{ FrameVLDS, FrameData };

#[derive(Debug)]
pub struct ReadGuard<'a>(&'a FrameVLDS<'a>, &'a FrameData<'a>, usize);

// Methods

impl<'a> ReadGuard<'a> {
  fn vlds(&self) -> &FrameVLDS {
    self.0
  }

  fn data(&self) -> &FrameData {
    self.1
  }

  fn version(&self) -> usize {
    self.2
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
  pub fn try_read<W: AsRef<[u8]> + Write>(&'a mut self, offset: usize, len: usize, dest: &mut W) -> Result<Option<usize>> {
    let vlds = self.vlds();
    let data = self.data();

    if FrameVLDS::version(vlds.value()) != self.version() {
      return Ok(None)
    }

    loop {
      while FrameVLDS::is_exclusive(FrameVLDS::latch(vlds.value())) {
        core::hint::spin_loop();
      }

      // Read into dest buffer
      let bytes_read = data.try_read(offset, len, dest)?;

      // Recheck version
      return if FrameVLDS::version(vlds.value()) != self.version() {
        Ok(None)
      } else {
        Ok(Some(bytes_read))
      }
    }
  }
}