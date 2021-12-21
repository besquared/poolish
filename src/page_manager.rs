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
    self.0.load(Ordering::Acquire)
  }

  pub fn try_swip(&mut self, len: u32) -> Result<FizzledSWIP> {
    let pid = self.page_ids().next();
    Ok(FizzledSWIP::pack(pid, page_class::cid_to_fit(len)?))
  }

  pub fn try_free(&mut self, page: &mut Page) -> Result<()> {
    let guard = page.try_write()?;
    self.decrement_used(PageClass::size_of(guard.read_cid()?));
    Ok(())
  }

  pub fn try_fetch<'a>(&self, swip: &PageSWIP) -> Result<Page> {
    match swip {
      PageSWIP::Fizzled(_pid) => {
        todo!("Check the fridge and freezer for this page")
      }

      PageSWIP::Swizzled(swip) => {
        let frame_ptr = swip.address() as *const u8;
        let frame_len = handle.class().size() as usize;
        Ok(Page::new(Frame::new(frame_ptr, frame_len))?)
      }
    }
  }

  // This isn't going to work cause I gotta swizzle this
  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<Page> {
    match handle.swip() {
      PageSWIP::Fizzled(swip) => match self.pool_mut(swip.cid()?)?.alloc() {
        Some(frame) => {
          handle.swizzle(frame.address());
          self.increment_used(frame.len());
          Ok(Page::try_alloc(&swip, &state, frame)?)
        }

        None => Err(anyhow!("No more free space, need to write some memory to disk"))
      }

      PageSWIP::Swizzled(_ptr) => Err(anyhow!("Cannot alloc an already allocated page"))
    }
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let mut pools: MemoryPools = vec![];

    for cid in MIN_CLASS_ID..=MAX_CLASS_ID {
      pools.push(frame_pool::try_new(pool_size, cid)?)
    }

    Ok(Self(pools, PageIdent::new(), AtomicUsize::new(0)))
  }

  // Private Accessors + Helpers

  fn pool_mut(&mut self, cid: usize) -> Result<&mut frame_pool> {
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
