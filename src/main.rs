#![feature(slice_ptr_get)]

mod buffer_manager;
mod storage_manager;

use std::io::Cursor;
use anyhow::Result;
use std::sync::Arc;
use parking_lot::Mutex;
use buffer_manager::BufferManager;

pub use buffer_manager::*;
pub use storage_manager::*;

const POOL_SIZE: usize = usize::pow(2, 31);

fn main() -> Result<()> {
  let manager = build_buffer_manager()?;

  let mut pages = manager.lock();
  let mut handle = pages.new_handle();
  println!("[main] page handle {:?}", handle);

  let mut page_latch = pages.try_alloc(&mut handle, 12).unwrap();
  println!("[main] page latch {:?}", page_latch);

  println!("Frame is {:b}", &page_latch.frame());

  let mut page_guard = page_latch.acquire_exclusive()?;
  println!("[main] guard is {:?}", page_guard);

  let data = vec![ 1u8, 2u8, 3u8 ];
  let mut data = Cursor::new(data);

  page_guard.write(0, 3, &mut data);

  println!("Frame is {:b}", &page_guard.frame());

  Ok(())
}

fn build_buffer_manager() -> Result<Arc<Mutex<BufferManager>>> {
  Ok(Arc::new(Mutex::new(BufferManager::try_new(POOL_SIZE)?)))
}

pub fn thread_test() -> Result<()> {
  let manager = build_buffer_manager()?;

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