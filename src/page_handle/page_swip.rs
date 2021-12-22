pub const CID_BITS: usize = 0x0006;
pub const CID_MASK: usize = 0x02FF;

pub const TAG_BITS: usize = 0x0001;
pub const TAG_MASK: usize = 0x0001;

#[derive(Clone, Debug)]
pub enum PageSWIP {
  Fizzled(FizzledPageSWIP),
  Swizzled(SwizzledPageSWIP)
}

impl From<usize> for PageSWIP {
  fn from(value: usize) -> Self {
    if value & TAG_MASK == 1 {
      Self::Fizzled(FizzledPageSWIP::from(value))
    } else {
      Self::Swizzled(SwizzledPageSWIP::from(value))
    }
  }
}

#[derive(Clone, Debug)]
pub struct FizzledPageSWIP(usize);

impl From<usize> for FizzledPageSWIP {
  fn from(swip: usize) -> Self {
    Self(swip)
  }
}

impl FizzledPageSWIP {
  pub fn value(&self) -> usize {
    self.0
  }

  pub fn tag(&self) -> usize {
    self.value() & TAG_MASK
  }

  pub fn cid(&self) -> usize {
    (self.value() & CID_MASK) >> TAG_BITS
  }

  pub fn pid(&self) -> usize {
    self.value() >> TAG_BITS >> CID_BITS
  }

  pub fn pack(pid: usize, cid: usize) -> Self {
    // Build LSB -> MSB
    Self(Self::pack_pid(Self::pack_cid(Self::pack_tag(0), cid), pid))
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
}

#[derive(Clone, Debug)]
pub struct SwizzledPageSWIP(usize);

impl From<usize> for SwizzledPageSWIP {
  fn from(swip: usize) -> Self {
    Self(swip)
  }
}

impl SwizzledPageSWIP {
  pub fn address(&self) -> usize {
    self.0
  }

  pub fn as_ptr(&self) -> *const u8 {
    self.address() as *const u8
  }
}
