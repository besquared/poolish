//
// If we're going to pass frames across the wire we should
//  consider implementing the latch here instead of in the page
//

use anyhow::{ anyhow, Result };
use std::io::{ Read, Write };

use crate::{ HEADER_LEN };

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
      std::slice::from_raw_parts(self.as_ptr(), self.len())
    }
  }
}

impl AsMut<[u8]> for Frame {
  fn as_mut(&mut self) -> &mut [u8] {
    unsafe {
      std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
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

  pub fn cid_ref(&self) -> Result<&u8> {
    self.byte_ref(0)
  }

  pub fn pid_ref(&self) -> Result<&u8> {
    self.byte_ref(8)
  }

  pub fn dirty_ref(&self) -> Result<&u8> {
    self.byte_ref(9)
  }

  pub fn latch_ref(&self) -> Result<&u8> {
    self.byte_ref(10)
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
    let offset = usize::from(HEADER_LEN) + offset;
    Ok(src.read(&mut bytes[offset .. offset + len])?)
  }

  pub fn new(ptr: *const u8, len: usize) -> Self {
    Self(ptr, len)
  }

  // Private Helper Functions

  fn as_ptr(&self) -> *const u8 {
    self.0
  }

  fn as_mut_ptr(&self) -> *mut u8 {
    self.0 as *mut u8
  }

  fn byte_ref(&self, idx: usize) -> Result<&u8> {
    let frame_ptr = self.as_ptr();
    match self.as_ref().get(idx) {
      Some(byte_ref) => Ok(byte_ref),
      None => Err(anyhow!("Cannot find reference to pid at frame {:?}", frame_ptr))
    }
  }
}
