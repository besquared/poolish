mod memory_pool;

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

pub use memory_pool::*;

pub type MemoryPools = Vec<MemoryPool>;

#[derive(Debug)]
pub struct PageManager(AtomicUsize, AtomicU64, MemoryPools);

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
      // if pid is 0 then this allocation
      PageHandleState::Fizzled(pid) => match self.get_pool_mut(class)?.alloc() {
        Some(frame) => {
          handle.swizzle(&frame);
          self.increment_used(frame.len());
          Ok(Page::try_alloc(cid, pid, frame)?)
        }

        None => Err(anyhow!("No more free space, need to write some memory to disk"))
      }

      PageHandleState::Swizzled(_ptr) => Err(anyhow!("Cannot alloc an already allocated page"))
    }
  }

  pub fn try_release(&mut self, handle: &mut PageHandle) -> Result<()> {
    self.decrement_used(handle.class().size() as usize);
    Ok(())
  }

  pub fn try_new_handle(&mut self, data_len_in_bytes: u32) -> Result<PageHandle> {
    let pid = self.pids().fetch_add(1, Ordering::SeqCst);
    let class = PageClass::try_new_to_fit(data_len_in_bytes)?;

    Ok(PageHandle::new(pid, class))
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let mut memory_pools: MemoryPools = vec![];

    for class in MIN_CLASS..MAX_CLASS {
      memory_pools.push(MemoryPool::try_new(pool_size, PageClass::try_new(class)?)?)
    }

    // (used, pids, pools)
    Ok(Self(AtomicUsize::new(0), AtomicU64::new(1), memory_pools))
  }

  // Private Helper Functions
  fn used(&self) -> &AtomicUsize {
    &self.0
  }

  fn pids(&self) -> &AtomicU64 {
    &self.1
  }

  fn decrement_used(&mut self, len: usize) -> usize {
    self.used().fetch_sub(len, Ordering::SeqCst)
  }

  fn increment_used(&mut self, len: usize) -> usize {
    self.used().fetch_add(len, Ordering::SeqCst)
  }

  fn get_pool_mut(&mut self, class: &PageClass) -> Result<&mut MemoryPool> {
    match self.2.get_mut(class.index()) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found {}", class.id()))
    }
  }
}
