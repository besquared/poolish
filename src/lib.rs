#![feature(slice_ptr_get)]

mod page;
mod page_class;
mod page_guard;
mod page_manager;

pub use page::*;
pub use page_class::*;
pub use page_guard::*;
pub use page_manager::*;

use std::mem::size_of;

pub const SWIP_LEN: usize = size_of::<usize>();
pub const VLDS_LEN: usize = size_of::<usize>();
pub const HEADER_LEN: usize = SWIP_LEN + VLDS_LEN;
