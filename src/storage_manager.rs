use std::sync::atomic::AtomicU64;

pub struct BufferPage<'a> {
  pid: u64,
  size: u8,
  lock: AtomicU64,
  data: &'a mut [u8]
}