#![feature(slice_ptr_get)]

mod page;
mod page_class;
mod page_handle;
mod page_manager;

pub use page::*;
pub use page_class::*;
pub use page_handle::*;
pub use page_manager::*;

pub const PID_LEN: usize = 8;
pub const CLASS_LEN: usize = 1;
pub const DIRTY_LEN: usize = 1;
pub const LATCH_LEN: usize = 8;
pub const HEADER_LEN: usize = PID_LEN + CLASS_LEN + DIRTY_LEN + LATCH_LEN;
