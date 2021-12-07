use std::ptr::NonNull;
use std::sync::Arc;
use memmap2::MmapMut;

#[derive(Clone, Debug)]
pub struct BufferRef(NonNull<[u8]>, usize);

impl AsRef<[u8]> for BufferRef {
  fn as_ref(&self) -> &[u8] {
    unsafe { self.0.as_ref() }
  }
}

impl AsMut<[u8]> for BufferRef {
  fn as_mut(&mut self) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr(), self.1) }
  }
}

impl BufferRef {
  pub fn new(buffer: &[u8]) -> Self {
    Self(NonNull::from(buffer), buffer.len())
  }
}