mod buffer_frame;
mod buffer_page;
mod buffer_pool;

use std::sync::atomic::{AtomicI64, Ordering};
use anyhow::{anyhow, Result};

pub use buffer_frame::*;
pub use buffer_page::*;
pub use buffer_pool::*;

// const SIZE_CLASSES: [usize; 20] = [
//   2^12, 2^13, 2^14, 2^15, //   4k,   8k,  16k,  32k
//   2^16, 2^17, 2^18, 2^19, //  64k, 128k, 256k, 512k
//   2^20, 2^21, 2^22, 2^23, //   1m,   2m,   4m,   8m
//   2^24, 2^25, 2^26, 2^27, //  16m,  32m,  64m, 128m
//   2^28, 2^29, 2^30, 2^31  // 256m, 512m,   1g,   2g
// ];

#[derive(Debug)]
pub struct BufferManager {
  counter: AtomicI64,
  buffers: [BufferPool; 20]
}

impl BufferManager {
  pub fn new_handle(&mut self) -> PageHandle {
    PageHandle::new(self.counter.fetch_sub(1, Ordering::SeqCst))
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle, size: usize) -> Result<BufferPage> {
    if handle.is_fizzled() {
      self.buffers[0].try_alloc(handle)
    } else {
      Err(anyhow!("Cannot allocate an already allocated page handle"))
    }
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let buffers: [BufferPool; 20] = [
      BufferPool::try_new(pool_size, 12)?, //   4kb
      BufferPool::try_new(pool_size, 13)?, //   8kb
      BufferPool::try_new(pool_size, 14)?, //  16kb
      BufferPool::try_new(pool_size, 15)?, //  32kb
      BufferPool::try_new(pool_size, 16)?, //  64kb
      BufferPool::try_new(pool_size, 17)?, // 128kb
      BufferPool::try_new(pool_size, 18)?, // 256kb
      BufferPool::try_new(pool_size, 19)?, // 512kb
      BufferPool::try_new(pool_size, 20)?, //   1mb
      BufferPool::try_new(pool_size, 21)?, //   2mb
      BufferPool::try_new(pool_size, 22)?, //   4mb
      BufferPool::try_new(pool_size, 23)?, //   8mb
      BufferPool::try_new(pool_size, 24)?, //  16mb
      BufferPool::try_new(pool_size, 25)?, //  32mb
      BufferPool::try_new(pool_size, 26)?, //  64mb
      BufferPool::try_new(pool_size, 27)?, // 128mb
      BufferPool::try_new(pool_size, 28)?, // 256mb
      BufferPool::try_new(pool_size, 29)?, // 512mb
      BufferPool::try_new(pool_size, 30)?, //   1gb
      BufferPool::try_new(pool_size, 31)?  //   2gb
    ];

    // PID sequence
    let counter = AtomicI64::new(0);

    Ok(Self { buffers, counter })
  }
}