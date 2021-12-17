use anyhow::{anyhow, Result };

use memmap2::{
  MmapMut
};

use parking_lot::Mutex;

use std::{
  collections::VecDeque,
  sync::{ Arc }
};

use crate::{BufferFrame, BufferPage, PageHandle, PageLatch};

#[derive(Debug)]
pub struct BufferPool {
  class: u8,
  data: Arc<MmapMut>,
  frames: Arc<Mutex<VecDeque<BufferFrame>>>
}

impl BufferPool {
  pub fn class(&self) -> u8 {
    self.class
  }

  pub fn page_size(&self) -> usize {
    usize::pow(2usize, u32::from(self.class()))
  }

  pub fn try_alloc<'a>(&mut self, handle: &mut PageHandle) -> Result<PageLatch<'a>> {
    let mut frames = self.frames.lock();

    let class = self.class();
    if let Some(mut frame) = frames.pop_front() {
      if frame.pid() == 0 {
        let pid = handle.value();
        frame.activate(pid, class, 1u8, 0u64)?;
        handle.swizzle(frame.as_ref().as_ptr() as u64);
        let page = PageLatch::new(frame.clone());

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
      let ptr = data.get(offset).unwrap() as *const u8;
      frames.push_back(BufferFrame::new(ptr, page_size));
    }

    let data = Arc::new(data);
    let frames = Arc::new(Mutex::new(frames));

    Ok(Self { class, data, frames })
  }
}