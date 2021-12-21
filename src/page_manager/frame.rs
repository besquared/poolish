mod frame_swip;
mod frame_vlds;

use anyhow::{
  anyhow, Result
};

use std::{
  io::{ Read, Write },
  mem::size_of
};

use crate::{
  HEADER_LEN,
  FizzledSWIP,
  page_class
};

pub use frame_swip::*;
pub use frame_vlds::*;

// Frames need to be setup to have interior mutability but passed around as Arcs

#[derive(Clone, Debug)]
pub struct Frame(*const u8, usize);

// Allow frames to shared between threads
unsafe impl Sync for Frame {}

// Allow Frames to be moved between threads
unsafe impl Send for Frame {}

impl AsRef<[u8]> for Frame {
  fn as_ref(&self) -> &[u8] {
    unsafe {
      std::slice::from_raw_parts(self.0, self.len())
    }
  }
}

impl AsMut<[u8]> for Frame {
  fn as_mut(&mut self) -> &mut [u8] {
    unsafe {
      let mut_ptr = self.0 as *mut u8;
      std::slice::from_raw_parts_mut(mut_ptr, self.len())
    }
  }
}

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
  pub fn len(&self) -> usize {
    self.1
  }

  pub fn address(&self) -> usize {
    self.0 as usize
  }

  // This could return a FrameSWIP
  pub fn read_swip(&self) -> Result<usize> {
    let len = size_of::<u8>();
    let offset = Self::swip_offset() as u64;
    Ok(usize::from_le_bytes(self.as_ref().read_slice_at(offset, len)?))
  }

  // This could return a FrameState
  pub fn read_state(&self) -> Result<usize> {
    let len = size_of::<usize>();
    let offset = Self::state_offset() as u64;
    Ok(usize::from_le_bytes(self.as_ref().read_slice_at(offset, len)?))
  }

  // todo: if dest is longer than the frame then only write up to the end of the frame
  pub fn read<W: Write>(&self, offset: usize, len: usize, dest: &mut W) -> Result<usize> {
    let bytes = self.as_ref();
    let offset = usize::from(HEADER_LEN) + offset;
    Ok(dest.write(&bytes[offset..offset + len])?)
  }

  // todo: if src is longer than the frame then only read up to the end of the frame
  pub fn write<R: Read>(&mut self, offset: usize, len: usize, src: &mut R) -> Result<usize> {
    let bytes = self.as_mut();
    let offset = Self::values_offset() + offset;
    Ok(src.read(&mut bytes[offset .. offset + len])?)
  }

  // Private Accessors

  fn swip_ref(&self) -> Result<&usize> {
    Ok(self.byte_ref(Self::swip_offset())? as &usize)
  }

  fn state_ref(&self) -> Result<&usize> {
    Ok(self.byte_ref(Self::state_offset())? as &usize)
  }

  fn byte_ref(&self, idx: usize) -> Result<&u8> {
    let address = self.address();
    match self.as_ref().get(idx) {
      Some(byte_ref) => Ok(byte_ref),
      None => Err(anyhow!("Cannot find reference to pid at frame {:?}", address))
    }
  }
}

impl Frame {
  fn swip_offset() -> usize {
    0usize
  }

  fn state_offset() -> usize {
    8usize
  }

  fn data_offset() -> usize {
    16usize
  }

  pub fn new(ptr: *const u8, len: usize) -> Self {
    Self(ptr, len)
  }

  pub fn try_from_ptr(ptr: *const u8) -> Result<Self> {
    let tmp = Self::new(ptr, 16);
    let swip = FizzledSWIP::new(tmp.read_swip()?);
    Ok(Self::new(ptr, page_class::size_of(swip.cid())))
  }
}
