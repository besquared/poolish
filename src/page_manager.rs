mod frame;
mod frame_pool;

use anyhow::{
  anyhow, Result
};

use std::sync::atomic::{
  AtomicU64, AtomicUsize, Ordering
};

use crate::{
  MAX_CLASS, MIN_CLASS,
  Page, PageClass, PageHandle, PageHandleState
};

pub use frame::*;
pub use frame_pool::*;

pub type FramePools = Vec<FramePool>;

#[derive(Debug)]
pub struct PageManager(AtomicUsize, AtomicU64, FramePools);

impl PageManager {
  pub fn used_bytes(&self) -> usize {
    self.0.load(Ordering::Acquire)
  }

  pub fn try_fetch(&self, handle: &mut PageHandle) -> Result<Page> {
    match handle.state() {
      PageHandleState::Fizzled(_pid) => {
        todo!()
        // This page is cold, check the fridge, and then the freezer
      }

      PageHandleState::Swizzled(frame_ptr) => {
        let frame_ptr: *const u8 = unsafe {
          std::mem::transmute(frame_ptr)
        };

        let frame_len = handle.class().size() as usize;

        // This is already hot, create a fetched page with this
        Ok(Page::try_fetch(handle, Frame::new(frame_ptr, frame_len))?)
      }
    }
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<Page> {
    let cid = handle.cid();
    let state = handle.state();
    let class = handle.class();

    match state {
      PageHandleState::Fizzled(pid) => match self.pool_mut(class)?.alloc() {
        Some(frame) => {
          handle.swizzle(&frame);
          self.alloc(frame.len());
          Ok(Page::try_alloc(cid, pid, frame)?)
        }
        None => Err(anyhow!("No more free space, need to write some memory to disk"))
      }

      PageHandleState::Swizzled(_ptr) => Err(anyhow!("Cannot alloc an already allocated page"))
    }
  }

  pub fn try_release(&mut self, handle: &mut PageHandle) -> Result<()> {
    self.free(handle.class().size() as usize);
    Ok(())
  }

  pub fn try_new_handle(&mut self, data_len_in_bytes: u32) -> Result<PageHandle> {
    let pid = self.pids().fetch_add(1, Ordering::SeqCst);
    let class = PageClass::try_new_to_fit(data_len_in_bytes)?;

    Ok(PageHandle::new(pid, class))
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let mut pools: FramePools = vec![];

    for class in MIN_CLASS..MAX_CLASS {
      pools.push(FramePool::try_new(pool_size, PageClass::try_new(class)?)?)
    }

    // (used, pids, pools)
    Ok(Self(AtomicUsize::new(0), AtomicU64::new(1), pools))
  }

  // Private Helper Functions
  fn used(&self) -> &AtomicUsize {
    &self.0
  }

  fn pids(&self) -> &AtomicU64 {
    &self.1
  }

  fn pools_mut(&mut self) -> &mut FramePools {
    &mut self.2
  }

  fn pool_mut(&mut self, class: &PageClass) -> Result<&mut FramePool> {
    match self.pools_mut().get_mut(class.index()) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found {}", class.id()))
    }
  }

  fn free(&mut self, len: usize) -> usize {
    self.used().fetch_sub(len, Ordering::SeqCst)
  }

  fn alloc(&mut self, len: usize) -> usize {
    self.used().fetch_add(len, Ordering::SeqCst)
  }
}
