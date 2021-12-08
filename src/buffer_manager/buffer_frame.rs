use std::ptr::NonNull;

#[derive(Clone, Debug)]
pub struct BufferFrame(NonNull<[u8]>, usize);

impl AsRef<[u8]> for BufferFrame {
  fn as_ref(&self) -> &[u8] {
    unsafe {
      let as_ref = self.pointer().as_ref();
      std::slice::from_raw_parts(as_ref.as_ptr(), self.1)
    }
  }
}

impl AsMut<[u8]> for BufferFrame {
  fn as_mut(&mut self) -> &mut [u8] {
    unsafe {
      let as_mut_ptr = self.pointer().as_mut_ptr();
      std::slice::from_raw_parts_mut(as_mut_ptr, self.1)
    }
  }
}

impl BufferFrame {
  fn pointer(&self) -> NonNull<[u8]> {
    self.0
  }

  pub fn new(buffer: &[u8]) -> Self {
    Self(NonNull::from(buffer), buffer.len())
  }
}