use anyhow::{
  anyhow, Result
};

use memmap2::{
  MmapMut
};

use parking_lot::Mutex;

use std::{
  collections::VecDeque,
  sync::{ Arc }
};

use crate::{
  Frame,
  Page,
  PageHandle,
  PageHandleState
};

#[derive(Debug)]
pub struct FramePool {
  class: u8,
  data: Arc<MmapMut>,
  frames: Arc<Mutex<VecDeque<Frame>>>
}

impl FramePool {
  pub fn class(&self) -> u8 {
    self.class
  }

  pub fn size(&self) -> usize {
    usize::pow(2usize, u32::from(self.class()))
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<Page> {
    match handle.state() {
      PageHandleState::Fizzled(pid) => {
        let class = self.class();
        let mut frames = self.frames.lock();
        if let Some(mut frame) = frames.pop_front() {
          if frame.pid() == 0 {
            handle.swizzle(&frame);
            frame.activate(pid, class, 1u8, 0u64)?;
            let page = Page::try_new(pid, frame.clone())?;

            frames.push_back(frame);

            return Ok(page);
          }
        }

        Err(anyhow!("No free frames found in buffer pool"))
      }
      PageHandleState::Swizzled(_ptr) => {
        return Err(anyhow!("Cannot allocate an swizzled page handle"))
      }
    }
  }

  pub fn try_fetch(&self, handle: &mut PageHandle) -> Result<Page> {
    match handle.state() {
      PageHandleState::Fizzled(_pid) => {
        todo!()
        // This page is cold, check the fridge, and then the disk manager
      }
      PageHandleState::Swizzled(address) => {
        let address: *const u8 = unsafe {
          std::mem::transmute(address)
        };

        // This is already hot, new up a page with this
        Ok(Page::try_new(handle.pid(), Frame::new(address, self.size()))?)
      }
    }
  }

  pub fn try_new(size_in_bytes: usize, class: u8) -> Result<Self> {
    let frame_size = usize::pow(2usize, u32::from(class));

    if frame_size > usize::pow(2usize, 31u32) {
      return Err(anyhow!("Page size cannot be greater than 2gb"))
    }

    if size_in_bytes < usize::pow(2, 31) {
      return Err(anyhow!("Page pool size must be a minimum of 2gb"))
    }

    if size_in_bytes % frame_size != 0 {
      return Err(anyhow!("Frame pool must be a multiple of frame size, {}, {}", size_in_bytes, frame_size))
    }

    // Allocate virtual memory
    let data = MmapMut::map_anon(size_in_bytes)?;

    let mut frames = VecDeque::new();
    for offset in (0..size_in_bytes).step_by(frame_size) {
      let ptr = match data.get(offset) {
        Some(byte_ref) => byte_ref as *const u8,
        None => return Err(anyhow!("PoolAllocationError: Cannot get reference to byte at offset {}", offset))
      };

      frames.push_back(Frame::new(ptr, frame_size));
    }

    let data = Arc::new(data);
    let frames = Arc::new(Mutex::new(frames));

    Ok(Self { class, data, frames })
  }
}