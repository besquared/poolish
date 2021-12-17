#![feature(slice_ptr_get)]

use anyhow::Result;

fn main() -> Result<()> {
  // let manager = build_buffer_manager()?;
  //
  // let mut pages = manager.lock();
  // let mut handle = pages.new_handle();
  // println!("[main] page handle {:?}", handle);
  //
  // let mut page_latch = pages.try_alloc(&mut handle, 12).unwrap();
  // println!("[main] page latch {:?}", page_latch);
  //
  // println!("Frame is {:b}", &page_latch.frame());
  //
  // let mut page_guard = page_latch.acquire_exclusive()?;
  // println!("[main] guard is {:?}", page_guard);
  //
  // let data = vec![ 1u8, 2u8, 3u8 ];
  // let mut data = Cursor::new(data);
  //
  // page_guard.write(0, 3, &mut data)?;
  //
  // println!("Frame is {:b}", &page_guard.frame());
  //
  Ok(())
}
