//! All functionality related to the emulated CPU of the Game Boy.
use crate::{instructions::execute_opcode, Registers};

/// The emulated CPU of the Game Boy.
#[derive(Default, Debug)]
pub struct Cpu {
    regs: Registers,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
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
