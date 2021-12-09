use std::cell::RefCell;
use anyhow::{anyhow, Result };

use memmap2::{
  MmapMut
};

use std::collections::VecDeque;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::{BufferFrame, BufferPage, PageHandle};

#[derive(Debug)]
pub struct BufferPool {
  class: u8,
  data: MmapMut,
  frames: Mutex<VecDeque<Arc<BufferFrame>>>
}

impl BufferPool {
  pub fn class(&self) -> u8 {
    self.class
  }

  pub fn page_size(&self) -> usize {
    usize::pow(2usize, u32::from(self.class()))
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<BufferPage> {
    let mut frames = match self.frames.try_lock() {
      Ok(frames) => frames,
      Err(err) => return Err(anyhow!(err.to_string()))
    };

    let class = self.class();
    if let Some(mut frame) = frames.pop_front() {
      if !frame.is_active() {
        frame.activate();
        let page = BufferPage::try_alloc(frame.clone(), class, handle)?;
        frames.push_back(frame);

        return Ok(page);
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

    let mut frames = VecDeque::new();
    for offset in (0..size_in_bytes).step_by(page_size) {
      let range = offset .. (offset + page_size);
      frames.push_back(Arc::new(BufferFrame::new(&data[range])))
    }
    let frames = Mutex::new(frames);

    Ok(Self { class, data, frames })
  }
}