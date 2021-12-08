#![feature(slice_ptr_get)]

mod buffer_manager;
mod storage_manager;

use std::sync::Arc;
use anyhow::Result;
use buffer_manager::BufferManager;

pub use buffer_manager::*;
pub use storage_manager::*;

fn main() -> Result<()> {
  let _ = Arc::new(BufferManager::try_new(usize::pow(2, 31))?);
  println!("Hello, world!");
  Ok(())
}
