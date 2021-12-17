//
// If we're going to pass frames across the wire we should
//  consider implementing the latch here instead of in the page
//

use anyhow::Result;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

/**
 *
 * Memory Layout
 *
 * +-----------------------------+
 * | Field   | Offset |   Length |  Description
 * |---------+--------+----------|
 * | PID     |      0 |        8 |  Logical Page ID
 * | Class   |      8 |        1 |  Length is 2^Class bytes
 * | Dirty   |      9 |        1 |  0 for clean, >0 for dirty
 * | Latch   |     10 |        8 |  0 for no locks, 1 for exclusive, N where N > 1 for shared
 * | Data    |     18 | LEN - 18 |  Bytes minus the 18 byte header
 * +-----------------------------+
 *
 */

const PID_LEN: usize = 8;
const CLASS_LEN: usize = 1;
const DIRTY_LEN: usize = 1;
const LATCH_LEN: usize = 8;
const HEADER_LEN: usize = PID_LEN + CLASS_LEN + DIRTY_LEN + LATCH_LEN;

#[derive(Clone, Debug)]
pub struct BufferFrame(*const u8, usize);

// Allow passing frames between threads
//  This works because all interaction with a
//  frame is done via a latch in the buffer page
unsafe impl Send for BufferFrame {}
unsafe impl Sync for BufferFrame {}

impl AsRef<[u8]> for BufferFrame {
  fn as_ref(&self) -> &[u8] {
    unsafe {
      std::slice::from_raw_parts(self.as_ptr(), self.len())
    }
  }
}

impl AsMut<[u8]> for BufferFrame {
  fn as_mut(&mut self) -> &mut [u8] {
    unsafe {
      std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
    }
  }
}

impl BufferFrame {
  pub fn len(&self) -> usize {
    self.1
  }

  pub fn pid(&self) -> i64 {
    let b = self.as_ref();
    i64::from_le_bytes([ b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7] ])
  }

  pub fn pid_ref(&mut self) -> &mut u8 {
    self.as_mut().get_mut(0).unwrap()
  }

  pub fn class_ref(&mut self) -> &mut u8 {
    self.as_mut().get_mut(8).unwrap()
  }

  pub fn dirty_ref(&mut self) -> &mut u8 {
    self.as_mut().get_mut(9).unwrap()
  }

  pub fn latch_ref(&mut self) -> &mut u8 {
    self.as_mut().get_mut(10).unwrap()
  }

  // todo: if dest is longer than the frame then only read up to the end of the frame
  pub fn read<W: Write>(&self, offset: usize, len: usize, dest: &mut W) -> Result<usize> {
    let bytes = self.as_ref();
    let offset = HEADER_LEN + offset;
    Ok(dest.write(&bytes[offset..offset + len])?)
  }

  pub fn write<R: Read>(&mut self, offset: usize, len: usize, src: &mut R) -> Result<usize> {
    let bytes = self.as_mut();
    let offset = HEADER_LEN + offset;
    Ok(src.read(&mut bytes[offset .. offset + len])?)
  }

  pub fn new(ptr: *const u8, len: usize) -> Self {
    Self(ptr, len)
  }

  pub fn activate(&mut self, pid: i64, class: u8, dirty: u8, latch: u64) -> Result<()> {
    let mut cursor = Cursor::new(self.as_mut());

    cursor.write(&pid.to_le_bytes())?;
    cursor.write(&class.to_le_bytes())?;
    cursor.write(&dirty.to_le_bytes())?;
    cursor.write(&latch.to_le_bytes())?;

    Ok(())
  }

  // Internal

  fn as_ptr(&self) -> *const u8 {
    self.0
  }

  fn as_mut_ptr(&self) -> *mut u8 {
    self.0 as *mut u8
  }
}

impl std::fmt::Binary for BufferFrame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let bytes = self.as_ref();
    for offset in 0..self.len() {
      std::fmt::Binary::fmt(&bytes[offset], f)?
    }
    Ok(())
  }
}
