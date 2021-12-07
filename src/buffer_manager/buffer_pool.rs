use anyhow::{ anyhow, Result };

use memmap2::{
  MmapMut,
  MmapOptions
};

use std::collections::VecDeque;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::{BufferRef, BufferFrame, BufferSWIP};

#[derive(Debug)]
pub struct BufferPool {
  class: u8,
  size: usize,
  buffer: MmapMut,
  free_frames: VecDeque<BufferFrame>,
  used_frames: VecDeque<BufferFrame>
}

impl BufferPool {
  pub fn class(&self) -> u8 {
    self.class
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn page_size(&self) -> usize {
    usize::pow(2usize, u32::from(self.class()))
  }
  pub fn try_new(size: usize, page_class: u8) -> Result<Self> {
    let page_size = usize::pow(2usize, u32::from(page_class));

    if page_size > usize::pow(2usize, 31u32) {
      return Err(anyhow!("Buffer pool page size cannot be greater than 2gb"))
    }

    if size % page_size != 0 {
      return Err(anyhow!("Buffer pool size is not evenly divisible by page size, {}, {}", size, page_size))
    }

    // Allocate virtual memory
    let buffer = MmapMut::map_anon(size)?;

    let used_frames = VecDeque::new();
    let mut free_frames = VecDeque::new();
    for offset in (0..size).step_by(page_size) {
      let range = offset .. (offset + page_size);
      let buffer_ref = BufferRef::new(&buffer[range]);
      free_frames.push_back(BufferFrame::try_new(buffer_ref)?)
    }

    Ok(Self { size, class: page_class, buffer, free_frames, used_frames })
  }
}