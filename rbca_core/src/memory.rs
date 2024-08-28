//! Functionality related to emulator memory.
use std::default::Default;

/// Memory bus of the Game Boy.
#[derive(Debug)]
pub struct MemoryBus {
    /// The memory contents.
    pub memory: [u8; 0x10000],
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

    /// Read 2 bytes from the given start address. (little-endian)
    pub fn read_2_bytes(&mut self, start_address: u16) -> u16 {
        (self.read_byte(start_address) as u16) | ((self.read_byte(start_address + 1) as u16) << 8)
    }

    /// Write a byte to a given address.
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }

    /// Write 2 bytes to a given start address. (little-endian)
    pub fn write_2_bytes(&mut self, start_address: u16, value: u16) {
        self.write_byte(start_address, (value & 0x00FF) as u8);
        self.write_byte(start_address + 1, (value >> 8) as u8);
    }
}
impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_read_write() {
        let mut mem_bus = MemoryBus::new();
        mem_bus.write_byte(0x0000, 0x12);
        mem_bus.write_2_bytes(0x0001, 0x3456);
        assert_eq!(mem_bus.read_byte(0x0000), 0x12);
        assert_eq!(mem_bus.read_byte(0x0001), 0x56);
        assert_eq!(mem_bus.read_byte(0x0002), 0x34);
        assert_eq!(mem_bus.read_2_bytes(0x0000), 0x5612);
        assert_eq!(mem_bus.read_2_bytes(0x0001), 0x3456);
        assert_eq!(mem_bus.read_byte(0x0002), 0x34);
        mem_bus.write_byte(0x0001, 0xFB);
        assert_eq!(mem_bus.read_byte(0x0000), 0x12);
        assert_eq!(mem_bus.read_byte(0x0001), 0xFB);
        assert_eq!(mem_bus.read_byte(0x0002), 0x34);
    }
}
