mod buffer_frame;
mod buffer_page;
mod buffer_pool;

use std::sync::atomic::{AtomicU64, Ordering};
use anyhow::Result;

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
  counter: AtomicU64,
  buffers: [BufferPool; 20]
}

impl BufferManager {
  pub fn alloc(&mut self) -> PageHandle {
    PageHandle::new(self.counter.fetch_add(1, Ordering::SeqCst))
  }

  pub fn fetch(&mut self, handle: &PageHandle) -> BufferPage {

  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let buffers: [BufferPool; 20] = [
      BufferPool::try_new(pool_size, 12)?,
      BufferPool::try_new(pool_size, 13)?,
      BufferPool::try_new(pool_size, 14)?,
      BufferPool::try_new(pool_size, 15)?,
      BufferPool::try_new(pool_size, 16)?,
      BufferPool::try_new(pool_size, 17)?,
      BufferPool::try_new(pool_size, 18)?,
      BufferPool::try_new(pool_size, 19)?,
      BufferPool::try_new(pool_size, 20)?,
      BufferPool::try_new(pool_size, 21)?,
      BufferPool::try_new(pool_size, 22)?,
      BufferPool::try_new(pool_size, 23)?,
      BufferPool::try_new(pool_size, 24)?,
      BufferPool::try_new(pool_size, 25)?,
      BufferPool::try_new(pool_size, 26)?,
      BufferPool::try_new(pool_size, 27)?,
      BufferPool::try_new(pool_size, 28)?,
      BufferPool::try_new(pool_size, 29)?,
      BufferPool::try_new(pool_size, 30)?,
      BufferPool::try_new(pool_size, 31)?
    ];

    // PID sequence
    let counter = AtomicU64::new(0);

    Ok(Self { buffers, counter })
  }
}