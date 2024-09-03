pub mod io_registers;

pub trait MemorySection {
    /// Return the start index of this [MemorySection] within greater memory.
    fn start_index(&self) -> u16;
    /// Return the size of this [MemorySection].
    fn size(&self) -> usize;
    /// Write a byte to this [MemorySection].
    fn write_byte(&mut self, address: u16, byte: u8);
    /// Read a byte from this [MemorySection].
    fn read_byte(&self, address: u16) -> u8;
    /// Write two bytes to this [MemorySection].
    fn write_word(&mut self, start_address: u16, word: u16);
    /// Read two bytes from this [MemorySection].
    fn read_word(&mut self, start_address: u16) -> u16;
}

/// A section of memory without any special attributes.
#[derive(Debug)]
pub struct StandardMemorySection<const N: usize> {
    /// The contents of the memory.
    contents: [u8; N],
    /// The start index of the memory.
    start_index: u16,
}
impl MemorySection for StandardMemorySection<N: Sized> {
    fn start_index(&self) -> u16 {
        self.start_index
    }

    fn size(&self) -> usize {
        self.contents.len()
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        self.contents[(address - self.start_index) as usize] = byte;
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.contents[(address - self.start_index) as usize]
    }

    fn write_word(&mut self, start_address: u16, word: u16) {
        self.write_byte(start_address - self.start_index, (word & 0x00FF) as u8);
        self.write_byte((start_address - self.start_index) + 1, (word >> 8) as u8);
    }

    fn read_word(&mut self, start_address: u16) -> u16 {
        (self.read_byte(start_address - self.start_index) as u16)
            | ((self.read_byte((start_address - self.start_index) + 1) as u16) << 8)
    }
}
