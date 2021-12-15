//
// If we're going to pass frames across the wire we should
//  consider implementing the latch here instead of in the page
//

#[derive(Clone, Debug)]
pub struct BufferFrame(*const u8, usize, bool);

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

  pub fn is_active(&self) -> bool {
    self.2
  }

  pub fn activate(&mut self) {
    self.2 = true
  }

  pub fn deactivate(&mut self) {
    self.2 = false
  }

  pub fn new(ptr: *const u8, len: usize) -> Self {
    Self(ptr, len, false)
  }

  // Internal

  fn as_ptr(&self) -> *const u8 {
    self.0
  }

  fn as_mut_ptr(&self) -> *mut u8 {
    self.0 as *mut u8
  }
}