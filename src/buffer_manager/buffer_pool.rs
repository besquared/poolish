use anyhow::{ anyhow, Result };

use memmap2::{
  MmapMut
};

use std::collections::VecDeque;
use std::sync::{Mutex};

use crate::{BufferFrame};

#[derive(Debug)]
pub struct BufferPool {
  data: MmapMut,
  page_type: u8,
  free_frames: Mutex<VecDeque<BufferFrame>>,
  used_frames: Mutex<VecDeque<BufferFrame>>
}

impl BufferPool {
  pub fn page_type(&self) -> u8 {
    self.page_type
  }

  pub fn page_size(&self) -> usize {
    usize::pow(2usize, u32::from(self.page_type()))
  }

  pub fn try_new(size_in_bytes: usize, page_type: u8) -> Result<Self> {
    let page_size = usize::pow(2usize, u32::from(page_type));

    if page_size > usize::pow(2usize, 31u32) {
      return Err(anyhow!("Buffer pool page size cannot be greater than 2gb"))
    }

    if size_in_bytes % page_size != 0 {
      return Err(anyhow!("Buffer pool size is not evenly divisible by page size, {}, {}", size_in_bytes, page_size))
    }

    // Allocate virtual memory
    let data = MmapMut::map_anon(size_in_bytes)?;

    let used_frames = VecDeque::new();
    let mut free_frames = VecDeque::new();
    for offset in (0..size_in_bytes).step_by(page_size) {
      let range = offset .. (offset + page_size);
      free_frames.push_back(BufferFrame::new(&data[range]))
    }

    let used_frames = Mutex::new(used_frames);
    let free_frames = Mutex::new(free_frames);

    Ok(Self { data, page_type, free_frames, used_frames })
  }
}