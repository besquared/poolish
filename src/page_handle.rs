mod page_swip;

use crate::{ FrameSWIP };

pub use page_swip::*;

#[derive(Debug)]
pub struct PageHandle<'a>(FrameSWIP<'a>);

impl<'a> From<FrameSWIP<'a>> for PageHandle<'a> {
  fn from(swip: FrameSWIP<'a>) -> Self {
    Self(swip)
  }
}

impl<'a> PageHandle<'a> {
  pub fn swip(&self) -> PageSWIP {
    PageSWIP::from(self.value())
  }

  fn frame_swip(&self) -> &FrameSWIP {
    &self.0
  }

  fn value(&self) -> usize {
    self.frame_swip().value()
  }
}
