//! All functionality related to the CPU instructions.
use crate::{Cpu, RegFlag, Target, VirtTarget};

/// Execute a given opcode.
pub fn execute_opcode(cpu: &mut Cpu, opcode: u8) {
    match opcode {
        // LD nn,n
        // 0x06 =>
        // 0x0E =>
        // 0x16 =>
        // 0x1E =>
        // 0x26 =>
        // 0x2E =>

        // LD r1,r2
        // 0x7F =>
        // 0x78 =>
        // 0x79 =>
        // 0x7A =>
        // 0x7B =>
        // 0x7C =>
        // 0x7D =>
        // 0x7E =>
        // 0x40 =>
        // 0x41 =>
        // 0x42 =>
        // 0x43 =>
        // 0x44 =>
        // 0x45 =>
        // 0x46 =>
        // 0x48 =>
        // 0x49 =>
        // 0x4A =>
        // 0x4B =>
        // 0x4C =>
        // 0x4D =>
        // 0x4E =>
        // 0x50 =>
        // 0x51 =>
        // 0x52 =>
        // 0x53 =>
        // 0x54 =>
        // 0x55 =>
        // 0x56 =>
        // 0x58 =>
        // 0x59 =>
        // 0x5A =>
        // 0x5B =>
        // 0x5C =>
        // 0x5D =>
        // 0x5E =>
        // 0x60 =>
        // 0x61 =>
        // 0x62 =>
        // 0x63 =>
        // 0x64 =>
        // 0x65 =>
        // 0x66 =>
        // 0x68 =>
        // 0x69 =>
        // 0x6A =>
        // 0x6B =>
        // 0x6C =>
        // 0x6D =>
        // 0x6E =>
        // 0x70 =>
        // 0x71 =>
        // 0x72 =>
        // 0x73 =>
        // 0x74 =>
        // 0x75 =>
        // 0x36 =>

        // LD A,n
        // 0x7F =>
        // 0x78 =>
        // 0x79 =>
        // 0x7A =>
        // 0x7B =>
        // 0x7C =>
        // 0x7D =>
        // 0x0A =>
        // 0x1A =>
        // 0x7E =>
        // 0xFA =>
        // 0x3E =>

        // LD n,A
        // 0x7F =>
        // 0x47 =>
        // 0x4F =>
        // 0x57 =>
        // 0x5F =>
        // 0x67 =>
        // 0x6F =>
        // 0x02 =>
        // 0x12 =>
        // 0x77 =>
        // 0xEA =>

        // LD A,(C)
        // 0xF2 =>

        // LD (C),A
        // 0xE2 =>

        // LD A,(HLD) / LD A,(HL-) / LDD A,(HL)
        // 0x3A =>

        // LD (HLD),A / LD (HL-),A / LDD (HL,A)
        0x32 => ld_hld_a(cpu),

        // LD A,(HLI) / LD A,(HL+) / LDI A,(HL)
        // 0x2A =>

        // LD (HLI),A / LD (HL+),A / LDI (HL,A)
        // 0x22 =>

        // LDH (n),A
        // 0xE0 =>

        // LDH A,(n)
        // 0xF0 =>

        // LD n,nn
        0x01 => ld_n_nn(cpu, VirtTarget::BC),
        0x11 => ld_n_nn(cpu, VirtTarget::DE),
        0x21 => ld_n_nn(cpu, VirtTarget::HL),
        0x31 => ld_n_nn_sp(cpu),

        // LD SP,HL
        // 0xF9 =>

        // LD HL,SP+n / LDHL SP,n
        // 0xF8 =>

        // LD (nn),SP
        // 0x08 =>

        // PUSH nn
        // 0xF5 =>
        // 0xC5 =>
        // 0xD5 =>
        // 0xE5 =>

        // POP nn
        // 0xF1 =>
        // 0xC1 =>
        // 0xD1 =>
        // 0xE1 =>

        // ADD A,n
        // 0x87 =>
        // 0x80 =>
        // 0x81 =>
        // 0x82 =>
        // 0x83 =>
        // 0x84 =>
        // 0x85 =>
        // 0x86 =>
        // 0xC6 =>

        // ADC A,n
        // 0x8F =>
        // 0x88 =>
        // 0x89 =>
        // 0x8A =>
        // 0x8B =>
        // 0x8C =>
        // 0x8D =>
        // 0x8E =>
        // 0xCE =>

        // SUB n
        // 0x97 =>
        // 0x90 =>
        // 0x91 =>
        // 0x92 =>
        // 0x93 =>
        // 0x94 =>
        // 0x95 =>
        // 0x96 =>
        // 0xD6 =>

        // SBC A,n
        // 0x9F =>
        // 0x98 =>
        // 0x99 =>
        // 0x9A =>
        // 0x9B =>
        // 0x9C =>
        // 0x9D =>
        // 0x9E =>
        // TODO ??

        // AND n
        // 0xA7 =>
        // 0xA0 =>
        // 0xA1 =>
        // 0xA2 =>
        // 0xA3 =>
        // 0xA4 =>
        // 0xA5 =>
        // 0xA6 =>
        // 0xE6 =>

        // OR n
        // 0xB7 =>
        // 0xB0 =>
        // 0xB1 =>
        // 0xB2 =>
        // 0xB3 =>
        // 0xB4 =>
        // 0xB5 =>
        // 0xB6 =>
        // 0xF6 =>

        // XOR n
        0xAF => xor_n(cpu, Target::A),
        0xA8 => xor_n(cpu, Target::B),
        0xA9 => xor_n(cpu, Target::C),
        0xAA => xor_n(cpu, Target::D),
        0xAB => xor_n(cpu, Target::E),
        0xAC => xor_n(cpu, Target::H),
        0xAD => xor_n(cpu, Target::L),
        // 0xAE =>
        // 0xEE =>

        // CP n
        // 0xBF =>
        // 0xB8 =>
        // 0xB9 =>
        // 0xBA =>
        // 0xBB =>
        // 0xBC =>
        // 0xBD =>
        // 0xBE =>
        // 0xFE =>

        // INC n
        // 0x3C =>
        // 0x04 =>
        // 0x0C =>
        // 0x14 =>
        // 0x1C =>
        // 0x24 =>
        // 0x2C =>
        // 0x34 =>

        // DEC n
        // 0x3D =>
        // 0x05 =>
        // 0x0D =>
        // 0x15 =>
        // 0x1D =>
        // 0x25 =>
        // 0x2D =>
        // 0x35 =>

        // ADD HL,n
        // 0x09 =>
        // 0x19 =>
        // 0x29 =>
        // 0x39 =>

        // ADD SP,n
        // 0xE8 =>

        // INC nn
        // 0x03 =>
        // 0x13 =>
        // 0x23 =>
        // 0x33 =>

        // DEC nn
        // 0x0B =>
        // 0x1B =>
        // 0x2B =>
        // 0x3B =>

        // DAA
        // 0x27 =>

        // CPL
        // 0x2F =>

        // CCF
        // 0x3F =>

        // SCF
        // 0x37 =>

        // NOP
        0x00 => nop(cpu),

        // HALT
        // 0x76 =>

        // STOP
        // 0x10 => need to check next byte == 00

        // DI
        // 0xF3 =>

        // EI
        // 0xFB =>

        // RLCA
        // 0x07 =>

        // RLA
        // 0x17 =>

        // RRCA
        // 0x0F =>

        // RRA
        // 0x1F =>

        // JP nn
        // 0xC3 =>

        // JP cc,nn
        // 0xC2 =>
        // 0xCA =>
        // 0xD2 =>
        // 0xDA =>

        // JP (HL)
        // 0xE9 =>

        // JR n
        // 0x18 =>

        // JR cc,n
        // 0x20 =>
        // 0x28 =>
        // 0x30 =>
        // 0x38 =>

        // CALL nn
        // 0xCD =>

        // CALL cc,nn
        // 0xC4 =>
        // 0xCC =>
        // 0xD4 =>
        // 0xDC =>

        // RST n
        // 0xC7 =>
        // 0xCF =>
        // 0xD7 =>
        // 0xDF =>
        // 0xE7 =>
        // 0xEF =>
        // 0xF7 =>
        // 0xFF =>

        // RET
        // 0xC9 =>

        // RET cc
        // 0xC0 =>
        // 0xC8 =>
        // 0xD0 =>
        // 0xD8 =>

        // RETI
        // 0xD9 =>
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

// XOR n: Set A = A XOR n.
fn xor_n(cpu: &mut Cpu, target: Target) {
    cpu.regs.reset_flags();
    let result = cpu.regs.get_reg(Target::A) ^ cpu.regs.get_reg(target);
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_reg(Target::A, result);
    cpu.pc += 1;
}

// LD (HLD),A: (HL) = A. HL -= 1.
fn ld_hld_a(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(VirtTarget::HL);
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(Target::A));
    cpu.regs.set_virt_reg(VirtTarget::HL, address - 1);
    cpu.pc += 1;
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

    #[test]
    fn test_xor_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(Target::A, 0b1010_1010);
        cpu.regs.set_reg(Target::B, 0b0011_1100);
        execute_opcode(&mut cpu, 0xA8);
        assert_eq!(cpu.regs.get_reg(Target::A), 0b1001_0110);
        assert_eq!(cpu.regs.get_reg(Target::B), 0b0011_1100);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(Target::C, 0b1001_0110);
        execute_opcode(&mut cpu, 0xA9);
        assert_eq!(cpu.regs.get_reg(Target::A), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(Target::D, 0b1111_0000);
        execute_opcode(&mut cpu, 0xAA);
        assert_eq!(cpu.regs.get_reg(Target::A), 0b1111_0000);
        cpu.regs.set_reg(Target::E, 0b0000_1111);
        execute_opcode(&mut cpu, 0xAB);
        assert_eq!(cpu.regs.get_reg(Target::A), 0b1111_1111);
        cpu.regs.set_reg(Target::H, 0b0100_0010);
        execute_opcode(&mut cpu, 0xAC);
        assert_eq!(cpu.regs.get_reg(Target::A), 0b1011_1101);
        cpu.regs.set_reg(Target::L, 0b0011_1100);
        execute_opcode(&mut cpu, 0xAD);
        assert_eq!(cpu.regs.get_reg(Target::A), 0b1000_0001);
    }

    #[test]
    fn test_ld_hld_a() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(VirtTarget::HL, 0x1234);
        cpu.regs.set_reg(Target::A, 0xDC);
        execute_opcode(&mut cpu, 0x32);
        assert_eq!(cpu.mem_bus.read_byte(0x1234), 0xDC);
    }
}
