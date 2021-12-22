#![feature(slice_ptr_get)]

mod page;
mod page_class;
mod page_handle;
mod page_manager;

pub use page::*;
pub use page_class::*;
pub use page_handle::*;
pub use page_manager::*;

pub const SWIP_LEN: usize = std::mem::size_of::<usize>();
pub const VLDS_LEN: usize = std::mem::size_of::<usize>();
pub const HEADER_LEN: usize = SWIP_LEN + VLDS_LEN;
