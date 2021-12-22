mod read_guard;
mod share_guard;
mod write_guard;

use anyhow::{ Result };

use std::{
  io::{ Cursor, Write }
};

use crate::{
  SWIP_LEN, VLDS_LEN, page_class,
  FrameData, FrameSWIP, FrameVLDS, PageHandle
};

pub use read_guard::*;
pub use share_guard::*;
pub use write_guard::*;

#[derive(Debug)]
pub struct Page<'a>(&'a mut [u8]);

// Methods

impl<'a> Page<'a> {
  fn swip(&'a self) -> FrameSWIP<'a> {
    FrameSWIP::from(Self::slice_swip(self.0))
  }

  fn vlds(&'a self) -> FrameVLDS<'a> {
    FrameVLDS::from(Self::slice_vlds(self.0))
  }

  fn data(&'a self) -> FrameData<'a> {
    FrameData::from(Self::slice_data_mut(self.0, self.0.len()))
  }

  // What is the point of this exactly?
  pub fn try_handle(&self) -> Result<PageHandle> {
    Ok(PageHandle::from(self.swip()))
  }

  //
  // try_read
  // try_share
  // try_write
  //

  pub fn try_write(&'a mut self) -> Result<WriteGuard<'a>> {
    WriteGuard::try_new(self)
  }
}

// Associated

impl<'a> Page<'a> {
  pub fn try_alloc(addr: usize, pid: usize, cid: usize) -> Result<Self> {
    let vlen = page_class::size_of(cid);
    let swip = FrameSWIP::pack(pid, cid);
    let vlds = FrameVLDS::default_value();

    let slice = Self::slice_mut(addr, vlen);
    Self::try_alloc_head(slice, swip, vlds)?;

    Ok(Self(slice))
  }

  fn try_alloc_head(slice: &mut [u8], swip: usize, vlds: usize) -> Result<usize> {
    let mut cursor = Cursor::new(slice);
    Ok(cursor.write(&swip.to_be_bytes())? + cursor.write(&vlds.to_be_bytes())?)
  }

  //
  // Individual Accessors
  //

  fn slice_swip(slice: &[u8]) -> &[u8] {
    &slice[0 .. SWIP_LEN]
  }

  fn slice_vlds(slice: &[u8]) -> &[u8] {
    &slice[SWIP_LEN .. VLDS_LEN]
  }

  fn slice_data(slice: &[u8], data_len: usize) -> &[u8] {
    &slice[(SWIP_LEN + VLDS_LEN) .. data_len]
  }

  fn slice_data_mut(slice: &mut [u8], data_len: usize) -> &mut [u8] {
    &mut slice[(SWIP_LEN + VLDS_LEN) .. data_len]
  }

  fn slice(addr: usize, len: usize) -> &'a [u8] {
    unsafe {
      std::slice::from_raw_parts(addr as *const u8, len)
    }
  }

  fn slice_mut(addr: usize, len: usize) -> &'a mut [u8] {
    unsafe {
      std::slice::from_raw_parts_mut(addr as *mut u8, len)
    }
  }
}
