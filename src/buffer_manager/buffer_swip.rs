use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicI64, Ordering};


pub struct BufferSWIP(Arc<AtomicI64>);

impl BufferSWIP {
  pub fn is_swizzled(&self) -> bool {
    self.0.load(Ordering::SeqCst).is_positive()
  }

  pub fn is_unswizzled(&self) -> bool {
    self.0.load(Ordering::SeqCst).is_negative()
  }
}