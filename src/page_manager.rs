mod frame;
mod frame_pool;
mod page_id_pool;

use anyhow::{
  anyhow, Result
};

use std::sync::atomic::{
  AtomicUsize, Ordering
};

use crate::{
  MAX_CLASS_ID,
  MIN_CLASS_ID,
  page_class,
  Page, FizzledPageSWIP
};

pub use frame::*;
pub use frame_pool::*;
pub use page_id_pool::*;

type ClassPools = Vec<FramePool>;

#[derive(Debug)]
pub struct PageManager(ClassPools, PageIdPool, AtomicUsize);

impl PageManager {
  pub fn used_bytes(&self) -> usize {
    self.2.load(Ordering::Acquire)
  }

  pub fn try_free(&self, page: &mut Page) -> Result<()> {
    let guard = page.try_write()?;
    let swip = guard.frame().try_swip()?;
    let value = FrameSWIP::cid(swip.value());
    self.decrement_used(page_class::size_of(value));
    Ok(())
  }

  pub fn try_alloc(&self, len: u32) -> Result<Page> {
    let cid = page_class::to_fit(len)?;
    let idx = page_class::index_of(cid);

    match self.try_frame_pool(idx) {
      Ok(frames) => {
        if let Some(address) = frames.alloc() {
          let pid = self.page_id_pool().next();
          self.increment_used(page_class::size_of(cid));
          return match Frame::try_activate(address, pid, cid) {
            Err(err) => {
              // TODO: What should we do if we can't activate a frame?
              frames.release(address);
              Err(err)
            }

            Ok(frame) => Ok(Page::from(frame))
          }
        }

        todo!("No more free space, write some memory to disk")
      }

      Err(err) => Err(err)
    }
  }

  pub fn try_fetch<'a>(&self, _handle: &FizzledPageSWIP) -> Result<Page> {
    todo!("Check the fridge and freezer for this page")
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let mut pools: ClassPools = vec![];

    for cid in MIN_CLASS_ID..=MAX_CLASS_ID {
      pools.push(FramePool::try_new(pool_size, cid)?)
    }

    Ok(Self(pools, PageIdPool::new(), AtomicUsize::new(0)))
  }

  // Private Accessors + Helpers
  fn page_id_pool(&self) -> &PageIdPool {
    &self.1
  }

  fn increment_used(&self, len: usize) -> usize {
    self.2.fetch_add(len, Ordering::SeqCst)
  }

  fn decrement_used(&self, len: usize) -> usize {
    self.2.fetch_sub(len, Ordering::SeqCst)
  }

  fn try_frame_pool(&self, idx: usize) -> Result<&FramePool> {
    match self.0.get(idx) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found at {}", idx))
    }
  }
}
