use std::{
  sync::atomic::{
    AtomicUsize, Ordering
  }
};

pub const CID_BITS: usize = 0x0006;
pub const CID_MASK: usize = 0x02FF;

pub const TAG_BITS: usize = 0x0001;
pub const TAG_MASK: usize = 0x0001;

#[derive(Debug)]
pub struct FrameSWIP<'a>(&'a AtomicUsize);

impl<'a> From<&'a AtomicUsize> for FrameSWIP<'a> {
  fn from(swip: &'a AtomicUsize) -> Self {
    Self(swip)
  }
}

impl<'a> From<&'a usize> for FrameSWIP<'a> {
  fn from(swip: &'a usize) -> Self {
    Self::from(Self::make_atomic(swip))
  }
}

impl<'a> FrameSWIP<'a> {
  fn swip(&self) -> &AtomicUsize {
    &self.0
  }

  pub fn value(&self) -> usize {
    self.swip().load(Ordering::Acquire)
  }

  pub fn tag(value: usize) -> usize {
    value & TAG_MASK
  }

  pub fn cid(value: usize) -> usize {
    (value & CID_MASK) >> TAG_BITS
  }

  pub fn pid(value: usize) -> usize {
    value >> TAG_BITS >> CID_BITS
  }

  pub fn pack(pid: usize, cid: usize) -> usize {
    Self::pack_pid(Self::pack_cid(Self::pack_tag(0), cid), pid)
  }

  fn pack_tag(value: usize) -> usize {
    (value & !TAG_MASK) | (1 & TAG_MASK)
  }

  fn pack_cid(value: usize, cid: usize) -> usize {
    (value & !CID_MASK) | (cid & CID_MASK)
  }

  fn pack_pid(value: usize, pid: usize) -> usize {
    (value & (TAG_MASK + CID_MASK)) | (pid << TAG_BITS << CID_BITS)
  }

  fn make_atomic(atomic_ref: &usize) -> &AtomicUsize {
    unsafe {
      &(*(atomic_ref as *const usize as *const AtomicUsize))
    }
  }
}
