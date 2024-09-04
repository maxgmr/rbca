//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod boot;
mod cpu;
mod flags;
mod gpu;
mod ie_register;
mod instructions;
mod io_registers;
mod memory;
mod registers;

// Re-exports
pub use boot::{DMG_BOOT, DMG_BOOT_SIZE};
pub use cpu::Cpu;
pub use flags::*;
pub use gpu::{DISPLAY_HEIGHT, DISPLAY_WIDTH, GPU};
pub use memory::MemoryBus;
pub use registers::{RegFlag, Registers, Target, VirtTarget};
