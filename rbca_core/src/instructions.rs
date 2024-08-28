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
        0xC3 => jp_nn(cpu),

        // JP cc,nn
        0xC2 => jp_cc_nn(cpu, RegFlag::Z, false),
        0xCA => jp_cc_nn(cpu, RegFlag::Z, true),
        0xD2 => jp_cc_nn(cpu, RegFlag::C, false),
        0xDA => jp_cc_nn(cpu, RegFlag::C, true),

        // JP (HL)
        0xE9 => jp_hl(cpu),

        // JR n
        0x18 => jr_n(cpu),

        // JR cc,n
        0x20 => jr_cc_n(cpu, RegFlag::Z, false),
        0x28 => jr_cc_n(cpu, RegFlag::Z, true),
        0x30 => jr_cc_n(cpu, RegFlag::C, false),
        0x38 => jr_cc_n(cpu, RegFlag::C, true),

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

        // CB-Opcodes
        0xCB => {
            let ext_opcode = cpu.mem_bus.read_byte(cpu.pc + 1);
            match ext_opcode {
                // SWAP n
                // 0x37 =>
                // 0x30 =>
                // 0x31 =>
                // 0x32 =>
                // 0x33 =>
                // 0x34 =>
                // 0x35 =>
                // 0x36 =>

                // RLC n
                // 0x07 =>
                // 0x00 =>
                // 0x01 =>
                // 0x02 =>
                // 0x03 =>
                // 0x04 =>
                // 0x05 =>
                // 0x06 =>

                // RL n
                // 0x17 =>
                // 0x10 =>
                // 0x11 =>
                // 0x12 =>
                // 0x13 =>
                // 0x14 =>
                // 0x15 =>
                // 0x16 =>

                // RRC n
                // 0x0F =>
                // 0x08 =>
                // 0x09 =>
                // 0x0A =>
                // 0x0B =>
                // 0x0C =>
                // 0x0D =>
                // 0x0E =>

                // RR n
                // 0x1F =>
                // 0x18 =>
                // 0x19 =>
                // 0x1A =>
                // 0x1B =>
                // 0x1C =>
                // 0x1D =>
                // 0x1E =>

                // SLA n
                // 0x27 =>
                // 0x20 =>
                // 0x21 =>
                // 0x22 =>
                // 0x23 =>
                // 0x24 =>
                // 0x25 =>
                // 0x26 =>

                // SRA n
                // 0x2F =>
                // 0x28 =>
                // 0x29 =>
                // 0x2A =>
                // 0x2B =>
                // 0x2C =>
                // 0x2D =>
                // 0x2E =>

                // SRL n
                // 0x3F =>
                // 0x38 =>
                // 0x39 =>
                // 0x3A =>
                // 0x3B =>
                // 0x3C =>
                // 0x3D =>
                // 0x3E =>

                // BIT 0,r
                0x47 => bit_b_r(cpu, 0, Target::A),
                0x40 => bit_b_r(cpu, 0, Target::B),
                0x41 => bit_b_r(cpu, 0, Target::C),
                0x42 => bit_b_r(cpu, 0, Target::D),
                0x43 => bit_b_r(cpu, 0, Target::E),
                0x44 => bit_b_r(cpu, 0, Target::H),
                0x45 => bit_b_r(cpu, 0, Target::L),
                0x46 => bit_b_r_hl(cpu, 0),

                // BIT 1,r
                0x4F => bit_b_r(cpu, 1, Target::A),
                0x48 => bit_b_r(cpu, 1, Target::B),
                0x49 => bit_b_r(cpu, 1, Target::C),
                0x4A => bit_b_r(cpu, 1, Target::D),
                0x4B => bit_b_r(cpu, 1, Target::E),
                0x4C => bit_b_r(cpu, 1, Target::H),
                0x4D => bit_b_r(cpu, 1, Target::L),
                0x4E => bit_b_r_hl(cpu, 1),

                // BIT 2,r
                0x57 => bit_b_r(cpu, 2, Target::A),
                0x50 => bit_b_r(cpu, 2, Target::B),
                0x51 => bit_b_r(cpu, 2, Target::C),
                0x52 => bit_b_r(cpu, 2, Target::D),
                0x53 => bit_b_r(cpu, 2, Target::E),
                0x54 => bit_b_r(cpu, 2, Target::H),
                0x55 => bit_b_r(cpu, 2, Target::L),
                0x56 => bit_b_r_hl(cpu, 2),

                // BIT 3,r
                0x5F => bit_b_r(cpu, 3, Target::A),
                0x58 => bit_b_r(cpu, 3, Target::B),
                0x59 => bit_b_r(cpu, 3, Target::C),
                0x5A => bit_b_r(cpu, 3, Target::D),
                0x5B => bit_b_r(cpu, 3, Target::E),
                0x5C => bit_b_r(cpu, 3, Target::H),
                0x5D => bit_b_r(cpu, 3, Target::L),
                0x5E => bit_b_r_hl(cpu, 3),

                // BIT 4,r
                0x67 => bit_b_r(cpu, 4, Target::A),
                0x60 => bit_b_r(cpu, 4, Target::B),
                0x61 => bit_b_r(cpu, 4, Target::C),
                0x62 => bit_b_r(cpu, 4, Target::D),
                0x63 => bit_b_r(cpu, 4, Target::E),
                0x64 => bit_b_r(cpu, 4, Target::H),
                0x65 => bit_b_r(cpu, 4, Target::L),
                0x66 => bit_b_r_hl(cpu, 4),

                // BIT 5,r
                0x6F => bit_b_r(cpu, 5, Target::A),
                0x68 => bit_b_r(cpu, 5, Target::B),
                0x69 => bit_b_r(cpu, 5, Target::C),
                0x6A => bit_b_r(cpu, 5, Target::D),
                0x6B => bit_b_r(cpu, 5, Target::E),
                0x6C => bit_b_r(cpu, 5, Target::H),
                0x6D => bit_b_r(cpu, 5, Target::L),
                0x6E => bit_b_r_hl(cpu, 5),

                // BIT 6,r
                0x77 => bit_b_r(cpu, 6, Target::A),
                0x70 => bit_b_r(cpu, 6, Target::B),
                0x71 => bit_b_r(cpu, 6, Target::C),
                0x72 => bit_b_r(cpu, 6, Target::D),
                0x73 => bit_b_r(cpu, 6, Target::E),
                0x74 => bit_b_r(cpu, 6, Target::H),
                0x75 => bit_b_r(cpu, 6, Target::L),
                0x76 => bit_b_r_hl(cpu, 6),

                // BIT 7,r
                0x7F => bit_b_r(cpu, 7, Target::A),
                0x78 => bit_b_r(cpu, 7, Target::B),
                0x79 => bit_b_r(cpu, 7, Target::C),
                0x7A => bit_b_r(cpu, 7, Target::D),
                0x7B => bit_b_r(cpu, 7, Target::E),
                0x7C => bit_b_r(cpu, 7, Target::H),
                0x7D => bit_b_r(cpu, 7, Target::L),
                0x7E => bit_b_r_hl(cpu, 7),

                // SET 0,r
                // 0xC7 =>
                // 0xC0 =>
                // 0xC1 =>
                // 0xC2 =>
                // 0xC3 =>
                // 0xC4 =>
                // 0xC5 =>
                // 0xC6 =>

                // SET 1,r
                // 0xCF =>
                // 0xC8 =>
                // 0xC9 =>
                // 0xCA =>
                // 0xCB =>
                // 0xCC =>
                // 0xCD =>
                // 0xCE =>

                // SET 2,r
                // 0xD7 =>
                // 0xD0 =>
                // 0xD1 =>
                // 0xD2 =>
                // 0xD3 =>
                // 0xD4 =>
                // 0xD5 =>
                // 0xD6 =>

                // SET 3,r
                // 0xDF =>
                // 0xD8 =>
                // 0xD9 =>
                // 0xDA =>
                // 0xDB =>
                // 0xDC =>
                // 0xDD =>
                // 0xDE =>

                // SET 4,r
                // 0xE7 =>
                // 0xE0 =>
                // 0xE1 =>
                // 0xE2 =>
                // 0xE3 =>
                // 0xE4 =>
                // 0xE5 =>
                // 0xE6 =>

                // SET 5,r
                // 0xEF =>
                // 0xE8 =>
                // 0xE9 =>
                // 0xEA =>
                // 0xEB =>
                // 0xEC =>
                // 0xED =>
                // 0xEE =>

                // SET 6,r
                // 0xF7 =>
                // 0xF0 =>
                // 0xF1 =>
                // 0xF2 =>
                // 0xF3 =>
                // 0xF4 =>
                // 0xF5 =>
                // 0xF6 =>

                // SET 7,r
                // 0xFF =>
                // 0xF8 =>
                // 0xF9 =>
                // 0xFA =>
                // 0xFB =>
                // 0xFC =>
                // 0xFD =>
                // 0xFE =>

                // RES 0,r
                // 0x87 =>
                // 0x80 =>
                // 0x81 =>
                // 0x82 =>
                // 0x83 =>
                // 0x84 =>
                // 0x85 =>
                // 0x86 =>

                // RES 1,r
                // 0x8F =>
                // 0x88 =>
                // 0x89 =>
                // 0x8A =>
                // 0x8B =>
                // 0x8C =>
                // 0x8D =>
                // 0x8E =>

                // RES 2,r
                // 0x97 =>
                // 0x90 =>
                // 0x91 =>
                // 0x92 =>
                // 0x93 =>
                // 0x94 =>
                // 0x95 =>
                // 0x96 =>

                // RES 3,r
                // 0x9F =>
                // 0x98 =>
                // 0x99 =>
                // 0x9A =>
                // 0x9B =>
                // 0x9C =>
                // 0x9D =>
                // 0x9E =>

                // RES 4,r
                // 0xA7 =>
                // 0xA0 =>
                // 0xA1 =>
                // 0xA2 =>
                // 0xA3 =>
                // 0xA4 =>
                // 0xA5 =>
                // 0xA6 =>

                // RES 5,r
                // 0xAF =>
                // 0xA8 =>
                // 0xA9 =>
                // 0xAA =>
                // 0xAB =>
                // 0xAC =>
                // 0xAD =>
                // 0xAE =>

                // RES 6,r
                // 0xB7 =>
                // 0xB0 =>
                // 0xB1 =>
                // 0xB2 =>
                // 0xB3 =>
                // 0xB4 =>
                // 0xB5 =>
                // 0xB6 =>

                // RES 7,r
                // 0xCF =>
                // 0xC8 =>
                // 0xC9 =>
                // 0xCA =>
                // 0xCB =>
                // 0xCC =>
                // 0xCD =>
                // 0xCE =>

                // Unimplemented instruction
                _ => panic!(
                    "Unimplemented extended opcode {:#04X} at {:#04X}",
                    ((opcode as u16) << 8) | (ext_opcode as u16),
                    cpu.pc
                ),
            }
        }

        // Unimplemented instruction
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

// BIT b,r: Iff bit b in register r == 0, set Z flag = 1. Else, set Z flag = 0.
fn bit_b_r(cpu: &mut Cpu, b: usize, target: Target) {
    bit_b_r_helper(cpu, b, cpu.regs.get_reg(target));
}
fn bit_b_r_hl(cpu: &mut Cpu, b: usize) {
    let target_byte = cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(VirtTarget::HL));
    bit_b_r_helper(cpu, b, target_byte)
}
fn bit_b_r_helper(cpu: &mut Cpu, b: usize, byte: u8) {
    let is_bit_zero = (byte & (0b1 << b)) == 0;
    cpu.regs.set_flag(RegFlag::Z, is_bit_zero);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, true);
    cpu.pc += 2;
}

// JP nn: Jump to address nn.
fn jp_nn(cpu: &mut Cpu) {
    let nn = cpu.get_next_2_bytes();
    jp_helper(cpu, nn);
}

// JP cc,nn: Iff C/Z flag == true/false, jump to address n.
fn jp_cc_nn(cpu: &mut Cpu, flag: RegFlag, expected_value: bool) {
    jp_cc_helper(cpu, flag, expected_value, false);
}

// JP (HL): Jump to address contained in (HL).
fn jp_hl(cpu: &mut Cpu) {
    jp_helper(cpu, cpu.regs.get_virt_reg(VirtTarget::HL));
}

// JR n: Add n to current address & jump to it.
fn jr_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    jp_helper(cpu, cpu.pc + (n as u16));
}

// JR cc,n: Iff C/Z flag == true/false, add n to current address & jump to it.
fn jr_cc_n(cpu: &mut Cpu, flag: RegFlag, expected_value: bool) {
    jp_cc_helper(cpu, flag, expected_value, true);
}

// Helper function for conditional jumps
fn jp_cc_helper(cpu: &mut Cpu, flag: RegFlag, expected_value: bool, is_jr: bool) {
    let test_val = match flag {
        RegFlag::Z | RegFlag::C => cpu.regs.get_flag(flag),
        _ => panic!("jr_cc_n: Cannot use flag {:?}. C or Z flags only.", flag),
    };
    match (is_jr, test_val == expected_value) {
        (true, true) => jr_n(cpu),
        (true, false) => cpu.pc += 2,
        (false, true) => jp_nn(cpu),
        (false, false) => cpu.pc += 3,
    }
}

// Helper function for jumps
fn jp_helper(cpu: &mut Cpu, address: u16) {
    cpu.pc = address;
}

// PUSH nn: Push virtual register nn to stack. Set sp = sp -= 2.

// POP nn: Pop 2 bytes off stack into virtual register nn. Set sp = sp += 2.

// RST n: Push current address to stack. Jump to address 0x0000 + n.

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

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
        assert_eq!(cpu.regs.get_virt_reg(VirtTarget::BC), 0x3412);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0006);
        assert_eq!(cpu.regs.get_virt_reg(VirtTarget::DE), 0x7856);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0009);
        assert_eq!(cpu.regs.get_virt_reg(VirtTarget::HL), 0xBC9A);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x000C);
        assert_eq!(cpu.sp, 0xF0DE);
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

    #[test]
    fn test_bit_b_r() {
        let mut cpu = Cpu::default();
        cpu.mem_bus.write_byte(0b0101_0101_0101_0101, 0b1010_1010);

        for target in Target::iter() {
            cpu.regs.reset_flags();
            cpu.regs.set_reg(target, 0b0101_0101);
            for bit_pos in 0..8 {
                // For testing purposes, the bits alternate between 1 & 0.
                // odd-indexed bits = 0, so Z-flag should be set to 1.
                let is_bit_pos_odd = (bit_pos % 2) == 1;
                // Alternate between setting C flag to 1 or 0 to test if it's unaffected.
                cpu.regs.set_flag(RegFlag::C, is_bit_pos_odd);

                bit_b_r(&mut cpu, bit_pos, target);
                assert_eq!(cpu.regs.get_flag(RegFlag::Z), is_bit_pos_odd);
                assert!(!cpu.regs.get_flag(RegFlag::N));
                assert!(cpu.regs.get_flag(RegFlag::H));
                assert_eq!(cpu.regs.get_flag(RegFlag::C), is_bit_pos_odd);
            }
        }

        // Test (HL) version.
        for bit_pos in 0..8 {
            cpu.regs.reset_flags();
            // even-indexed bits = 0, so Z-flag should be set to 1.
            let is_bit_pos_even = (bit_pos % 2) == 0;
            // Alternate between setting C flag to 1 or 0 to test if it's unaffected.
            cpu.regs.set_flag(RegFlag::C, !is_bit_pos_even);

            bit_b_r_hl(&mut cpu, bit_pos);
            assert_eq!(cpu.regs.get_flag(RegFlag::Z), is_bit_pos_even);
            assert!(!cpu.regs.get_flag(RegFlag::N));
            assert!(cpu.regs.get_flag(RegFlag::H));
            assert_eq!(cpu.regs.get_flag(RegFlag::C), !is_bit_pos_even);
        }
    }

    #[test]
    fn test_jumps() {
        let mut cpu = Cpu::new();
        // (Testing JP nn) Jump to address 0x1234.
        let data_1 = [0xC3, 0x34, 0x12];
        cpu.load(0x0000, &data_1);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x1234);

        // (Testing JP ~cc,nn) Don't do these jumps; conditions not met. Then, jump to address
        // 0x2000.
        let data_2 = [
            0xC2, 0xFF, 0xFF, 0xCA, 0xFF, 0xFF, 0xD2, 0xFF, 0xFF, 0xDA, 0xFF, 0xFF, 0xC3, 0x00,
            0x20,
        ];
        cpu.load(0x1234, &data_2);

        cpu.regs.reset_flags();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x1234 + 0x3);

        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x1234 + 0x6);

        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x1234 + 0x9);

        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x1234 + 0xC);

        cpu.regs.reset_flags();
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2000);

        // (Testing JP cc,nn) Do all these jumps; the conditions are met.
        let data_3 = [0xC2, 0x10, 0x20];
        cpu.load(0x2000, &data_3);
        let data_4 = [0xCA, 0x20, 0x20];
        cpu.load(0x2010, &data_4);
        let data_5 = [0xD2, 0x30, 0x20];
        cpu.load(0x2020, &data_5);
        let data_6 = [0xDA, 0x40, 0x20];
        cpu.load(0x2030, &data_6);

        cpu.regs.reset_flags();
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2010);

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2020);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x2030);

        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2040);

        // (Testing JP (HL)) Jump to address 0x2050.
        cpu.regs.set_virt_reg(VirtTarget::HL, 0x2050);
        cpu.mem_bus.write_byte(0x2040, 0xE9);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2050);

        // (Testing JR n) Add 0x28 to address and jump to 0x2078.
        let data_7 = [0x18, 0x28];
        cpu.load(0x2050, &data_7);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2078);

        // (Testing JR ~cc,n) Don't do these jumps; conditions not met. Then, jump to 0x3000.
        let data_8 = [
            0x20, 0xFF, 0x28, 0xFF, 0x30, 0xFF, 0x38, 0xFF, 0xC3, 0x00, 0x30,
        ];
        cpu.load(0x2078, &data_8);
        cpu.regs.reset_flags();

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2078 + 2);

        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2078 + 4);

        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2078 + 6);

        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x2078 + 8);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x3000);

        // (Testing JR cc,n) Conditions met; do all of these jumps.
        let data_9 = [0x20, 0x10];
        cpu.load(0x3000, &data_9);
        let data_10 = [0x28, 0x10];
        cpu.load(0x3010, &data_10);
        let data_11 = [0x30, 0x10];
        cpu.load(0x3020, &data_11);
        let data_12 = [0x38, 0x10];
        cpu.load(0x3030, &data_12);
        cpu.regs.reset_flags();

        cpu.cycle();
        assert_eq!(cpu.pc, 0x3010);

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x3020);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x3030);

        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.pc, 0x3040);
    }

    #[test]
    fn test_rst_n() {
        let mut cpu = Cpu::new();
    }
}
