//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod boot;
mod cpu;
mod instructions;
mod memory;
mod registers;

// Re-exports
pub use boot::{DMG_BOOT, DMG_BOOT_SIZE};
pub use cpu::Cpu;
pub use memory::MemoryBus;
pub use registers::{Registers, Target, VirtTarget};
