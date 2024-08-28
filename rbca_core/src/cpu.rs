//! All functionality related to the emulated CPU of the Game Boy.
use std::default::Default;

use crate::{instructions::execute_opcode, MemoryBus, Registers};

/// The emulated CPU of the Game Boy.
#[derive(Debug)]
pub struct Cpu {
    /// Registers
    pub regs: Registers,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
    /// Memory
    pub mem_bus: MemoryBus,
}
impl Cpu {
    /// Create a new [Cpu].
    pub fn new() -> Self {
        Self::default()
    }

    // TODO temp test function
    /// Perform one cycle.
    pub fn cycle(&mut self) {
        let opcode = self.mem_bus.memory[self.pc as usize];
        execute_opcode(self, opcode);
    }

    /// Load something into memory.
    pub fn load(&mut self, start_index: u16, data: &[u8]) {
        let end_index = (start_index as usize) + data.len();
        self.mem_bus.memory[(start_index as usize)..end_index].copy_from_slice(data);
    }

    /// Get next two bytes.
    // TODO might be little endian!!!
    pub fn get_next_2_bytes(&mut self) -> u16 {
        ((self.mem_bus.memory[(self.pc as usize) + 1] as u16) << 8)
            | (self.mem_bus.memory[(self.pc as usize) + 2] as u16)
    }
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            regs: Registers::default(),
            pc: u16::default(),
            sp: u16::default(),
            mem_bus: MemoryBus::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_next_2_bytes() {
        let mut cpu = Cpu::new();
        let data = [0x01, 0x23, 0x45, 0x67];
        cpu.load(0x0000, &data);
        assert_eq!(cpu.get_next_2_bytes(), 0x2345);
    }
}
