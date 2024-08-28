//! All functionality related to the CPU instructions.
use crate::{Cpu, VirtTarget};

/// Execute a given opcode.
pub fn execute_opcode(cpu: &mut Cpu, opcode: u8) {
    match opcode {
        // NOP
        0x00 => nop(cpu),
        // LD n,nn
        0x01 => ld_n_nn(cpu, VirtTarget::BC),
        0x11 => ld_n_nn(cpu, VirtTarget::DE),
        0x21 => ld_n_nn(cpu, VirtTarget::HL),
        0x31 => ld_n_nn_sp(cpu),
        _ => panic!("Unimplemented opcode {:#02X} at {:#04X}", opcode, cpu.pc,),
    }
}

// NOP: Do nothing.
fn nop(cpu: &mut Cpu) {
    cpu.pc += 1;
}

// LD n,nn: Set n = nn.
fn ld_n_nn(cpu: &mut Cpu, target: VirtTarget) {
    let nn = cpu.get_next_2_bytes();
    cpu.regs.set_virt_reg(target, nn);
    cpu.pc += 3;
}
fn ld_n_nn_sp(cpu: &mut Cpu) {
    let nn = cpu.get_next_2_bytes();
    cpu.sp = nn;
    cpu.pc += 3;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_ld_n_nn() {
        let mut cpu = Cpu::new();
        // Load 0x1234 into BC, 0x5678 into DE, 0x9ABC into HL, 0xDEF0 into stack pointer.
        let data: [u8; 12] = [
            0x01, 0x12, 0x34, 0x11, 0x56, 0x78, 0x21, 0x9A, 0xBC, 0x31, 0xDE, 0xF0,
        ];
        cpu.load(0x0000, &data);
        assert_eq!(cpu.pc, 0x0000);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.regs.get_virt_reg(VirtTarget::BC), 0x1234);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0006);
        assert_eq!(cpu.regs.get_virt_reg(VirtTarget::DE), 0x5678);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0009);
        assert_eq!(cpu.regs.get_virt_reg(VirtTarget::HL), 0x9ABC);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x000C);
        assert_eq!(cpu.sp, 0xDEF0);
    }
}
