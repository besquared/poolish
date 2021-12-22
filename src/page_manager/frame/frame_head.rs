// This is like the others bug has a reference to the data instead

use anyhow::{
  anyhow, Result
};

use std::{
  mem,
  io::{
    Cursor, Seek, SeekFrom, Write
  }
};

use crate::{ FrameSWIP, FrameVLDS };

#[derive(Debug)]
pub struct FrameHead<'a>(&'a mut [u8]);

impl<'a> From<&'a mut [u8]> for FrameHead<'a> {
  fn from(data: &'a mut [u8]) -> Self {
    Self(data)
  }
}

impl<'a> FrameHead<'a> {
  fn swip_offset() -> usize {
    0usize
  }

  fn vlds_offset() -> usize {
    Self::swip_offset() + mem::size_of::<usize>()
  }
}

impl<'a> FrameHead<'a> {
  fn as_ref(&self) -> &[u8] {
    self.0
  }

  fn as_mut(&mut self) -> &mut [u8] {
    self.0
  }

  fn cursor(&mut self) -> Cursor<&mut [u8]> {
    Cursor::new(self.as_mut())
  }

  pub fn try_swip(&self) -> Result<FrameSWIP> {
    Ok(FrameSWIP::from(self.try_swip_ref()?))
  }

  pub fn try_vlds(&self) -> Result<FrameVLDS> {
    Ok(FrameVLDS::from(self.try_vlds_ref()?))
  }

  pub fn try_write_swip(&mut self, swip: usize) -> Result<usize> {
    Ok(self.cursor().write(&swip.to_be_bytes())?)
  }

  pub fn try_write_vlds(&mut self, vlds: usize) -> Result<usize> {
    let mut cursor = self.cursor();
    let offset = Self::vlds_offset();
    cursor.seek(SeekFrom::Start(offset as u64))?;

    Ok(cursor.write(&vlds.to_be_bytes())?)
  }

  pub fn try_write_all(&mut self, swip: usize, vlds: usize) -> Result<usize> {
    let mut cursor = self.cursor();
    Ok(cursor.write(&swip.to_be_bytes())? + cursor.write(&vlds.to_be_bytes())?)
  }

  // Private Accessors + Helpers

  fn try_swip_ref(&self) -> Result<&usize> {
    Ok(self.try_usize_ref(Self::swip_offset())?)
  }

  fn try_vlds_ref(&self) -> Result<&usize> {
    Ok(self.try_usize_ref(Self::vlds_offset())?)
  }

  fn try_usize_ref(&self, idx: usize) -> Result<&usize> {
    Ok(unsafe { &*(self.try_byte_ref(idx)? as *const u8 as *const usize) })
  }

  fn try_byte_ref(&self, idx: usize) -> Result<&u8> {
    println!("Frame::try_byte_ref({}) => {:?}", idx, self.as_ref());
    println!("{:?}", self.as_ref().get(idx));

    match self.as_ref().get(idx) {
      Some(byte_ref) => Ok(byte_ref),
      None => Err(anyhow!("index {} out of bounds for frame", idx))
    }
  }
}