//! All functionality related to the emulated CPU of the Game Boy.
use std::default::Default;

use crate::{
    boot::{DMG_BOOT, DMG_BOOT_SIZE},
    instructions::execute_opcode,
    Registers,
};

const MEM_SIZE: usize = 0xFFFF;

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
    // TODO temporary
    pub memory: [u8; MEM_SIZE],
}
impl Cpu {
    /// Create a new [Cpu].
    pub fn new() -> Self {
        Self::default()
    }

    // TODO temp test function
    /// Perform one cycle.
    pub fn cycle(&mut self) {
        let opcode = self.memory[self.pc as usize];
        execute_opcode(self, opcode);
    }

    /// Load something into memory.
    pub fn load(&mut self, start_index: u16, data: &[u8]) {
        let end_index = (start_index as usize) + data.len();
        self.memory[(start_index as usize)..end_index].copy_from_slice(data);
    }
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            regs: Registers::default(),
            pc: u16::default(),
            sp: u16::default(),
            memory: [0; MEM_SIZE],
        }
    }
}
