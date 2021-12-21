mod frame;
mod frame_pool;

use anyhow::{
  anyhow, Result
};

use std::sync::atomic::{
  AtomicU64, AtomicUsize, Ordering
};

use crate::{
  MAX_CLASS_ID,
  MIN_CLASS_ID,
  Page, PageHandle,
  PageIdent, PageSWIP,
  FizzledSWIP,
  page_class
};

pub use frame::*;
pub use frame_pool::*;

pub type MemoryPools = Vec<FramePool>;

#[derive(Debug)]
pub struct PageManager(MemoryPools, PageIdent, AtomicUsize);

impl PageManager {
  pub fn used_bytes(&self) -> usize {
    self.2.load(Ordering::Acquire)
  }

  pub fn try_swip(&mut self, len: u32) -> Result<FizzledSWIP> {
    let pid = self.page_ids().next();
    Ok(FizzledSWIP::pack(pid, page_class::cid_to_fit(len)?))
  }

  pub fn try_free(&mut self, page: &mut Page) -> Result<()> {
    let guard = page.try_write()?;
    let value = guard.frame().try_swip()?.value();
    self.decrement_used(page_class::size_of(FrameSWIP::cid(value)));
    Ok(())
  }

  pub fn try_fetch<'a>(&self, swip: &PageSWIP) -> Result<Page> {
    match swip {
      PageSWIP::Fizzled(_pid) => {
        todo!("Check the fridge and freezer for this page")
      }

      PageSWIP::Swizzled(swip) => {
        Ok(Page::from(Frame::try_from(swip.as_ptr())?)?)
      }
    }
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<Page> {
    match handle.swip() {
      PageSWIP::Fizzled(swip) => match self.pool_mut(swip.cid())?.alloc() {
        Some(mut frame) => {
          handle.swizzle(frame.address());
          self.increment_used(frame.len());
          frame.try_write_swip(swip.value())?;
          // todo: this isn't how this works... sort out FrameSWIP vs. PageSWIP
          frame.try_write_vlds(vlds.value())?;
          Ok(Page::from(frame))
        }

        None => todo!("No more free space, need to write some memory to disk")
      }

      PageSWIP::Swizzled(_ptr) => Err(anyhow!("Cannot alloc an already allocated page"))
    }
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let mut pools: MemoryPools = vec![];

    for cid in MIN_CLASS_ID..=MAX_CLASS_ID {
      pools.push(FramePool::try_new(pool_size, cid)?)
    }

    Ok(Self(pools, PageIdent::new(), AtomicUsize::new(0)))
  }

  // Private Accessors + Helpers

  fn pool_mut(&mut self, cid: usize) -> Result<&mut FramePool> {
    match self.0.get_mut(page_class::index_of(cid)) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found {}", cid))
    }
  }

  fn page_ids(&self) -> &PageIdent {
    &self.1
  }

  fn decrement_used(&mut self, len: usize) -> usize {
    self.used().fetch_sub(len, Ordering::SeqCst)
  }

  fn increment_used(&mut self, len: usize) -> usize {
    self.used().fetch_add(len, Ordering::SeqCst)
  }
}
