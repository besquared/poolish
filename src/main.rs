mod buffer_page;
mod buffer_manager;
mod storage_manager;

use anyhow::Result;

use buffer_manager::BufferManager;

fn main() -> Result<()> {
  let pool = BufferManager::try_new()?;
  println!("Hello, world!");

  Ok(())
}
