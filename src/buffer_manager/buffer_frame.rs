use std::ptr::NonNull;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct BufferFrame(Arc<NonNull<[u8]>>, usize, bool);

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
    Self(Arc::new(NonNull::from(buffer)), buffer.len(), false)
  }

  // Internal

  fn as_ptr(&self) -> *const u8 {
    self.0.as_mut_ptr()
  }

  fn as_mut_ptr(&self) -> *mut u8 {
    self.0.as_mut_ptr()
  }
}