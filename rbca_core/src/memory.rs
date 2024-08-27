//! Memory-related functionality.
use std::default::Default;

/// The 16-bit memory address bus. Addresses ROM, RAM, & I/O.
#[derive(Debug)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}
impl MemoryBus {
    /// Read a single byte from memory at the given 16-bit address.
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0_u8; 0xFFFF],
        }
    }
}
