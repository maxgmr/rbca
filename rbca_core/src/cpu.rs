//! All functionality related to the virtual CPU of the emulator.
use super::Registers;

/// The virtual CPU.
#[derive(Debug, Default)]
pub struct Cpu {
    /// The registers of the CPU.
    pub reg: Registers,
    /// The program counter.
    pub pc: u16,
    /// The stack pointer.
    pub sp: u16,
}
impl Cpu {}
