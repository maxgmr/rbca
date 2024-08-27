//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod cpu;
mod registers;

// Re-exports
pub use cpu::Cpu;
pub use registers::Registers;
