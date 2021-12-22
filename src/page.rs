mod page_data;
mod page_swip;
mod page_vlds;

use anyhow::{ Result };

use std::{
  io::{ Cursor, Write }
};

use crate::{
  SWIP_LEN, VLDS_LEN, page_class
};

pub use page_data::*;
pub use page_swip::*;
pub use page_vlds::*;

#[derive(Debug)]
pub struct Page<'a>(&'a mut [u8]);

// Methods

impl<'a> Page<'a> {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn swip(&'a self) -> PageSWIP<'a> {
    PageSWIP::from(Self::slice_swip(self.0))
  }

  pub fn vlds(&'a self) -> PageVLDS<'a> {
    PageVLDS::from(Self::slice_vlds(self.0))
  }

  pub fn data(&'a self) -> PageData<&'a [u8]> {
    PageData::from(Self::slice_data(self.0, self.0.len()))
  }

  pub fn data_mut(&'a mut self) -> PageData<&'a mut [u8]> {
    PageData::from(Self::slice_data_mut(self.0, self.0.len()))
  }
}

// Associated

impl<'a> Page<'a> {
  pub fn try_alloc(addr: usize, pid: usize, cid: usize) -> Result<Self> {
    let vlen = page_class::size_of(cid);
    let swip = PageSWIP::pack(pid, cid);
    let vlds = PageVLDS::default_value();

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

  fn slice_mut(addr: usize, len: usize) -> &'a mut [u8] {
    unsafe {
      std::slice::from_raw_parts_mut(addr as *mut u8, len)
    }
  }
}
