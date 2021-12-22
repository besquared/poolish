mod free_pool;
mod used_pool;
mod addr_pool;

use anyhow::{
  anyhow, Result
};

use memmap2::MmapMut;
use parking_lot::Mutex;
use std::sync::{ Arc };

use crate::{ MAX_CLASS_ID };

use free_pool::*;
use used_pool::*;
use addr_pool::*;

#[derive(Debug)]
pub struct AddressPool(usize, Arc<MmapMut>, Arc<Mutex<AddrPool>>);

impl AddressPool {
  fn cid(&self) -> usize {
    self.0
  }

  fn data(&self) -> &MmapMut {
    &self.1.as_ref()
  }

  fn pools(&self) -> &Mutex<AddrPool> {
    &self.2.as_ref()
  }

  pub fn alloc(&self) -> Option<usize> {
    self.pools().lock().alloc()
  }

  pub fn free(&self, addr: usize) -> bool {
    self.pools().lock().free(addr)
  }

  pub fn try_new(pool_size: usize, cid: usize) -> Result<Self> {
    // TODO: use page_class::size_of(cid) and page_class::size_of(MAX_CLASS_ID)
    let frame_size = 2usize.pow(cid as u32);
    let max_frame_size = usize::pow(2usize, MAX_CLASS_ID as u32);

    if frame_size > max_frame_size {
      return Err(anyhow!("Page size must be less than {} bytes", max_frame_size))
    }

    if pool_size < max_frame_size {
      return Err(anyhow!("Page pool size must be greater than {} bytes", max_frame_size))
    }

    if pool_size % frame_size != 0 {
      return Err(anyhow!("Page pool size must be divisible by page size: {} / {}", pool_size, frame_size))
    }

    // Allocate virtual memory pools
    let data = Arc::new(MmapMut::map_anon(pool_size)?);
    let frames = Arc::new(Mutex::new(AddrPool::try_new(data.clone(), frame_size)?));

    Ok(Self(cid, data, frames))
  }
}