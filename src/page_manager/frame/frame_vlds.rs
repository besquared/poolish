use std::sync::atomic::{ AtomicUsize, Ordering };

pub const DIRTY_BITS: usize = 0x01;
pub const DIRTY_MASK: usize = 0x01;   // 0001

pub const LATCH_BITS: usize = 0x0F;
pub const LATCH_MASK: usize = 0xFFFE; // 1111_1111_1111_1110

#[derive(Debug)]
pub struct FrameVLDS<'a>(&'a AtomicUsize);

impl<'a> From<&'a AtomicUsize> for FrameVLDS<'a> {
  fn from(vlds: &'a AtomicUsize) -> Self {
    Self(vlds)
  }
}

impl<'a> From<&'a usize> for FrameVLDS<'a> {
  fn from(vlds: &'a usize) -> Self {
    Self::from(Self::make_atomic(vlds))
  }
}

impl<'a> FrameVLDS<'a> {
  // Version 0, Open Latch, Dirty page
  pub fn default_value() -> usize {
    1usize
  }

  pub fn dirty(value: usize) -> usize {
    value & DIRTY_MASK
  }

  pub fn latch(value: usize) -> usize {
    (value & LATCH_MASK) >> DIRTY_BITS
  }

  pub fn version(value: usize) -> usize {
    value >> DIRTY_BITS >> LATCH_BITS
  }

  pub fn is_open(latch: usize) -> bool {
    latch == 0
  }

  pub fn is_shared(latch: usize) -> bool {
    latch > 1
  }

  pub fn is_exclusive(latch: usize) -> bool {
    latch == 1
  }

  // Private Helpers

  fn make_atomic(state_ref: &usize) -> &AtomicUsize {
    unsafe {
      &(*(state_ref as *const usize as *const AtomicUsize))
    }
  }

  fn pack_dirty(value: usize, dirty: usize) -> usize {
    (value & !DIRTY_MASK) | (dirty & DIRTY_MASK)
  }

  fn pack_latch(value: usize, latch: usize) -> usize {
    (value & !LATCH_MASK) | (latch & LATCH_MASK)
  }

  fn pack_version(value: usize, version: usize) -> usize {
    (value & (DIRTY_MASK + LATCH_MASK)) | (version << DIRTY_BITS << LATCH_BITS)
  }
}

impl<'a> FrameVLDS<'a> {
  fn vlds(&self) -> &AtomicUsize {
    self.0
  }

  pub fn value(&self) -> usize {
    self.vlds().load(Ordering::Acquire)
  }

  pub fn mark_clean(&self) -> Result<usize, usize> {
    let value = self.value();
    let new_value = Self::pack_dirty(1, 0);
    self.vlds().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }

  pub fn mark_dirty(&self) -> Result<usize, usize> {
    let value = self.value();
    let new_value = Self::pack_dirty(1, 1);
    self.vlds().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }

  pub fn latch_read(&self) -> Result<usize, usize> {
    let value = self.value();
    let latch = Self::latch(value);
    let new_value = Self::pack_latch(1, latch + 1);
    self.vlds().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }

  pub fn latch_open(&self) -> Result<usize, usize> {
    let value = self.value();
    let new_value = Self::pack_latch(value, 0);
    self.vlds().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }

  pub fn latch_write(&self) -> Result<usize, usize> {
    let value = self.value();
    let new_value = Self::pack_latch(value, 1);
    self.vlds().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }

  pub fn increment_version(&self) -> Result<usize, usize> {
    let value = self.value();
    let version = Self::version(value);
    let new_value = Self::pack_version(value, version + 1);
    self.vlds().compare_exchange(value, new_value, Ordering::SeqCst, Ordering::Acquire)
  }
}
