//! Core library for backend `rbca` functionality.
#![warn(missing_docs)]

mod audio;
mod boot;
mod cartridge;
mod cpu;
mod flags;
mod ie_register;
mod instructions;
mod joypad;
mod mmu;
mod ppu;
mod registers;
mod timer;

// Re-exports
pub use audio::Audio;
pub use boot::{DMG_BOOT, DMG_BOOT_SIZE};
pub use cartridge::Cartridge;
pub use cpu::{Cpu, EmuState};
pub use flags::*;
pub use joypad::{Button, Joypad};
pub use mmu::Mmu;
pub use ppu::{DISPLAY_HEIGHT, DISPLAY_WIDTH, PPU};
pub use registers::{RegFlag, Registers, Target, VirtTarget};
pub use timer::Timer;
