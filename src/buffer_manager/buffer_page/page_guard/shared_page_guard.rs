use anyhow::{ Result };
use crate::{ PageLatch };

#[derive(Debug)]
pub struct SharedPageGuard<'a>(&'a PageLatch<'a>);

impl<'a> SharedPageGuard<'a> {
  pub fn try_new(page: &'a PageLatch<'a>) -> Result<Self> {
    todo!()

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
