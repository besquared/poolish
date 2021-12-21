mod frame_data;
mod frame_swip;
mod frame_vlds;

use anyhow::{
  anyhow, Result
};

use crate::{HEADER_LEN, page_class};

pub use frame_data::*;
pub use frame_swip::*;
pub use frame_vlds::*;

//
// Frames need to be setup to have interior mutability but passed around as Arcs
//  This means that we need to not implement sync/send here but pass around Arc everywhere
//

#[derive(Clone, Debug)]
pub struct Frame(usize, usize);

impl TryFrom<usize> for Frame {
  type Error = anyhow::Error;
  fn try_from(address: usize) -> Result<Self> {
    let head = Self(address, 16);
    let swip = head.try_swip()?;
    let cid = FrameSWIP::cid(swip.value());
    Ok(Self(address, page_class::size_of(cid)))
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

  pub fn try_activate(address: usize, pid: usize, cid: usize) -> Result<Self> {
    let mut frame = Self::try_from(address)?;
    frame.try_write_swip(FrameSWIP::pack(pid, cid))?;
    frame.try_write_vlds(FrameVLDS::default_value())?;
    Ok(frame)
  }
}

impl Frame {
  pub fn len(&self) -> usize {
    self.1
  }

  pub fn address(&self) -> usize {
    self.0
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
    Ok(FrameData::from(&mut buf[HEADER_LEN .. len - HEADER_LEN]))
  }

  pub fn try_write_swip(&mut self, swip: usize) -> Result<()> {
    Ok(*(self.try_swip_mut()?) = swip)
  }

  pub fn try_write_vlds(&mut self, vlds: usize) -> Result<()> {
    Ok(*(self.try_vlds_mut()?) = vlds)
  }

  // Private Accessors + Helpers

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

  fn try_swip_ref(&self) -> Result<&usize> {
    Ok(self.try_usize_ref(Self::swip_offset())?)
  }

  fn try_swip_mut(&self) -> Result<&mut usize> {
    Ok(self.try_usize_mut(Self::swip_offset())?)
  }

  fn try_vlds_ref(&self) -> Result<&usize> {
    Ok(self.try_usize_ref(Self::vlds_offset())?)
  }

  fn try_vlds_mut(&self) -> Result<&mut usize> {
    Ok(self.try_usize_mut(Self::vlds_offset())?)
  }

  fn try_byte_ref(&self, idx: usize) -> Result<&u8> {
    let address = self.address();
    match self.as_ref().get(idx) {
      Some(byte_ref) => Ok(byte_ref),
      None => Err(anyhow!("index {} out of bounds for frame at {:?}", idx, address))
    }
  }

  fn try_byte_mut(&self, idx: usize) -> Result<&mut u8> {
    let address = self.address();
    match self.as_mut().get_mut(idx) {
      Some(byte_mut) => Ok(byte_mut),
      None => Err(anyhow!("index {} out of bounds for frame at {:?}", idx, address))
    }
  }

  fn try_usize_ref(&self, idx: usize) -> Result<&usize> {
    Ok(unsafe { &*(self.try_byte_ref(idx)? as *const u8 as *const usize) })
  }

  fn try_usize_mut(&self, idx: usize) -> Result<&mut usize> {
    Ok(unsafe { &mut *(self.try_byte_mut(idx)? as *mut u8 as *mut usize) })
  }
}
