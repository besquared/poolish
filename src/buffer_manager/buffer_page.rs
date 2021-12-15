mod page_guard;
mod page_handle;
mod page_latch;

use std::{
  io::{Cursor, Seek, SeekFrom, Write},
  sync::atomic::AtomicU64
};

use anyhow::{ anyhow, Result };

use crate::BufferFrame;

pub use page_guard::*;
pub use page_handle::*;
pub use page_latch::*;

/**
 *
 * Memory Layout
 *
 * +-----------------------------+
 * | Field   | Offset |   Length |
 * |---------+--------+----------|
 * | ID      |      0 |        8 |
 * | Class   |      8 |        1 |
 * | Dirty   |      9 |        1 |
 * | Latch   |     10 |        8 |
 * | Data    |     18 |      ... |
 * +-----------------------------+
 *
 */

const HEADER_LEN: usize = 34usize;

#[derive(Debug)]
pub struct BufferPage<'a> {
  frame: BufferFrame,
  latch: PageLatch<'a>
}

/**
 *
 * We don't need to do reclamation as long as version ALWAYS increments even if new pages are put in
 *
 */

impl<'a> BufferPage<'a> {
  pub fn id(&self) -> u64 {
    let bytes = self.bytes();

    u64::from_le_bytes([
      bytes[0], bytes[1], bytes[2], bytes[3],
      bytes[4], bytes[5], bytes[6], bytes[7]
    ])
  }

  pub fn class(&self) -> u8 {
    let bytes = self.bytes();
    u8::from_le_bytes([bytes[8]])
  }

  pub fn dirty(&self) -> u8 {
    let bytes = self.bytes();
    u8::from_le_bytes([bytes[9]])
  }

  pub fn latch(&self) -> &PageLatch {
    &self.latch
  }

  pub fn version(&self) -> u64 {
    self.latch().version()
  }

  pub fn bytes(&self) -> &[u8] {
    self.frame.as_ref()
  }

  pub fn bytes_mut(&mut self) -> &mut [u8] {
    self.frame.as_mut()
  }

  pub fn read<W: AsRef<[u8]> + Write>(&self, offset: usize, dest: &mut W) -> Result<usize> {
    let bytes = self.bytes();
    let offset = offset + HEADER_LEN;
    Ok(dest.write(&bytes[offset..offset + dest.as_ref().len()])?)
  }

  pub fn try_load(mut frame: BufferFrame) -> Result<Self> {
    let bytes = frame.as_mut();

    let latch = unsafe {
      let pointer = &mut bytes[10];
      make_atomic_u64(std::mem::transmute(pointer))
    };

    let latch = PageLatch::new(latch);

    Ok(Self { latch, frame })
  }

  pub fn try_alloc(class: u8, handle: &mut PageHandle, mut frame: BufferFrame) -> Result<Self> {
    if handle.is_swizzled() {
      return Err(anyhow!("Cannot allocate an already allocated page handle {:?}", handle))
    }

    let pid = handle.value();
    let bytes = frame.as_mut();

    // swizzle page handle here
    handle.swizzle(bytes.as_ref().as_ptr() as u64);

    let mut cursor = Cursor::new(bytes);

    // Seek to frame header
    cursor.seek(SeekFrom::Start(0))?;

    // Write header information
    cursor.write(&pid.to_le_bytes())?;
    cursor.write(&class.to_le_bytes())?;
    cursor.write(&1u8.to_le_bytes())?;
    cursor.write(&0u64.to_le_bytes())?;
    cursor.write(&0u64.to_le_bytes())?;

    Self::try_load(frame)
  }
}

fn make_atomic_u64(src: &mut u64) -> &AtomicU64 {
  unsafe {
    &*(src as *mut u64 as *const AtomicU64)
  }
}

// // if we have a mut reference, it must have unique ownership over the
// // referenced data, so we can safely cast that into an immutable reference
// // to AtomicI64
// fn make_atomic_i64<'a>(src: &'a mut i64) -> &'a AtomicI64 {
//   unsafe {
//     &*(src as *mut i64 as *const AtomicI64)
//   }
// }
//
// // if we have a mut pointer, we have no guarantee of ownership or lifetime, and
// // therefore it's unsafe to cast into an immutable reference to AtomicI64
// unsafe fn make_ptr_atomic_i64<'a>(src: *mut i64) -> &'a AtomicU64 {
//   &*(src as *const AtomicU64)
// }

// use std::sync::atomic::{AtomicI64, Ordering};
//
// fn main() -> () {
//   // declare underlying buffer
//   let mut v = vec![1i64, 2i64];
//
//   {
//     // get atomic safely
//     let atomic = make_atomic_i64(&mut v[0]);
//
//     // try to access atomic
//     println!("{}", atomic.swap(10, Ordering::Relaxed)); // = 1
//   }
//
//   unsafe {
//     // get atomic unsafely
//     let atomic = make_ptr_atomic_i64(&mut v[0] as *mut i64);
//
//     // try to access atomic
//     println!("{}", atomic.swap(100, Ordering::Relaxed)); // = 10
//   }
//
//   // print final state of variable
//   println!("{}", v[0]); // = 100
// }
