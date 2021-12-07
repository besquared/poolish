use std::{
  io::{Cursor, Read},
  sync::{
    Arc,
    atomic::AtomicU64
  }
};

use anyhow::Result;

use crate::BufferRef;

/**
 *
 * A buffer frame represents a single page-sized block of memory
 *
 */

#[derive(Debug)]
pub struct BufferFrame {
  dirty: bool,
  latch: AtomicU64,
  buffer: Cursor<BufferRef>
}

impl BufferFrame {
  pub fn try_new(buffer: BufferRef) -> Result<Self> {
    let mut buffer = Cursor::new(buffer);

    Ok(Self {
      buffer,
      dirty: false,
      latch: AtomicU64::new(0)
    })
  }

  pub fn page_id(&mut self) -> Result<u64> {
    let mut bytes = [0u8; 8];
    self.buffer.set_position(0);
    self.buffer.read_exact(&mut bytes)?;
    Ok(u64::from_le_bytes(bytes))
  }

  pub fn page_class(&mut self) -> Result<u8> {
    let mut bytes = [0u8; 1];
    self.buffer.set_position(7);
    self.buffer.read_exact(&mut bytes)?;
    Ok(u8::from_le_bytes(bytes))
  }
}