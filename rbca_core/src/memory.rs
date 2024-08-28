//! Functionality related to emulator memory.
use std::default::Default;

/// Memory bus of the Game Boy.
#[derive(Debug)]
pub struct MemoryBus {
    /// The memory contents.
    pub memory: [u8; 0xFFFF],
}
impl MemoryBus {
    /// Create a new [MemoryBus] initialised to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the byte at the given address.
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Write a byte to a given address.
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}
