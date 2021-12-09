use std::ptr::NonNull;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BufferFrame(usize, usize, bool);

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

impl AsMut<BufferFrame> for BufferFrame {
  fn as_mut(&mut self) -> &mut BufferFrame {
    &mut self
  }
}

impl BufferFrame {
  pub fn len(&self) -> usize {
    self.1
  }

  pub fn activate(&mut self) {
    self.2 = true
  }

  pub fn deactivate(&mut self) {
    self.2 = false
  }

  pub fn is_active(&self) -> bool {
    self.2
  }

  pub fn new(buffer: &[u8]) -> Self {
    unsafe {
      Self(std::mem::transmute::<&[u8], usize>(buffer), buffer.len(), false)
    }
  }

  // Internal

  fn as_ptr(&self) -> *const u8 {
    unsafe {
      std::mem::transmute::<usize, *const u8>(self.0)
    }
  }

  fn as_mut_ptr(&self) -> *mut u8 {
    unsafe {
      std::mem::transmute::<usize, *mut u8>(self.0)
    }
  }
}