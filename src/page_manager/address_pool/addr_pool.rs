use anyhow::{
  anyhow, Result
};

use memmap2::MmapMut;
use std::sync::Arc;

use super::{ FreePool, UsedPool };

#[derive(Clone, Debug)]
pub struct AddrPool(FreePool, UsedPool);

impl AddrPool {
  fn free_mut(&mut self) -> &mut FreePool {
    &mut self.0
  }

  fn used_mut(&mut self) -> &mut UsedPool {
    &mut self.1
  }

  pub fn alloc(&mut self) -> Option<usize> {
    if let Some(addr) = self.free_mut().pop_front() {
      self.used_mut().insert(addr);
      Some(addr)
    } else {
      None
    }
  }

  pub fn free(&mut self, addr: usize) -> bool {
    if let Some(addr) = self.used_mut().remove(addr) {
      self.free_mut().push_front(addr);
      true
    } else {
      // Should invalid free be an error?
      false
    }
  }


  //
  // How do we put something back into the free queue?
  //  We need to be able to pull it out of used and put
  //  it back into free. Used probably needs to be a Heap
  //

  pub fn try_new(data: Arc<MmapMut>, frame_size: usize) -> Result<Self> {
    let mut free = FreePool::default();
    for offset in (0..data.len()).step_by(frame_size) {
      let address = match data.get(offset) {
        Some(address) => address as *const _ as usize,
        None => return Err(anyhow!("PoolAllocationError: Cannot get reference to byte at offset {}", offset))
      };

      free.push_back(address);
    }

    Ok(Self(free, UsedPool::default()))
  }
}