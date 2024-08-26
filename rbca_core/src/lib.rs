//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod cpu;
mod instructions;
mod registers;

// Re-imports
pub use registers::Registers;
