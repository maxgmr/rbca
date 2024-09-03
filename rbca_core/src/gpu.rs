use std::default::Default;

const DISPLAY_WIDTH: usize = 160;
const DISPLAY_HEIGHT: usize = 144;

/// Graphics processing unit.
#[derive(Debug, Clone)]
pub struct GPU {
    memory: [u8; 0x10],
}
impl GPU {
    /// Create a new GPU.
    pub fn new() -> Self {
        Self {
            memory: [0x00; 0x10],
        }
    }

    /// Temp function to read byte.
    // TODO
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Temp function to write byte.
    // TODO
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value
    }
}
impl Default for GPU {
    fn default() -> Self {
        Self::new()
    }
}
