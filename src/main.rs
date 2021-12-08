#![feature(slice_ptr_get)]

mod buffer_manager;
mod storage_manager;

use anyhow::Result;
use buffer_manager::BufferManager;

pub use buffer_manager::*;
pub use storage_manager::*;

fn main() -> Result<()> {
  let mut manager = BufferManager::try_new(usize::pow(2, 31))?;

  let mut handle = manager.new_handle();
  let mut handle = manager.new_handle();
  let mut handle = manager.new_handle();
  let mut handle = manager.new_handle();
  let mut handle = manager.new_handle();

  println!("page handle {:?}", handle);

  let buffer_page = manager.try_alloc(&mut handle, 12)?;

  println!("page handle {:?}", handle);
  println!("buffer page {:?}", buffer_page);
  Ok(())
}
