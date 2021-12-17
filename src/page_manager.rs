mod frame;
mod frame_pool;

use anyhow::{
  anyhow, Result
};

use std::sync::atomic::{
  AtomicI64, Ordering
};

use crate::{
  Page,
  PageHandle
};

pub use frame::*;
pub use frame_pool::*;

//
// const SIZE_CLASSES: [usize; 20] = [
//   2^12, 2^13, 2^14, 2^15, //   4k,   8k,  16k,  32k
//   2^16, 2^17, 2^18, 2^19, //  64k, 128k, 256k, 512k
//   2^20, 2^21, 2^22, 2^23, //   1m,   2m,   4m,   8m
//   2^24, 2^25, 2^26, 2^27, //  16m,  32m,  64m, 128m
//   2^28, 2^29, 2^30, 2^31  // 256m, 512m,   1g,   2g
// ];
//

#[derive(Debug)]
pub struct PageManager(AtomicI64, [FramePool; 20]);

impl PageManager {
  fn pid(&self) -> &AtomicI64 {
    &self.0
  }

  fn pools(&self) -> &[FramePool; 20] {
    &self.1
  }

  fn pools_mut(&mut self) -> &mut [FramePool; 20] {
    &mut self.1
  }

  pub fn new_handle(&mut self, size: usize) -> PageHandle {
    PageHandle::new(size, self.pid().fetch_sub(1, Ordering::SeqCst))
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<Page> {
    let idx = handle.class() - 12;
    let pool = match self.pools_mut().get_mut(idx) {
      Some(pool) => pool,
      None => return Err(anyhow!("Page size class not found {}", handle.class()))
    };

    pool.try_alloc(handle)
  }

  // what do we do if we want to bring a page back in?

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let pools: [FramePool; 20] = [
      FramePool::try_new(pool_size, 12)?, //   4kb
      FramePool::try_new(pool_size, 13)?, //   8kb
      FramePool::try_new(pool_size, 14)?, //  16kb
      FramePool::try_new(pool_size, 15)?, //  32kb
      FramePool::try_new(pool_size, 16)?, //  64kb
      FramePool::try_new(pool_size, 17)?, // 128kb
      FramePool::try_new(pool_size, 18)?, // 256kb
      FramePool::try_new(pool_size, 19)?, // 512kb
      FramePool::try_new(pool_size, 20)?, //   1mb
      FramePool::try_new(pool_size, 21)?, //   2mb
      FramePool::try_new(pool_size, 22)?, //   4mb
      FramePool::try_new(pool_size, 23)?, //   8mb
      FramePool::try_new(pool_size, 24)?, //  16mb
      FramePool::try_new(pool_size, 25)?, //  32mb
      FramePool::try_new(pool_size, 26)?, //  64mb
      FramePool::try_new(pool_size, 27)?, // 128mb
      FramePool::try_new(pool_size, 28)?, // 256mb
      FramePool::try_new(pool_size, 29)?, // 512mb
      FramePool::try_new(pool_size, 30)?, //   1gb
      FramePool::try_new(pool_size, 31)?  //   2gb
    ];

    // PID sequence
    Ok(Self(AtomicI64::new(-1), pools))
  }
}