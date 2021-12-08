use anyhow::{ anyhow, Result };

use memmap2::{
  MmapMut
};

use std::collections::VecDeque;
use std::sync::{Mutex};

use crate::{BufferFrame, BufferPage, PageHandle};

#[derive(Debug)]
pub struct BufferPool {
  class: u8,
  data: MmapMut,
  frames: Vec<BufferFrame>
}

impl BufferPool {
  pub fn class(&self) -> u8 {
    self.class
  }

  pub fn page_size(&self) -> usize {
    usize::pow(2usize, u32::from(self.class()))
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<BufferPage> {
    let class = self.class();
    for frame in self.frames.iter_mut() {
      if !frame.is_active() {
        frame.activate();
        return Ok(BufferPage::try_alloc(frame, class, handle)?);
      }
    }

    Err(anyhow!("No free frames found in buffer pool"))
  }

  pub fn try_new(size_in_bytes: usize, class: u8) -> Result<Self> {
    let page_size = usize::pow(2usize, u32::from(class));

    if page_size > usize::pow(2usize, 31u32) {
      return Err(anyhow!("Buffer pool page size cannot be greater than 2gb"))
    }

    if size_in_bytes % page_size != 0 {
      return Err(anyhow!("Buffer pool size is not evenly divisible by page size, {}, {}", size_in_bytes, page_size))
    }

    // Allocate virtual memory
    let data = MmapMut::map_anon(size_in_bytes)?;

    let mut frames = Vec::new();
    for offset in (0..size_in_bytes).step_by(page_size) {
      let range = offset .. (offset + page_size);
      frames.push(BufferFrame::new(&data[range]))
    }


    Ok(Self { class, data, frames })
  }
}