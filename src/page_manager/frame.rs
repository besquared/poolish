mod frame_data;
mod frame_head;
mod frame_swip;
mod frame_vlds;

use anyhow::{
  Result
};

use std::{ fmt };

use crate::{HEADER_LEN, page_class};

pub use frame_data::*;
pub use frame_head::*;
pub use frame_swip::*;
pub use frame_vlds::*;

//
// Frames need to be setup to have interior mutability but passed around as Arcs
//  This means that we need to not implement sync/send here but pass around Arc everywhere
//

#[derive(Clone, Debug)]
pub struct Frame(usize, usize);

// Debugging frame content
impl std::fmt::Binary for Frame {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let bytes = self.as_ref();
    for offset in 0..self.len() {
      fmt::Binary::fmt(&bytes[offset], f)?
    }
    Ok(())
  }
}

impl TryFrom<usize> for Frame {
  type Error = anyhow::Error;
  fn try_from(addr: usize) -> Result<Self> {
    let init = Self(addr, HEADER_LEN);

    // todo: read these with an exclusive guard as they may change
    let head = init.try_head()?;
    let swip = head.try_swip()?;

    Ok(Self(addr, page_class::size_of(FrameSWIP::cid(swip.value()))))
  }
}

impl Frame {
  pub fn try_activate(addr: usize, pid: usize, cid: usize) -> Result<Self> {
    let swip = FrameSWIP::pack(pid, cid);
    let vlds = FrameVLDS::default_value();

    let frame = Self::try_from(addr)?;
    frame.try_head()?.try_write_all(swip, vlds)?;

    Ok(frame)
  }
}

impl Frame {
  pub fn len(&self) -> usize {
    self.1
  }

  pub fn try_head(&self) -> Result<FrameHead> {
    let buf = self.as_mut();
    Ok(FrameHead::from(&mut buf[0 .. HEADER_LEN]))
  }

  pub fn try_data(&self) -> Result<FrameData> {
    let buf = self.as_mut();
    Ok(FrameData::from(&mut buf[HEADER_LEN .. self.len() - HEADER_LEN]))
  }

  // Private Accessors
  fn as_ref(&self) -> &[u8] {
    unsafe {
      std::slice::from_raw_parts(self.0 as *const u8, self.len())
    }
  }

  fn as_mut(&self) -> &mut [u8] {
    unsafe {
      std::slice::from_raw_parts_mut(self.0 as *mut u8, self.len())
    }
  }
}
