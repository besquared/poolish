mod frame_deque;
mod frame_pools;

use anyhow::{
  anyhow, Result
};

use memmap2::MmapMut;
use parking_lot::Mutex;
use std::sync::{ Arc };

use crate::{ MAX_CLASS_ID };

use frame_deque::*;
use frame_pools::*;

#[derive(Debug)]
pub struct FramePool(usize, Arc<MmapMut>, Arc<Mutex<FramePools>>);

impl FramePool {
  pub fn cid(&self) -> usize {
    self.0
  }

  pub fn data(&self) -> &MmapMut {
    &self.1.as_ref()
  }

  pub fn pools(&self) -> &Mutex<FramePools> {
    &self.2.as_ref()
  }

  pub fn alloc(&self) -> Option<usize> {
    let mut frames = self.pools().lock();
    if let Some(frame) = frames.free_mut().pop_front() {
      frames.used_mut().push_back(frame);
      Some(frame)
    } else {
      None
    }
  }

  pub fn release(&self, address: usize) {
    todo!("Release frame at address {}", address)
  }

  pub fn try_new(pool_size: usize, cid: usize) -> Result<Self> {
    // TODO: use page_class::size_of(cid) and page_class::size_of(MAX_CLASS_ID)
    let frame_size = 2usize.pow(cid as u32);
    let max_frame_size = usize::pow(2usize, MAX_CLASS_ID as u32);

    if frame_size > max_frame_size {
      return Err(anyhow!("Page size must be less than {} bytes", max_frame_size))
    }

    if pool_size < max_frame_size {
      return Err(anyhow!("Page pool size must be greater than {} bytes", max_frame_size))
    }

    if pool_size % frame_size != 0 {
      return Err(anyhow!("Page pool size must be divisible by page size: {} / {}", pool_size, frame_size))
    }

    // Allocate virtual memory and frame pools
    let data = Arc::new(MmapMut::map_anon(pool_size)?);
    let frames = Arc::new(Mutex::new(FramePools::try_new(data.clone(), frame_size)?));

    Ok(Self(cid, data, frames))
  }
}