mod address_pool;
mod page_id_pool;

use anyhow::{
  anyhow, Result
};

use std::sync::atomic::{
  AtomicUsize, Ordering
};

use crate::{
  MAX_CLASS_ID, MIN_CLASS_ID,
  page_class, Page, PageGuard, PageSWIP
};

pub use address_pool::*;
pub use page_id_pool::*;

// Consider making PageClass wrap this
type ClassPools = Vec<AddressPool>;

#[derive(Debug)]
pub struct PageManager(ClassPools, PageIdPool, AtomicUsize);

impl PageManager {
  pub fn used_bytes(&self) -> usize {
    self.2.load(Ordering::Acquire)
  }

  pub fn try_free(&self, mut page: PageGuard) -> Result<()> {
    let page = page.try_write()?;
    let swip = page.swip().value();

    //
    // This could be a fizzled page in which case
    //  we need to figure out what to do here, we can't
    //  free it again so it seems like we just silently return?
    //

    // todo: very weird static design choices here
    let cid = PageSWIP::cid(swip);
    let idx = page_class::index_of(cid);

    match self.try_class_pool(idx) {
      Ok(pool) => {
        if pool.free(addr) {
          self.decrement_used(page_class::size_of(cid));
        }

        // Should this result in an error?
      }

      Err(err) => Err(err)
    }

    Ok(())
  }

  // todo: make this more thread-safe
  pub fn try_alloc(&self, len: u32) -> Result<PageGuard> {
    let cid = page_class::to_fit(len)?;
    let idx = page_class::index_of(cid);

    match self.try_class_pool(idx) {
      Ok(class) => {
        if let Some(address) = class.alloc() {
          let pid = self.page_id_pool().next();
          self.increment_used(page_class::size_of(cid));
          return match Page::try_alloc(address, pid, cid) {
            Err(err) => {
              // todo: What should we do if we can't activate a frame?
              class.free(address);
              Err(err)
            }

            Ok(page) => Ok(PageGuard::new(page))
          }
        }

        todo!("No more free space, write some memory to disk")
      }

      Err(err) => Err(err)
    }
  }

  // pub fn try_fetch<'a>(&self, _handle: &FizzledPageSWIP) -> Result<PageGuard> {
  //   todo!("Check the fridge and freezer for this page")
  // }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let mut pools: ClassPools = vec![];

    for cid in MIN_CLASS_ID..=MAX_CLASS_ID {
      pools.push(AddressPool::try_new(pool_size, cid)?)
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

  fn try_class_pool(&self, idx: usize) -> Result<&AddressPool> {
    match self.0.get(idx) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found at {}", idx))
    }
  }
}
