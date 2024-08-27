//! All functionality related to the emulated CPU of the Game Boy.
use std::default::Default;

use crate::{
    boot::{BOOT, BOOT_CODE_SIZE},
    instructions::execute_opcode,
    Registers,
};

const MEM_SIZE: usize = 0xFFFF;

/// The emulated CPU of the Game Boy.
#[derive(Debug)]
pub struct Cpu {
    regs: Registers,
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
    /// Execute a ROM.
    pub fn execute(&mut self, buffer: &[u8]) {
        while (self.pc as usize) < buffer.len() {
            let opcode = self.step(buffer);
            execute_opcode(self, opcode);
        }
    }

    // TODO
    fn step(&mut self, buffer: &[u8]) -> u8 {
        let opcode = buffer[self.pc as usize];
        self.pc += 1;
        opcode
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
