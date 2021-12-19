#![feature(slice_ptr_get)]

mod page;
mod page_class;
mod page_handle;
mod page_manager;

pub use page::*;
pub use page_class::*;
pub use page_handle::*;
pub use page_manager::*;

/**
 *
 * Memory Layout
 *
 * +-----------------------------+
 * | Field   | Offset |   Length |  Description
 * |---------+--------+----------|
 * | PID     |      0 |        8 |  Logical Page ID
 * | Class   |      8 |        1 |  Length is 2^Class bytes
 * | Dirty   |      9 |        1 |  0 for clean, >0 for dirty
 * | Latch   |     10 |        8 |  0 for no locks, 1 for exclusive, N where N > 1 for shared
 * | Data    |     18 | LEN - 18 |  Bytes minus the 18 byte header
 * +-----------------------------+
 *
 */

pub const PID_LEN: u8 = 8;
pub const CLASS_LEN: u8 = 1;
pub const DIRTY_LEN: u8 = 1;
pub const LATCH_LEN: u8 = 8;
pub const HEADER_LEN: u8 = PID_LEN + CLASS_LEN + DIRTY_LEN + LATCH_LEN;
