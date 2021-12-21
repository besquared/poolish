mod frame_data;
mod frame_swip;
mod frame_vlds;

use anyhow::{
  anyhow, Result
};

use std::{
  io::{ Read, Write },
  mem::size_of
};

use crate::{HEADER_LEN, FizzledSWIP, page_class};

pub use frame_data::*;
pub use frame_swip::*;
pub use frame_vlds::*;

//
// Frames need to be setup to have interior mutability but passed around as Arcs
//  This means that we need to not implement sync/send here but pass around Arc everywhere
//

#[derive(Clone, Debug)]
pub struct Frame(*const u8, usize);

// Allow frames to shared between threads
unsafe impl Sync for Frame {}

// Allow Frames to be moved between threads
unsafe impl Send for Frame {}

impl TryFrom<*const u8> for Frame {
  type Error = anyhow::Error;
  fn try_from(ptr: *const u8) -> Result<Self> {
    let head = Self(ptr, 16);
    let swip = head.try_swip()?;
    let cid = FrameSWIP::cid(swip.value());
    Ok(Self::new(ptr, page_class::size_of(cid)))
  }
}

// For debugging bits
impl std::fmt::Binary for Frame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let bytes = self.as_ref();
    for offset in 0..self.len() {
      std::fmt::Binary::fmt(&bytes[offset], f)?
    }
    Ok(())
  }
}

impl Frame {
  fn swip_offset() -> usize {
    0usize
  }

  fn vlds_offset() -> usize {
    Self::swip_offset() + std::mem::size_of::<usize>()
  }

  fn data_offset() -> usize {
    Self::vlds_offset() + std::mem::size_of::<usize>()
  }

  pub fn new(ptr: *const u8, len: usize) -> Self {
    Self(ptr, len)
  }
}

impl Frame {
  pub fn len(&self) -> usize {
    self.1
  }

  pub fn address(&self) -> usize {
    self.0 as usize
  }

  pub fn try_swip(&self) -> Result<FrameSWIP> {
    Ok(FrameSWIP::from(self.try_swip_ref()?))
  }

  pub fn try_vlds(&self) -> Result<FrameVLDS> {
    Ok(FrameVLDS::from(self.try_vlds_ref()?))
  }

  pub fn try_data(&self) -> Result<FrameData> {
    let len = self.len();
    let buf = self.as_mut();
    Ok(FrameData::from(buf.slice(HEADER_LEN, len - HEADER_LEN)))
  }

  pub fn try_write_swip(&mut self, swip: usize) -> Result<()> {
    Ok(*(self.try_swip_ref_mut()?) = swip)
  }

  pub fn try_write_vlds(&mut self, vlds: usize) -> Result<()> {
    Ok(*(self.try_vlds_ref_mut()?) = vlds)
  }

  // Private Accessors + Helpers

  fn as_ref(&self) -> &[u8] {
    unsafe {
      std::slice::from_raw_parts(self.0, self.len())
    }
  }

  fn as_mut(&self) -> &mut [u8] {
    let mut_ptr = self.0 as *mut u8;

    unsafe {
      std::slice::from_raw_parts_mut(mut_ptr, self.len())
    }
  }

  fn try_swip_ref(&self) -> Result<&usize> {
    Ok(self.try_byte_ref(Self::swip_offset())? as &usize)
  }

  fn try_swip_ref_mut(&self) -> Result<&mut usize> {
    Ok(self.try_byte_ref(Self::swip_offset())? as &mut usize)
  }

  fn try_vlds_ref(&self) -> Result<&usize> {
    Ok(self.try_byte_ref(Self::vlds_offset())? as &usize)
  }

  fn try_vlds_ref_mut(&self) -> Result<&mut usize> {
    Ok(self.try_byte_ref(Self::vlds_offset())? as &mut usize)
  }

  fn try_byte_ref(&self, idx: usize) -> Result<&u8> {
    let address = self.address();
    match self.as_ref().get(idx) {
      Some(byte_ref) => Ok(byte_ref),
      None => Err(anyhow!("Index {} out of bounds for frame {:?}", idx, address))
    }
  }
}
