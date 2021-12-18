mod frame;
mod frame_pool;

use anyhow::{
  anyhow, Result
};

use std::sync::atomic::{
  AtomicU64, Ordering
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

pub type FramePools = [FramePool; 20];

#[derive(Debug)]
pub struct PageManager(AtomicU64, FramePools);

impl PageManager {
  pub fn new_handle(&mut self, data_len_in_bytes: u32) -> Result<PageHandle> {
    let class = class_to_fit(data_len_in_bytes)?;
    Ok(PageHandle::new(class, self.pids().fetch_add(1, Ordering::SeqCst)))
  }

  pub fn try_fetch(&self, handle: &mut PageHandle) -> Result<Page> {
    self.pool(handle.class())?.try_fetch(handle)
  }

  pub fn try_alloc(&mut self, handle: &mut PageHandle) -> Result<Page> {
    self.pool_mut(handle.class())?.try_alloc(handle)
  }

  pub fn try_new(pool_size: usize) -> Result<Self> {
    let pools: FramePools = [
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
      FramePool::try_new(pool_size, 31)?, //   2gb
    ];

    // PID sequence
    Ok(Self(AtomicU64::new(1), pools))
  }

  // Private Helper Functions
  fn pids(&self) -> &AtomicU64 {
    &self.0
  }

  fn pools(&self) -> &FramePools {
    &self.1
  }

  fn pools_mut(&mut self) -> &mut FramePools {
    &mut self.1
  }

  fn pool(&self, class: u8) -> Result<&FramePool> {
    let idx = class_index(class)?;
    match self.pools().get(idx as usize) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found {}", class))
    }
  }

  fn pool_mut(&mut self, class: u8) -> Result<&mut FramePool> {
    let idx = class_index(class)?;
    match self.pools_mut().get_mut(idx as usize) {
      Some(pool) => Ok(pool),
      None => Err(anyhow!("Page size class not found {}", class))
    }
  }
}

fn class_index(class: u8) -> Result<u8> {
  if class <= 31u8 {
    if class < 12u8 {
      Ok(0)
    } else {
      Ok(class - 12u8)
    }
  } else {
    Err(anyhow!("Page size class must be less than 31"))
  }
}

// computes the class needed to fit a certain number of bytes
fn class_to_fit(data_len_in_bytes: u32) -> Result<u8> {
  let data_len = f64::from(data_len_in_bytes);
  let total_len = data_len + f64::from(FRAME_HEADER_LEN);

  if total_len <= f64::from(u32::MAX) {
    Ok(class_index((total_len.log(10f64) / 2f64.log(10f64)).ceil() as u8)?)
  } else {
    Err(anyhow!("Page size class not found to fit {} header + data bytes", total_len.round()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_class_to_fit() -> Result<()> {
    // Min class up to 4096b - 18b
    assert_eq!(0, class_to_fit(0)?);
    assert_eq!(0, class_to_fit(2u32.pow(1) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(2) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(3) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(4) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(5) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(6) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(7) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(8) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(9) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(10) + 1)?);
    assert_eq!(0, class_to_fit(2u32.pow(11) + 1)?);

    assert_eq!(1, class_to_fit(2u32.pow(12) + 1)?);
    assert_eq!(2, class_to_fit(2u32.pow(13) + 1)?);
    assert_eq!(3, class_to_fit(2u32.pow(14) + 1)?);
    assert_eq!(4, class_to_fit(2u32.pow(15) + 1)?);

    assert_eq!(5, class_to_fit(2u32.pow(16) + 1)?);
    assert_eq!(6, class_to_fit(2u32.pow(17) + 1)?);
    assert_eq!(7, class_to_fit(2u32.pow(18) + 1)?);
    assert_eq!(8, class_to_fit(2u32.pow(19) + 1)?);

    assert_eq!( 9, class_to_fit(2u32.pow(20) + 1)?);
    assert_eq!(10, class_to_fit(2u32.pow(21) + 1)?);
    assert_eq!(11, class_to_fit(2u32.pow(22) + 1)?);
    assert_eq!(12, class_to_fit(2u32.pow(23) + 1)?);

    assert_eq!(13, class_to_fit(2u32.pow(24) + 1)?);
    assert_eq!(14, class_to_fit(2u32.pow(25) + 1)?);
    assert_eq!(15, class_to_fit(2u32.pow(26) + 1)?);
    assert_eq!(16, class_to_fit(2u32.pow(27) + 1)?);

    assert_eq!(17, class_to_fit(2u32.pow(28) + 1)?);
    assert_eq!(18, class_to_fit(2u32.pow(29) + 1)?);
    assert_eq!(19, class_to_fit(2u32.pow(30) + 1)?);

    if let Ok(class) = class_to_fit(2u32.pow(31)) {
      assert!(false, "page class {} unexpectedly found for bytes {}", class, 2u32.pow(31))
    }

    if let Ok(class) = class_to_fit(u32::MAX) {
      assert!(false, "page class {} unexpectedly found for bytes {}", class, u32::MAX)
    }

    Ok(())
  }
}