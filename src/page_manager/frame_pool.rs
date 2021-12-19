mod alloc_pool;
mod allocations;

use anyhow::{
  anyhow, Result
};

use memmap2::MmapMut;
use parking_lot::Mutex;
use std::sync::{ Arc };

use crate::{
  MAX_CLASS,
  Frame, PageClass
};

use alloc_pool::*;
use allocations::*;

#[derive(Debug)]
pub struct FramePool(PageClass, Arc<MmapMut>, Arc<Mutex<Allocations>>);

impl FramePool {
  pub fn class(&self) -> &PageClass {
    &self.0
  }

  pub fn data(&self) -> &MmapMut {
    &self.1.as_ref()
  }

  pub fn frames(&self) -> &Mutex<Allocations> {
    &self.2.as_ref()
  }

  pub fn alloc(&mut self) -> Option<Frame> {
    let mut frames = self.frames().lock();
    if let Some(frame) = frames.free_mut().pop_front() {
      frames.used_mut().push_back(frame.clone());
      Some(frame)
    } else {
      None
    }
  }

  pub fn try_new(pool_size: usize, class: PageClass) -> Result<Self> {
    let frame_size = 2usize.pow(class.id() as u32);
    let max_frame_size = usize::pow(2usize, MAX_CLASS as u32);

    if frame_size > max_frame_size {
      return Err(anyhow!("Page size must be less than {} bytes", max_frame_size))
    }

    if pool_size < max_frame_size {
      return Err(anyhow!("Page pool size must be greater than {} bytes", max_frame_size))
    }

    if pool_size % frame_size != 0 {
      return Err(anyhow!("Page pool size must be divisible by page size: {} / {}", pool_size, frame_size))
    }

    // Allocate virtual memory
    let data = Arc::new(MmapMut::map_anon(pool_size)?);
    let frames = Arc::new(Mutex::new(Allocations::try_new(data.clone(), frame_size)?));

    Ok(Self(class, data, frames))
  }
}