/// Enum for access of I/O registers.
pub enum IOReg {}

/// I/O registers of memory.
#[derive(Debug, Clone)]
pub struct IORegisters {
    contents: [u8; 0x0080],
}
impl IORegisters {
    /// Create new [IORegisters].
    pub fn new() -> Self {
        Self {
            contents: [0x00; 0x0080],
        }
    }

    /// Directly retrieve the byte at the given address. Not recommended.
    pub fn read_byte(&self, address: u16) -> u8 {
        self.contents[address as usize]
    }

    /// Directly replace the byte at the given address. Not recommended.
    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.contents[address as usize] = byte;
    }
}
