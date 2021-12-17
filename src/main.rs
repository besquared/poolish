#![feature(slice_ptr_get)]

mod buffer_manager;
mod storage_manager;

use anyhow::Result;
use std::sync::Arc;
use parking_lot::Mutex;
use buffer_manager::BufferManager;

pub use buffer_manager::*;
pub use storage_manager::*;

const POOL_SIZE: usize = usize::pow(2, 31);

fn main() -> Result<()> {
  let manager = BufferManager::try_new(POOL_SIZE)?;
  let manager = Arc::new(Mutex::new(manager));

  let m1 = manager.clone();
  let t1 = std::thread::Builder::new();
  let t1 = t1.name("t1".to_string()).spawn(move || {
    for _ in 1..10 {
      let mut manager = m1.lock();

      let mut handle = manager.new_handle();
      println!("[t1] page handle {:?}", handle);
      let page_latch = manager.try_alloc(&mut handle, 12).unwrap();
      println!("[t2] page_latch {:?}", page_latch);
    }
  })?;

  let m2 = manager.clone();
  let t2 = std::thread::Builder::new();
  let t2 = t2.name("t2".to_string()).spawn(move || {
    for _ in 1..10 {
      let mut manager = m2.lock();
      let mut handle = manager.new_handle();

      println!("[t2] page handle {:?}", handle);
      let page_latch = manager.try_alloc(&mut handle, 12).unwrap();
      println!("[t2] page_latch {:?}", page_latch);
    }
  })?;

  t1.join().unwrap();
  t2.join().unwrap();

  Ok(())
}
