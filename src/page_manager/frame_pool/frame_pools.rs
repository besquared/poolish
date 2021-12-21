use anyhow::{
  anyhow, Result
};

use memmap2::MmapMut;
use std::sync::Arc;

use super::{ FrameDeque };

#[derive(Clone, Debug)]
pub struct FramePools(FrameDeque, FrameDeque);

impl FramePools {
  pub fn free(&self) -> &FrameDeque {
    &self.0
  }

  pub fn used(&self) -> &FrameDeque {
    &self.1
  }

  pub fn free_mut(&mut self) -> &mut FrameDeque {
    &mut self.0
  }

  pub fn used_mut(&mut self) -> &mut FrameDeque {
    &mut self.1
  }

  pub fn try_new(data: Arc<MmapMut>, frame_size: usize) -> Result<Self> {
    let mut free = FrameDeque::default();
    for offset in (0..data.len()).step_by(frame_size) {
      let address = match data.get(offset) {
        Some(address) => address as *const _ as usize,
        None => return Err(anyhow!("PoolAllocationError: Cannot get reference to byte at offset {}", offset))
      };

      free.push_back(address);
    }

    Ok(Self(free, FrameDeque::default()))
  }
}