//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod cpu;
mod memory;
mod registers;

// Re-imports
pub use cpu::Cpu;
pub use memory::MemoryBus;
pub use registers::Registers;
