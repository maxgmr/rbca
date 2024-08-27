//! All functionality related to the CPU instructions.
use crate::Cpu;

/// Execute a given opcode.
pub fn execute_opcode(cpu: &mut Cpu, opcode: u8) {
    match opcode {
        0x00 => nop(cpu),
        _ => panic!("Unimplemented opcode {:#02x} at {:#04x}", opcode, cpu.pc,),
    }
}

// NOP: Do nothing.
fn nop(cpu: &mut Cpu) {
    cpu.pc += 1;
}
