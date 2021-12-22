use anyhow::{ Result };
use crate::{ FrameData, FrameVLDS };

#[derive(Debug)]
pub struct ShareGuard<'a>(&'a FrameVLDS<'a>, &'a FrameData<'a>);

// Associated

impl<'a> ShareGuard<'a> {
  pub fn try_new(_: &'a FrameVLDS<'a>, _: &'a FrameData<'a>) -> Result<Self> {
    todo!()

    // pub fn lock_shared(&mut self) -> Option<SharedPageGuard<'a>> {
    //   // If the latch is open then cas(0, 2)
    //   // If the latch is shared then cas(shared_count, shared_count + 1)
    //   // If the latch is exclusive then wait for it to be unlocked, loop
    //   None
    // }


    //
    // How do we know if the page gets changed out from under us when we call this?
    //

    // if page.version() != self.version() {
    //   return Ok(None)
    // }
    //
    // loop {
    //   if self.page().latch().is_exclusive() {
    //     // Wait on access to this page
    //     while self.page().latch().is_exclusive() {
    //       core::hint::spin_loop();
    //     }
    //   }  else {
    //     // Read into dest buffer
    //     let bytes_written = self.page().read(offset, dest)?;
    //
    //     // Recheck version
    //     return if self.page().version() != self.version() {
    //       Ok(None)
    //     } else {
    //       Ok(Some(bytes_written))
    //     }
    //   }
    // }
  }
}
