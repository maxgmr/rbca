//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod boot;
mod cpu;
mod flags;
mod ie_register;
mod instructions;
mod io_registers;
mod memory;
mod ppu;
mod registers;

// Re-exports
pub use boot::{DMG_BOOT, DMG_BOOT_SIZE};
pub use cpu::Cpu;
pub use flags::*;
pub use memory::MemoryBus;
pub use ppu::{DISPLAY_HEIGHT, DISPLAY_WIDTH, PPU};
pub use registers::{RegFlag, Registers, Target, VirtTarget};
