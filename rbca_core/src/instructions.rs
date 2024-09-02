//! All functionality related to the CPU instructions.
use crate::{
    Cpu, RegFlag,
    Target::{self, A, B, C, D, E, H, L},
    VirtTarget::{self, AF, BC, DE, HL},
};

/// Execute a given opcode.
pub fn execute_opcode(cpu: &mut Cpu, opcode: u8) {
    match opcode {
        // LD nn,n
        0x06 => ld_nn_n(cpu, B),
        0x0E => ld_nn_n(cpu, C),
        0x16 => ld_nn_n(cpu, D),
        0x1E => ld_nn_n(cpu, E),
        0x26 => ld_nn_n(cpu, H),
        0x2E => ld_nn_n(cpu, L),

        // LD r1,r2
        0x7F => ld_r1_r2(cpu, A, A),
        0x78 => ld_r1_r2(cpu, A, B),
        0x79 => ld_r1_r2(cpu, A, C),
        0x7A => ld_r1_r2(cpu, A, D),
        0x7B => ld_r1_r2(cpu, A, E),
        0x7C => ld_r1_r2(cpu, A, H),
        0x7D => ld_r1_r2(cpu, A, L),
        0x7E => ld_r1_hl(cpu, A),
        0x40 => ld_r1_r2(cpu, B, B),
        0x41 => ld_r1_r2(cpu, B, C),
        0x42 => ld_r1_r2(cpu, B, D),
        0x43 => ld_r1_r2(cpu, B, E),
        0x44 => ld_r1_r2(cpu, B, H),
        0x45 => ld_r1_r2(cpu, B, L),
        0x46 => ld_r1_hl(cpu, B),
        0x48 => ld_r1_r2(cpu, C, B),
        0x49 => ld_r1_r2(cpu, C, C),
        0x4A => ld_r1_r2(cpu, C, D),
        0x4B => ld_r1_r2(cpu, C, E),
        0x4C => ld_r1_r2(cpu, C, H),
        0x4D => ld_r1_r2(cpu, C, L),
        0x4E => ld_r1_hl(cpu, C),
        0x50 => ld_r1_r2(cpu, D, B),
        0x51 => ld_r1_r2(cpu, D, C),
        0x52 => ld_r1_r2(cpu, D, D),
        0x53 => ld_r1_r2(cpu, D, E),
        0x54 => ld_r1_r2(cpu, D, H),
        0x55 => ld_r1_r2(cpu, D, L),
        0x56 => ld_r1_hl(cpu, D),
        0x58 => ld_r1_r2(cpu, E, B),
        0x59 => ld_r1_r2(cpu, E, C),
        0x5A => ld_r1_r2(cpu, E, D),
        0x5B => ld_r1_r2(cpu, E, E),
        0x5C => ld_r1_r2(cpu, E, H),
        0x5D => ld_r1_r2(cpu, E, L),
        0x5E => ld_r1_hl(cpu, E),
        0x60 => ld_r1_r2(cpu, H, B),
        0x61 => ld_r1_r2(cpu, H, C),
        0x62 => ld_r1_r2(cpu, H, D),
        0x63 => ld_r1_r2(cpu, H, E),
        0x64 => ld_r1_r2(cpu, H, H),
        0x65 => ld_r1_r2(cpu, H, L),
        0x66 => ld_r1_hl(cpu, H),
        0x68 => ld_r1_r2(cpu, L, B),
        0x69 => ld_r1_r2(cpu, L, C),
        0x6A => ld_r1_r2(cpu, L, D),
        0x6B => ld_r1_r2(cpu, L, E),
        0x6C => ld_r1_r2(cpu, L, H),
        0x6D => ld_r1_r2(cpu, L, L),
        0x6E => ld_r1_hl(cpu, L),
        0x70 => ld_hl_r2(cpu, B),
        0x71 => ld_hl_r2(cpu, C),
        0x72 => ld_hl_r2(cpu, D),
        0x73 => ld_hl_r2(cpu, E),
        0x74 => ld_hl_r2(cpu, H),
        0x75 => ld_hl_r2(cpu, L),
        0x36 => ld_hl_n(cpu),

        // LD A,n
        0x0A => ld_a_vr(cpu, BC),
        0x1A => ld_a_vr(cpu, DE),
        0xFA => ld_a_nn(cpu),
        0x3E => ld_a_n(cpu),

        // LD n,A
        0x47 => ld_r_a(cpu, B),
        0x4F => ld_r_a(cpu, C),
        0x57 => ld_r_a(cpu, D),
        0x5F => ld_r_a(cpu, E),
        0x67 => ld_r_a(cpu, H),
        0x6F => ld_r_a(cpu, L),
        0x02 => ld_vr_a(cpu, BC),
        0x12 => ld_vr_a(cpu, DE),
        0x77 => ld_vr_a(cpu, HL),
        0xEA => ld_nn_a(cpu),

        // LD A,(C)
        0xF2 => ld_a_c(cpu),

        // LD (C),A
        0xE2 => ld_c_a(cpu),

        // LD A,(HLD) / LD A,(HL-) / LDD A,(HL)
        0x3A => ld_a_hld(cpu),

        // LD (HLD),A / LD (HL-),A / LDD (HL,A)
        0x32 => ld_hld_a(cpu),

        // LD A,(HLI) / LD A,(HL+) / LDI A,(HL)
        0x2A => ld_a_hli(cpu),

        // LD (HLI),A / LD (HL+),A / LDI (HL,A)
        0x22 => ld_hli_a(cpu),

        // LDH (n),A
        0xE0 => ldh_n_a(cpu),

        // LDH A,(n)
        0xF0 => ldh_a_n(cpu),

        // LD n,nn
        0x01 => ld_n_nn(cpu, BC),
        0x11 => ld_n_nn(cpu, DE),
        0x21 => ld_n_nn(cpu, HL),
        0x31 => ld_n_nn_sp(cpu),

        // LD SP,HL
        0xF9 => ld_sp_hl(cpu),

        // LD HL,SP+n / LDHL SP,n
        0xF8 => ld_hl_sp_n(cpu),

        // LD (nn),SP
        0x08 => ld_nn_sp(cpu),

        // PUSH nn
        0xF5 => push_nn(cpu, AF),
        0xC5 => push_nn(cpu, BC),
        0xD5 => push_nn(cpu, DE),
        0xE5 => push_nn(cpu, HL),

        // POP nn
        0xF1 => pop_nn(cpu, AF),
        0xC1 => pop_nn(cpu, BC),
        0xD1 => pop_nn(cpu, DE),
        0xE1 => pop_nn(cpu, HL),

        // ADD A,n
        0x87 => add_a_n(cpu, A),
        0x80 => add_a_n(cpu, B),
        0x81 => add_a_n(cpu, C),
        0x82 => add_a_n(cpu, D),
        0x83 => add_a_n(cpu, E),
        0x84 => add_a_n(cpu, H),
        0x85 => add_a_n(cpu, L),
        0x86 => add_a_n_hl(cpu),
        0xC6 => add_a_n_n(cpu),

        // ADC A,n
        0x8F => adc_a_n(cpu, A),
        0x88 => adc_a_n(cpu, B),
        0x89 => adc_a_n(cpu, C),
        0x8A => adc_a_n(cpu, D),
        0x8B => adc_a_n(cpu, E),
        0x8C => adc_a_n(cpu, H),
        0x8D => adc_a_n(cpu, L),
        0x8E => adc_a_n_hl(cpu),
        0xCE => adc_a_n_n(cpu),

        // SUB n
        0x97 => sub_n(cpu, A),
        0x90 => sub_n(cpu, B),
        0x91 => sub_n(cpu, C),
        0x92 => sub_n(cpu, D),
        0x93 => sub_n(cpu, E),
        0x94 => sub_n(cpu, H),
        0x95 => sub_n(cpu, L),
        0x96 => sub_n_hl(cpu),
        0xD6 => sub_n_n(cpu),

        // SBC A,n
        0x9F => sbc_n(cpu, A),
        0x98 => sbc_n(cpu, B),
        0x99 => sbc_n(cpu, C),
        0x9A => sbc_n(cpu, D),
        0x9B => sbc_n(cpu, E),
        0x9C => sbc_n(cpu, H),
        0x9D => sbc_n(cpu, L),
        0x9E => sbc_n_hl(cpu),
        0xDE => sbc_n_n(cpu),

        // AND n
        0xA7 => and_n(cpu, A),
        0xA0 => and_n(cpu, B),
        0xA1 => and_n(cpu, C),
        0xA2 => and_n(cpu, D),
        0xA3 => and_n(cpu, E),
        0xA4 => and_n(cpu, H),
        0xA5 => and_n(cpu, L),
        0xA6 => and_n_hl(cpu),
        0xE6 => and_n_n(cpu),

        // OR n
        0xB7 => or_n(cpu, A),
        0xB0 => or_n(cpu, B),
        0xB1 => or_n(cpu, C),
        0xB2 => or_n(cpu, D),
        0xB3 => or_n(cpu, E),
        0xB4 => or_n(cpu, H),
        0xB5 => or_n(cpu, L),
        0xB6 => or_n_hl(cpu),
        0xF6 => or_n_n(cpu),

        // XOR n
        0xAF => xor_n(cpu, A),
        0xA8 => xor_n(cpu, B),
        0xA9 => xor_n(cpu, C),
        0xAA => xor_n(cpu, D),
        0xAB => xor_n(cpu, E),
        0xAC => xor_n(cpu, H),
        0xAD => xor_n(cpu, L),
        0xAE => xor_n_hl(cpu),
        0xEE => xor_n_n(cpu),

        // CP n
        0xBF => cp_n(cpu, A),
        0xB8 => cp_n(cpu, B),
        0xB9 => cp_n(cpu, C),
        0xBA => cp_n(cpu, D),
        0xBB => cp_n(cpu, E),
        0xBC => cp_n(cpu, H),
        0xBD => cp_n(cpu, L),
        0xBE => cp_n_hl(cpu),
        0xFE => cp_n_n(cpu),

        // INC n
        0x3C => inc_n(cpu, A),
        0x04 => inc_n(cpu, B),
        0x0C => inc_n(cpu, C),
        0x14 => inc_n(cpu, D),
        0x1C => inc_n(cpu, E),
        0x24 => inc_n(cpu, H),
        0x2C => inc_n(cpu, L),
        0x34 => inc_n_hl(cpu),

        // DEC n
        0x3D => dec_n(cpu, A),
        0x05 => dec_n(cpu, B),
        0x0D => dec_n(cpu, C),
        0x15 => dec_n(cpu, D),
        0x1D => dec_n(cpu, E),
        0x25 => dec_n(cpu, H),
        0x2D => dec_n(cpu, L),
        0x35 => dec_n_hl(cpu),

        // ADD HL,n
        0x09 => add_hl_n(cpu, BC),
        0x19 => add_hl_n(cpu, DE),
        0x29 => add_hl_n(cpu, HL),
        0x39 => add_hl_n_sp(cpu),

        // ADD SP,n
        0xE8 => add_sp_n(cpu),

        // INC nn
        0x03 => inc_nn(cpu, BC),
        0x13 => inc_nn(cpu, DE),
        0x23 => inc_nn(cpu, HL),
        0x33 => inc_nn_sp(cpu),

        // DEC nn
        0x0B => dec_nn(cpu, BC),
        0x1B => dec_nn(cpu, DE),
        0x2B => dec_nn(cpu, HL),
        0x3B => dec_nn_sp(cpu),

        // DAA
        0x27 => daa(cpu),

        // CPL
        0x2F => cpl(cpu),

        // CCF
        0x3F => ccf(cpu),

        // SCF
        0x37 => scf(cpu),

        // NOP
        0x00 => nop(cpu),

        // HALT
        0x76 => halt(cpu),

        // STOP
        0x10 => match cpu.get_next_byte() {
            0x00 => stop(cpu),
            other => panic!("Illegal operation: {:#04X}", 0x1000 | (other as u16)),
        },

        // DI
        0xF3 => di(cpu),

        // EI
        0xFB => ei(cpu),

        // RLCA
        0x07 => rlca(cpu),

        // RLA
        0x17 => rla(cpu),

        // RRCA
        0x0F => rrca(cpu),

        // RRA
        0x1F => rra(cpu),

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
        0xC7 => rst_n(cpu, 0x0000),
        0xCF => rst_n(cpu, 0x0008),
        0xD7 => rst_n(cpu, 0x0010),
        0xDF => rst_n(cpu, 0x0018),
        0xE7 => rst_n(cpu, 0x0020),
        0xEF => rst_n(cpu, 0x0028),
        0xF7 => rst_n(cpu, 0x0030),
        0xFF => rst_n(cpu, 0x0038),

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
                0x37 => swap_n(cpu, A),
                0x30 => swap_n(cpu, B),
                0x31 => swap_n(cpu, C),
                0x32 => swap_n(cpu, D),
                0x33 => swap_n(cpu, E),
                0x34 => swap_n(cpu, H),
                0x35 => swap_n(cpu, L),
                0x36 => swap_n_hl(cpu),

                // RLC n
                0x07 => rlc_n(cpu, A),
                0x00 => rlc_n(cpu, B),
                0x01 => rlc_n(cpu, C),
                0x02 => rlc_n(cpu, D),
                0x03 => rlc_n(cpu, E),
                0x04 => rlc_n(cpu, H),
                0x05 => rlc_n(cpu, L),
                0x06 => rlc_n_hl(cpu),

                // RL n
                0x17 => rl_n(cpu, A),
                0x10 => rl_n(cpu, B),
                0x11 => rl_n(cpu, C),
                0x12 => rl_n(cpu, D),
                0x13 => rl_n(cpu, E),
                0x14 => rl_n(cpu, H),
                0x15 => rl_n(cpu, L),
                0x16 => rl_n_hl(cpu),

                // RRC n
                0x0F => rrc_n(cpu, A),
                0x08 => rrc_n(cpu, B),
                0x09 => rrc_n(cpu, C),
                0x0A => rrc_n(cpu, D),
                0x0B => rrc_n(cpu, E),
                0x0C => rrc_n(cpu, H),
                0x0D => rrc_n(cpu, L),
                0x0E => rrc_n_hl(cpu),

                // RR n
                0x1F => rr_n(cpu, A),
                0x18 => rr_n(cpu, B),
                0x19 => rr_n(cpu, C),
                0x1A => rr_n(cpu, D),
                0x1B => rr_n(cpu, E),
                0x1C => rr_n(cpu, H),
                0x1D => rr_n(cpu, L),
                0x1E => rr_n_hl(cpu),

                // SLA n
                0x27 => sla_n(cpu, A),
                0x20 => sla_n(cpu, B),
                0x21 => sla_n(cpu, C),
                0x22 => sla_n(cpu, D),
                0x23 => sla_n(cpu, E),
                0x24 => sla_n(cpu, H),
                0x25 => sla_n(cpu, L),
                0x26 => sla_n_hl(cpu),

                // SRA n
                0x2F => sra_n(cpu, A),
                0x28 => sra_n(cpu, B),
                0x29 => sra_n(cpu, C),
                0x2A => sra_n(cpu, D),
                0x2B => sra_n(cpu, E),
                0x2C => sra_n(cpu, H),
                0x2D => sra_n(cpu, L),
                0x2E => sra_n_hl(cpu),

                // SRL n
                0x3F => srl_n(cpu, A),
                0x38 => srl_n(cpu, B),
                0x39 => srl_n(cpu, C),
                0x3A => srl_n(cpu, D),
                0x3B => srl_n(cpu, E),
                0x3C => srl_n(cpu, H),
                0x3D => srl_n(cpu, L),
                0x3E => srl_n_hl(cpu),

                // BIT 0,r
                0x47 => bit_b_r(cpu, 0, A),
                0x40 => bit_b_r(cpu, 0, B),
                0x41 => bit_b_r(cpu, 0, C),
                0x42 => bit_b_r(cpu, 0, D),
                0x43 => bit_b_r(cpu, 0, E),
                0x44 => bit_b_r(cpu, 0, H),
                0x45 => bit_b_r(cpu, 0, L),
                0x46 => bit_b_r_hl(cpu, 0),

                // BIT 1,r
                0x4F => bit_b_r(cpu, 1, A),
                0x48 => bit_b_r(cpu, 1, B),
                0x49 => bit_b_r(cpu, 1, C),
                0x4A => bit_b_r(cpu, 1, D),
                0x4B => bit_b_r(cpu, 1, E),
                0x4C => bit_b_r(cpu, 1, H),
                0x4D => bit_b_r(cpu, 1, L),
                0x4E => bit_b_r_hl(cpu, 1),

                // BIT 2,r
                0x57 => bit_b_r(cpu, 2, A),
                0x50 => bit_b_r(cpu, 2, B),
                0x51 => bit_b_r(cpu, 2, C),
                0x52 => bit_b_r(cpu, 2, D),
                0x53 => bit_b_r(cpu, 2, E),
                0x54 => bit_b_r(cpu, 2, H),
                0x55 => bit_b_r(cpu, 2, L),
                0x56 => bit_b_r_hl(cpu, 2),

                // BIT 3,r
                0x5F => bit_b_r(cpu, 3, A),
                0x58 => bit_b_r(cpu, 3, B),
                0x59 => bit_b_r(cpu, 3, C),
                0x5A => bit_b_r(cpu, 3, D),
                0x5B => bit_b_r(cpu, 3, E),
                0x5C => bit_b_r(cpu, 3, H),
                0x5D => bit_b_r(cpu, 3, L),
                0x5E => bit_b_r_hl(cpu, 3),

                // BIT 4,r
                0x67 => bit_b_r(cpu, 4, A),
                0x60 => bit_b_r(cpu, 4, B),
                0x61 => bit_b_r(cpu, 4, C),
                0x62 => bit_b_r(cpu, 4, D),
                0x63 => bit_b_r(cpu, 4, E),
                0x64 => bit_b_r(cpu, 4, H),
                0x65 => bit_b_r(cpu, 4, L),
                0x66 => bit_b_r_hl(cpu, 4),

                // BIT 5,r
                0x6F => bit_b_r(cpu, 5, A),
                0x68 => bit_b_r(cpu, 5, B),
                0x69 => bit_b_r(cpu, 5, C),
                0x6A => bit_b_r(cpu, 5, D),
                0x6B => bit_b_r(cpu, 5, E),
                0x6C => bit_b_r(cpu, 5, H),
                0x6D => bit_b_r(cpu, 5, L),
                0x6E => bit_b_r_hl(cpu, 5),

                // BIT 6,r
                0x77 => bit_b_r(cpu, 6, A),
                0x70 => bit_b_r(cpu, 6, B),
                0x71 => bit_b_r(cpu, 6, C),
                0x72 => bit_b_r(cpu, 6, D),
                0x73 => bit_b_r(cpu, 6, E),
                0x74 => bit_b_r(cpu, 6, H),
                0x75 => bit_b_r(cpu, 6, L),
                0x76 => bit_b_r_hl(cpu, 6),

                // BIT 7,r
                0x7F => bit_b_r(cpu, 7, A),
                0x78 => bit_b_r(cpu, 7, B),
                0x79 => bit_b_r(cpu, 7, C),
                0x7A => bit_b_r(cpu, 7, D),
                0x7B => bit_b_r(cpu, 7, E),
                0x7C => bit_b_r(cpu, 7, H),
                0x7D => bit_b_r(cpu, 7, L),
                0x7E => bit_b_r_hl(cpu, 7),

                // SET 0,r
                0xC7 => set_b_r(cpu, 0, A),
                0xC0 => set_b_r(cpu, 0, B),
                0xC1 => set_b_r(cpu, 0, C),
                0xC2 => set_b_r(cpu, 0, D),
                0xC3 => set_b_r(cpu, 0, E),
                0xC4 => set_b_r(cpu, 0, H),
                0xC5 => set_b_r(cpu, 0, L),
                0xC6 => set_b_r_hl(cpu, 0),

                // SET 1,r
                0xCF => set_b_r(cpu, 1, A),
                0xC8 => set_b_r(cpu, 1, B),
                0xC9 => set_b_r(cpu, 1, C),
                0xCA => set_b_r(cpu, 1, D),
                0xCB => set_b_r(cpu, 1, E),
                0xCC => set_b_r(cpu, 1, H),
                0xCD => set_b_r(cpu, 1, L),
                0xCE => set_b_r_hl(cpu, 1),

                // SET 2,r
                0xD7 => set_b_r(cpu, 2, A),
                0xD0 => set_b_r(cpu, 2, B),
                0xD1 => set_b_r(cpu, 2, C),
                0xD2 => set_b_r(cpu, 2, D),
                0xD3 => set_b_r(cpu, 2, E),
                0xD4 => set_b_r(cpu, 2, H),
                0xD5 => set_b_r(cpu, 2, L),
                0xD6 => set_b_r_hl(cpu, 2),

                // SET 3,r
                0xDF => set_b_r(cpu, 3, A),
                0xD8 => set_b_r(cpu, 3, B),
                0xD9 => set_b_r(cpu, 3, C),
                0xDA => set_b_r(cpu, 3, D),
                0xDB => set_b_r(cpu, 3, E),
                0xDC => set_b_r(cpu, 3, H),
                0xDD => set_b_r(cpu, 3, L),
                0xDE => set_b_r_hl(cpu, 3),

                // SET 4,r
                0xE7 => set_b_r(cpu, 4, A),
                0xE0 => set_b_r(cpu, 4, B),
                0xE1 => set_b_r(cpu, 4, C),
                0xE2 => set_b_r(cpu, 4, D),
                0xE3 => set_b_r(cpu, 4, E),
                0xE4 => set_b_r(cpu, 4, H),
                0xE5 => set_b_r(cpu, 4, L),
                0xE6 => set_b_r_hl(cpu, 4),

                // SET 5,r
                0xEF => set_b_r(cpu, 5, A),
                0xE8 => set_b_r(cpu, 5, B),
                0xE9 => set_b_r(cpu, 5, C),
                0xEA => set_b_r(cpu, 5, D),
                0xEB => set_b_r(cpu, 5, E),
                0xEC => set_b_r(cpu, 5, H),
                0xED => set_b_r(cpu, 5, L),
                0xEE => set_b_r_hl(cpu, 5),

                // SET 6,r
                0xF7 => set_b_r(cpu, 6, A),
                0xF0 => set_b_r(cpu, 6, B),
                0xF1 => set_b_r(cpu, 6, C),
                0xF2 => set_b_r(cpu, 6, D),
                0xF3 => set_b_r(cpu, 6, E),
                0xF4 => set_b_r(cpu, 6, H),
                0xF5 => set_b_r(cpu, 6, L),
                0xF6 => set_b_r_hl(cpu, 6),

                // SET 7,r
                0xFF => set_b_r(cpu, 7, A),
                0xF8 => set_b_r(cpu, 7, B),
                0xF9 => set_b_r(cpu, 7, C),
                0xFA => set_b_r(cpu, 7, D),
                0xFB => set_b_r(cpu, 7, E),
                0xFC => set_b_r(cpu, 7, H),
                0xFD => set_b_r(cpu, 7, L),
                0xFE => set_b_r_hl(cpu, 7),

                // RES 0,r
                0x87 => res_b_r(cpu, 0, A),
                0x80 => res_b_r(cpu, 0, B),
                0x81 => res_b_r(cpu, 0, C),
                0x82 => res_b_r(cpu, 0, D),
                0x83 => res_b_r(cpu, 0, E),
                0x84 => res_b_r(cpu, 0, H),
                0x85 => res_b_r(cpu, 0, L),
                0x86 => res_b_r_hl(cpu, 0),

                // RES 1,r
                0x8F => res_b_r(cpu, 1, A),
                0x88 => res_b_r(cpu, 1, B),
                0x89 => res_b_r(cpu, 1, C),
                0x8A => res_b_r(cpu, 1, D),
                0x8B => res_b_r(cpu, 1, E),
                0x8C => res_b_r(cpu, 1, H),
                0x8D => res_b_r(cpu, 1, L),
                0x8E => res_b_r_hl(cpu, 1),

                // RES 2,r
                0x97 => res_b_r(cpu, 2, A),
                0x90 => res_b_r(cpu, 2, B),
                0x91 => res_b_r(cpu, 2, C),
                0x92 => res_b_r(cpu, 2, D),
                0x93 => res_b_r(cpu, 2, E),
                0x94 => res_b_r(cpu, 2, H),
                0x95 => res_b_r(cpu, 2, L),
                0x96 => res_b_r_hl(cpu, 2),

                // RES 3,r
                0x9F => res_b_r(cpu, 3, A),
                0x98 => res_b_r(cpu, 3, B),
                0x99 => res_b_r(cpu, 3, C),
                0x9A => res_b_r(cpu, 3, D),
                0x9B => res_b_r(cpu, 3, E),
                0x9C => res_b_r(cpu, 3, H),
                0x9D => res_b_r(cpu, 3, L),
                0x9E => res_b_r_hl(cpu, 3),

                // RES 4,r
                0xA7 => res_b_r(cpu, 4, A),
                0xA0 => res_b_r(cpu, 4, B),
                0xA1 => res_b_r(cpu, 4, C),
                0xA2 => res_b_r(cpu, 4, D),
                0xA3 => res_b_r(cpu, 4, E),
                0xA4 => res_b_r(cpu, 4, H),
                0xA5 => res_b_r(cpu, 4, L),
                0xA6 => res_b_r_hl(cpu, 4),

                // RES 5,r
                0xAF => res_b_r(cpu, 5, A),
                0xA8 => res_b_r(cpu, 5, B),
                0xA9 => res_b_r(cpu, 5, C),
                0xAA => res_b_r(cpu, 5, D),
                0xAB => res_b_r(cpu, 5, E),
                0xAC => res_b_r(cpu, 5, H),
                0xAD => res_b_r(cpu, 5, L),
                0xAE => res_b_r_hl(cpu, 5),

                // RES 6,r
                0xB7 => res_b_r(cpu, 6, A),
                0xB0 => res_b_r(cpu, 6, B),
                0xB1 => res_b_r(cpu, 6, C),
                0xB2 => res_b_r(cpu, 6, D),
                0xB3 => res_b_r(cpu, 6, E),
                0xB4 => res_b_r(cpu, 6, H),
                0xB5 => res_b_r(cpu, 6, L),
                0xB6 => res_b_r_hl(cpu, 6),

                // RES 7,r
                0xBF => res_b_r(cpu, 7, A),
                0xB8 => res_b_r(cpu, 7, B),
                0xB9 => res_b_r(cpu, 7, C),
                0xBA => res_b_r(cpu, 7, D),
                0xBB => res_b_r(cpu, 7, E),
                0xBC => res_b_r(cpu, 7, H),
                0xBD => res_b_r(cpu, 7, L),
                0xBE => res_b_r_hl(cpu, 7),

                // Unimplemented instruction
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "Unimplemented extended opcode {:#04X} at {:#04X}",
                    ((opcode as u16) << 8) | (ext_opcode as u16),
                    cpu.pc
                ),
            }
        }

        // TODO handle illegal opcodes

        // Unimplemented instruction
        _ => panic!("Unimplemented opcode {:#02X} at {:#04X}", opcode, cpu.pc,),
    }
}

// ----------------------------------------------------
// FUNCTIONS
// ----------------------------------------------------

// LD nn,n: Set 8-bit immediate value n = nn.
fn ld_nn_n(cpu: &mut Cpu, target: Target) {
    cpu.mem_bus.memory[(cpu.pc as usize) + 1] = cpu.regs.get_reg(target);
    cpu.pc += 2;
}

// LD r1,r2: Set r1 = r2.
fn ld_r1_r2(cpu: &mut Cpu, r1: Target, r2: Target) {
    cpu.regs.set_reg(r1, cpu.regs.get_reg(r2));
    cpu.pc += 1;
}
fn ld_r1_hl(cpu: &mut Cpu, r1: Target) {
    let address = cpu.regs.get_virt_reg(HL);
    let value = cpu.mem_bus.read_byte(address);
    cpu.regs.set_reg(r1, value);
    cpu.pc += 1;
}
fn ld_hl_r2(cpu: &mut Cpu, r2: Target) {
    let address = cpu.regs.get_virt_reg(HL);
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(r2));
    cpu.pc += 1;
}
fn ld_hl_n(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let value = cpu.get_next_byte();
    cpu.mem_bus.write_byte(address, value);
    // TODO not sure about length
    cpu.pc += 2;
}

// LD A,n: Set A = n.
fn ld_a_vr(cpu: &mut Cpu, target: VirtTarget) {
    let address = cpu.regs.get_virt_reg(target);
    let value = cpu.mem_bus.read_byte(address);
    ld_a_n_helper(cpu, value);
    cpu.pc += 1;
}
fn ld_a_nn(cpu: &mut Cpu) {
    let address = cpu.get_next_2_bytes();
    let value = cpu.mem_bus.read_byte(address);
    ld_a_n_helper(cpu, value);
    cpu.pc += 3;
}
fn ld_a_n(cpu: &mut Cpu) {
    let value = cpu.get_next_byte();
    ld_a_n_helper(cpu, value);
    cpu.pc += 2;
}
fn ld_a_n_helper(cpu: &mut Cpu, value: u8) {
    cpu.regs.set_reg(A, value);
}

// LD n,A: Set n = A.
fn ld_r_a(cpu: &mut Cpu, target: Target) {
    cpu.regs.set_reg(target, cpu.regs.get_reg(A));
    cpu.pc += 1;
}
fn ld_vr_a(cpu: &mut Cpu, target: VirtTarget) {
    let address = cpu.regs.get_virt_reg(target);
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(A));
    cpu.pc += 1;
}
fn ld_nn_a(cpu: &mut Cpu) {
    let address = cpu.get_next_2_bytes();
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(A));
    cpu.pc += 3;
}

// LD A,(C): Set A = (0xFF00 + C).
fn ld_a_c(cpu: &mut Cpu) {
    let address = 0xFF00 | (cpu.regs.get_reg(C) as u16);
    cpu.regs.set_reg(A, cpu.mem_bus.read_byte(address));
    cpu.pc += 1;
}

// LD (C),A: Set (0xFF00 + C) = A.
fn ld_c_a(cpu: &mut Cpu) {
    let address = 0xFF00 | (cpu.regs.get_reg(C) as u16);
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(A));
    cpu.pc += 1;
}

// LD A,(HLD): Set A = (HL). HL -= 1.
fn ld_a_hld(cpu: &mut Cpu) {
    ld_a_hl_helper(cpu, false);
}

// LD (HLD),A: Set (HL) = A. HL -= 1.
fn ld_hld_a(cpu: &mut Cpu) {
    ld_hl_a_helper(cpu, false);
}

// LD A,(HLI): Set A = (HL). HL += 1.
fn ld_a_hli(cpu: &mut Cpu) {
    ld_a_hl_helper(cpu, true);
}

// LD (HLI),A: Set (HL) = A. HL += 1.
fn ld_hli_a(cpu: &mut Cpu) {
    ld_hl_a_helper(cpu, true)
}

fn ld_a_hl_helper(cpu: &mut Cpu, is_inc: bool) {
    let address = cpu.regs.get_virt_reg(HL);
    cpu.regs.set_reg(A, cpu.mem_bus.read_byte(address));

    let new_val = if is_inc { address + 1 } else { address - 1 };
    cpu.regs.set_virt_reg(HL, new_val);

    cpu.pc += 1;
}
fn ld_hl_a_helper(cpu: &mut Cpu, is_inc: bool) {
    let address = cpu.regs.get_virt_reg(HL);
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(A));

    let new_val = if is_inc { address + 1 } else { address - 1 };
    cpu.regs.set_virt_reg(HL, new_val);

    cpu.pc += 1;
}

// LDH (n),A: Set (0xFF00 + n) = A.
fn ldh_n_a(cpu: &mut Cpu) {
    let address = 0xFF00 | (cpu.get_next_byte() as u16);
    cpu.mem_bus.write_byte(address, cpu.regs.get_reg(A));
    cpu.pc += 2;
}

// LDH A,(n): Set A = (0xFF00 + n).
fn ldh_a_n(cpu: &mut Cpu) {
    let address = 0xFF00 | (cpu.get_next_byte() as u16);
    cpu.regs.set_reg(A, cpu.mem_bus.read_byte(address));
    cpu.pc += 2;
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

// LD SP,HL: Set SP = HL.
fn ld_sp_hl(cpu: &mut Cpu) {
    cpu.sp = cpu.regs.get_virt_reg(HL);
    cpu.pc += 1;
}

// LD HL,SP+n: Set HL = SP + n.
// Iff n is positive, set H iff carry on lowest nibble.
// Iff n is positive, set C iff carry on lowest byte.
// Iff n is negative, set H iff lowest nibble is decreased.
// Iff n is negative, set C iff lowest byte is decreased.
fn ld_hl_sp_n(cpu: &mut Cpu) {
    let result = sp_n_helper(cpu);
    cpu.regs.set_virt_reg(HL, result);
    cpu.pc += 2;
}
fn sp_n_helper(cpu: &mut Cpu) -> u16 {
    let n_i = cpu.get_next_byte() as i8;
    let n_u = cpu.get_next_byte() as i8 as i16 as u16;

    cpu.regs.reset_flags();
    let h_val;
    let c_val;
    let result;
    if (n_i) >= 0 {
        result = cpu.sp.wrapping_add(n_u);
        h_val = ((0x000F & cpu.sp) + (0x000F & n_u)) > 0x000F;
        c_val = ((0x00FF & cpu.sp) + (n_u)) > 0x00FF;
    } else {
        result = cpu.sp.wrapping_add_signed(n_i as i16);
        h_val = (0x000F & result) <= (0x000F & cpu.sp);
        c_val = (0x00FF & result) <= (0x00FF & cpu.sp);
    }
    cpu.regs.set_flag(RegFlag::H, h_val);
    cpu.regs.set_flag(RegFlag::C, c_val);
    result
}

// LD (nn),SP: Set (nn) = SP.
fn ld_nn_sp(cpu: &mut Cpu) {
    let address = cpu.get_next_2_bytes();
    cpu.mem_bus.write_2_bytes(address, cpu.sp);
    cpu.pc += 3;
}

// PUSH nn: Push virtual register nn to stack. Set sp = sp -= 2.
fn push_nn(cpu: &mut Cpu, target: VirtTarget) {
    cpu.push_stack(cpu.regs.get_virt_reg(target));
    cpu.pc += 1;
}

// POP nn: Pop 2 bytes off stack into virtual register nn. Set sp = sp += 2.
fn pop_nn(cpu: &mut Cpu, target: VirtTarget) {
    let popped_val = cpu.pop_stack();
    cpu.regs.set_virt_reg(target, popped_val);
    cpu.pc += 1;
}

// ADD A,n: A += n.
fn add_a_n(cpu: &mut Cpu, target: Target) {
    add_a_n_helper(cpu, cpu.regs.get_reg(target), false);
    cpu.pc += 1;
}
fn add_a_n_hl(cpu: &mut Cpu) {
    let n = cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL));
    add_a_n_helper(cpu, n, false);
    cpu.pc += 1;
}
fn add_a_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    add_a_n_helper(cpu, n, false);
    cpu.pc += 2;
}
fn add_a_n_helper(cpu: &mut Cpu, n: u8, use_carry: bool) {
    let a = cpu.regs.get_reg(A);
    let carry = if use_carry && cpu.regs.get_flag(RegFlag::C) {
        1
    } else {
        0
    };
    let sum = a.wrapping_add(n).wrapping_add(carry);

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, sum == 0);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs
        .set_flag(RegFlag::H, ((0x0F & a) + (0x0F & n) + carry) > 0x0F);
    cpu.regs.set_flag(
        RegFlag::C,
        ((a as u16) + (n as u16) + (carry as u16)) > 0xFF,
    );

    cpu.regs.set_reg(A, sum);
}

// ADC A,n: A += (n + carry flags).
fn adc_a_n(cpu: &mut Cpu, target: Target) {
    add_a_n_helper(cpu, cpu.regs.get_reg(target), true);
    cpu.pc += 1;
}
fn adc_a_n_hl(cpu: &mut Cpu) {
    let n = cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL));
    add_a_n_helper(cpu, n, true);
    cpu.pc += 1;
}
fn adc_a_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    add_a_n_helper(cpu, n, true);
    cpu.pc += 2;
}

// SUB n: A -= n.
fn sub_n(cpu: &mut Cpu, target: Target) {
    sub_n_helper(cpu, cpu.regs.get_reg(target), false);
    cpu.pc += 1;
}
fn sub_n_hl(cpu: &mut Cpu) {
    let n = cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL));
    sub_n_helper(cpu, n, false);
    cpu.pc += 1;
}
fn sub_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    sub_n_helper(cpu, n, false);
    cpu.pc += 2;
}
fn sub_n_helper(cpu: &mut Cpu, n: u8, use_borrow: bool) {
    let a = cpu.regs.get_reg(A);
    let borrow = if use_borrow && cpu.regs.get_flag(RegFlag::C) {
        1
    } else {
        0
    };
    let difference = a.wrapping_sub(n).wrapping_sub(borrow);

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, difference == 0);
    cpu.regs.set_flag(RegFlag::N, true);
    cpu.regs
        .set_flag(RegFlag::H, (0x0F & a) < ((0x0F & n) + borrow));
    cpu.regs
        .set_flag(RegFlag::C, (a as u16) < ((n as u16) + (borrow as u16)));

    cpu.regs.set_reg(A, difference);
}

// SBC A,n: Set A -= (n + carry flag).
fn sbc_n(cpu: &mut Cpu, target: Target) {
    sub_n_helper(cpu, cpu.regs.get_reg(target), true);
    cpu.pc += 1;
}
fn sbc_n_hl(cpu: &mut Cpu) {
    let n = cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL));
    sub_n_helper(cpu, n, true);
    cpu.pc += 1;
}
fn sbc_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    sub_n_helper(cpu, n, true);
    cpu.pc += 2;
}

// AND n: Set A = A AND n.
fn and_n(cpu: &mut Cpu, target: Target) {
    and_n_helper(cpu, cpu.regs.get_reg(target));
    cpu.pc += 1;
}
fn and_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mem_bus.read_byte(address);
    and_n_helper(cpu, n);
    cpu.pc += 1;
}
fn and_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    and_n_helper(cpu, n);
    cpu.pc += 2;
}
fn and_n_helper(cpu: &mut Cpu, n: u8) {
    let result = cpu.regs.get_reg(A) & n;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::H, true);

    cpu.regs.set_reg(A, result);
}

// OR n: Set A = A OR n.
fn or_n(cpu: &mut Cpu, target: Target) {
    or_n_helper(cpu, cpu.regs.get_reg(target));
    cpu.pc += 1;
}
fn or_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mem_bus.read_byte(address);
    or_n_helper(cpu, n);
    cpu.pc += 1;
}
fn or_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    or_n_helper(cpu, n);
    cpu.pc += 2;
}
fn or_n_helper(cpu: &mut Cpu, n: u8) {
    let result = cpu.regs.get_reg(A) | n;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);

    cpu.regs.set_reg(A, result);
}

// XOR n: Set A = A XOR n.
fn xor_n(cpu: &mut Cpu, target: Target) {
    xor_n_helper(cpu, cpu.regs.get_reg(target));
    cpu.pc += 1;
}
fn xor_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mem_bus.read_byte(address);
    xor_n_helper(cpu, n);
    cpu.pc += 1;
}
fn xor_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    xor_n_helper(cpu, n);
    cpu.pc += 2;
}
fn xor_n_helper(cpu: &mut Cpu, n: u8) {
    let result = cpu.regs.get_reg(A) ^ n;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);

    cpu.regs.set_reg(A, result);
}

// CP n: Compare A with n.
fn cp_n(cpu: &mut Cpu, target: Target) {
    cp_n_helper(cpu, cpu.regs.get_reg(target));
    cpu.pc += 1;
}
fn cp_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mem_bus.read_byte(address);
    cp_n_helper(cpu, n);
    cpu.pc += 1;
}
fn cp_n_n(cpu: &mut Cpu) {
    let n = cpu.get_next_byte();
    cp_n_helper(cpu, n);
    cpu.pc += 2;
}
fn cp_n_helper(cpu: &mut Cpu, n: u8) {
    let a_val = cpu.regs.get_reg(A);
    sub_n_helper(cpu, n, false);
    cpu.regs.set_reg(A, a_val);
}

// INC n: n += 1.
fn inc_n(cpu: &mut Cpu, target: Target) {
    let reg_val = cpu.regs.get_reg(target);
    let result = reg_val.wrapping_add(1);

    inc_n_set_flags(cpu, reg_val, result);

    cpu.regs.set_reg(target, result);
    cpu.pc += 1;
}
fn inc_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let val = cpu.mem_bus.read_byte(address);
    let result = val.wrapping_add(1);

    inc_n_set_flags(cpu, val, result);

    cpu.mem_bus.write_byte(address, result);
    cpu.pc += 1;
}
fn inc_n_set_flags(cpu: &mut Cpu, val: u8, result: u8) {
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, ((0x0F & val) + 1) > 0x0F);
}

// DEC n: n -= 1.
fn dec_n(cpu: &mut Cpu, target: Target) {
    let reg_val = cpu.regs.get_reg(target);
    let result = reg_val.wrapping_sub(1);

    dec_n_set_flags(cpu, reg_val, result);

    cpu.regs.set_reg(target, result);
    cpu.pc += 1;
}
fn dec_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let val = cpu.mem_bus.read_byte(address);
    let result = val.wrapping_sub(1);

    inc_n_set_flags(cpu, val, result);

    cpu.mem_bus.write_byte(address, result);
    cpu.pc += 1;
}
fn dec_n_set_flags(cpu: &mut Cpu, val: u8, result: u8) {
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::N, true);
    cpu.regs.set_flag(RegFlag::H, (0x0F & val) == 0);
}

// ADD HL,n: HL += n.
fn add_hl_n(cpu: &mut Cpu, target: VirtTarget) {
    add_hl_n_helper(cpu, cpu.regs.get_virt_reg(target));
    cpu.pc += 1;
}
fn add_hl_n_sp(cpu: &mut Cpu) {
    add_hl_n_helper(cpu, cpu.sp);
    cpu.pc += 1;
}
fn add_hl_n_helper(cpu: &mut Cpu, n: u16) {
    let hl_val = cpu.regs.get_virt_reg(HL);
    let result = hl_val.wrapping_add(n);

    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs
        .set_flag(RegFlag::H, ((0x0FFF & hl_val) + (0x0FFF & n)) > 0x0FFF);
    cpu.regs
        .set_flag(RegFlag::C, ((hl_val as u32) + (n as u32)) > 0x0000_FFFF);

    cpu.regs.set_virt_reg(HL, result);
}

// ADD SP,n: SP += n. (n = one byte signed immediate value)
fn add_sp_n(cpu: &mut Cpu) {
    let result = sp_n_helper(cpu);
    cpu.sp = result;
    cpu.pc += 2;
}

// INC nn: nn += 1.
fn inc_nn(cpu: &mut Cpu, target: VirtTarget) {
    let val = cpu.regs.get_virt_reg(target);
    cpu.regs.set_virt_reg(target, val.wrapping_add(1));
    cpu.pc += 1;
}
fn inc_nn_sp(cpu: &mut Cpu) {
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.pc += 1;
}

// DEC nn: nn -= 1.
fn dec_nn(cpu: &mut Cpu, target: VirtTarget) {
    let val = cpu.regs.get_virt_reg(target);
    cpu.regs.set_virt_reg(target, val.wrapping_sub(1));
    cpu.pc += 1;
}
fn dec_nn_sp(cpu: &mut Cpu) {
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.pc += 1;
}

// SWAP n: Swap upper & lower nibbles of n.
fn swap_n(cpu: &mut Cpu, target: Target) {
    let val = cpu.regs.get_reg(target);
    let result = swap_n_helper(cpu, val);
    cpu.regs.set_reg(target, result);
    cpu.pc += 2;
}
fn swap_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let val = cpu.mem_bus.read_byte(address);
    let result = swap_n_helper(cpu, val);
    cpu.mem_bus.write_byte(address, result);
    cpu.pc += 2;
}
fn swap_n_helper(cpu: &mut Cpu, val: u8) -> u8 {
    let upper_nibble = (0xF0 & val) >> 4;
    let lower_nibble = 0x0F & val;
    let result = (lower_nibble << 4) | upper_nibble;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    result
}

// DAA: Adjust resgister A such that the correct representation of Binary Coded Decimal is
// obtained.
// Existing flag vals:
// N = 1 iff the previous operation was a subtraction.
// H = 1 iff there was a carry from bit 4 to 5
// C = 1 iff there was a carry from bit 8
//
// Implementation:
// Iff not subtracting && unit digit > 9, or there was a half carry, add 0x06 to A.
// Iff not subtracting && A > 0x99, or there was a full carry, add 0x60 to A.
fn daa(cpu: &mut Cpu) {
    let n_val = cpu.regs.get_flag(RegFlag::N);
    let h_val = cpu.regs.get_flag(RegFlag::H);
    let c_val = cpu.regs.get_flag(RegFlag::C);

    let original_value = cpu.regs.get_reg(A);
    let mut result = original_value;
    let mut correction: u8 = 0x00;

    if h_val || (!n_val && ((0x0F & original_value) > 0x09)) {
        correction |= 0x06;
    }
    let mut should_carry = false;
    if c_val || (!n_val && (original_value > 0x99)) {
        correction |= 0x60;
        should_carry = true;
    }

    if n_val {
        result = result.wrapping_sub(correction);
    } else {
        result = result.wrapping_add(correction);
    }

    cpu.regs.set_flag(RegFlag::Z, result == 0x0000);
    cpu.regs.set_flag(RegFlag::H, false);
    cpu.regs.set_flag(RegFlag::C, should_carry);

    cpu.regs.set_reg(A, result);

    cpu.pc += 1;
}

// CPL: Complement A register.
fn cpl(cpu: &mut Cpu) {
    cpu.regs.set_flag(RegFlag::N, true);
    cpu.regs.set_flag(RegFlag::H, true);
    cpu.regs.set_reg(A, cpu.regs.get_reg(A) ^ 0xFF);
    cpu.pc += 1;
}

// CCF: Complement carry flag.
fn ccf(cpu: &mut Cpu) {
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, false);

    if cpu.regs.get_flag(RegFlag::C) {
        cpu.regs.set_flag(RegFlag::C, false);
    } else {
        cpu.regs.set_flag(RegFlag::C, true);
    }
    cpu.pc += 1;
}

// SCF: Set carry flag.
fn scf(cpu: &mut Cpu) {
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, false);
    cpu.regs.set_flag(RegFlag::C, true);
    cpu.pc += 1;
}

// NOP: Do nothing.
fn nop(cpu: &mut Cpu) {
    cpu.pc += 1;
}

// HALT: Power down CPU until interrupt.
fn halt(cpu: &mut Cpu) {
    cpu.is_halted = true;
    cpu.pc += 1;
}

// STOP: Halt CPU & LCD display until button pressed.
fn stop(cpu: &mut Cpu) {
    cpu.is_stopped = true;
    cpu.pc += 2;
}

// DI: Disable interrupts after the instruction after DI is executed.
fn di(cpu: &mut Cpu) {
    cpu.di_countdown = 2;
    cpu.pc += 1;
}

// EI: Enable interrupts after the instruction after EI is executed.
fn ei(cpu: &mut Cpu) {
    cpu.ei_countdown = 2;
    cpu.pc += 1;
}

// RLCA: Rotate A left; set carry flag to original bit 7 in A.
fn rlca(cpu: &mut Cpu) {
    let original_val = cpu.regs.get_reg(A);
    let rotated_l = rlc_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_l);
    cpu.pc += 1;
}

// RLA: Rotate A left through carry flag.
fn rla(cpu: &mut Cpu) {
    let original_val = cpu.regs.get_reg(A);
    let rotated_l = rl_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_l);
    cpu.pc += 1;
}

// RRCA: Rotate A right; set carry flag to original bit 0 in A.
fn rrca(cpu: &mut Cpu) {
    let original_val = cpu.regs.get_reg(A);
    let rotated_r = rrc_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_r);
    cpu.pc += 1;
}

// RRA: Rotate A right through carry flag.
fn rra(cpu: &mut Cpu) {
    let original_val = cpu.regs.get_reg(A);
    let rotated_r = rr_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_r);
    cpu.pc += 1;
}

// RLC n: Rotate n left; set carry flag to original bit 7 in n.
fn rlc_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let rotated_l = rlc_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_l);
    cpu.pc += 2;
}
fn rlc_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let rotated_l = rlc_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, rotated_l);
    cpu.pc += 2;
}
fn rlc_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let bit_7 = original_val >> 7;
    let rotated_l = (original_val << 1) | bit_7;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, rotated_l == 0);
    cpu.regs.set_flag(RegFlag::C, bit_7 == 1);
    rotated_l
}

// RL n: Rotate n left through carry flag.
fn rl_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let rotated_l = rl_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_l);
    cpu.pc += 2;
}
fn rl_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let rotated_l = rl_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, rotated_l);
    cpu.pc += 2;
}
fn rl_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let bit_7 = original_val >> 7;
    let rotated_l = (original_val << 1) | if cpu.regs.get_flag(RegFlag::C) { 1 } else { 0 };

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, rotated_l == 0);
    cpu.regs.set_flag(RegFlag::C, bit_7 == 1);
    rotated_l
}

// RRC n: Rotate n right; set carry flag to original bit 0 in n.
fn rrc_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let rotated_r = rrc_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_r);
    cpu.pc += 2;
}
fn rrc_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let rotated_r = rrc_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, rotated_r);
    cpu.pc += 2;
}
fn rrc_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let bit_0 = original_val & 0x01;
    let rotated_r = (original_val >> 1) | (bit_0 << 7);

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, rotated_r == 0);
    cpu.regs.set_flag(RegFlag::C, bit_0 == 1);
    rotated_r
}

// RR n: Rotate n right through carry flag.
fn rr_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let rotated_r = rr_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_r);
    cpu.pc += 2;
}
fn rr_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let rotated_r = rr_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, rotated_r);
    cpu.pc += 2;
}
fn rr_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let bit_0 = original_val & 0x01;
    let rotated_r = (original_val >> 1)
        | if cpu.regs.get_flag(RegFlag::C) {
            0x80
        } else {
            0
        };

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, rotated_r == 0);
    cpu.regs.set_flag(RegFlag::C, bit_0 == 1);
    rotated_r
}

// SLA n: Shift n left into carry. LSB of n set to 0.
fn sla_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let result = sla_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, result);
    cpu.pc += 2;
}
fn sla_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let result = sla_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, result);
    cpu.pc += 2;
}
fn sla_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let result = original_val << 1;
    let bit_7 = original_val >> 7;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::C, bit_7 == 1);

    result
}

// SRA n: Shift n right into carry. MSB doesn't change.
fn sra_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let result = sra_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, result);
    cpu.pc += 2;
}
fn sra_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let result = sra_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, result);
    cpu.pc += 2;
}
fn sra_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let result = (original_val >> 1) | (original_val & 0x80);
    let bit_0 = original_val & 0x01;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::C, bit_0 == 1);

    result
}

// SRL n: Shift n right into carry. MSB of n set to 0.
fn srl_n(cpu: &mut Cpu, target: Target) {
    let original_val = cpu.regs.get_reg(target);
    let result = srl_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, result);
    cpu.pc += 2;
}
fn srl_n_hl(cpu: &mut Cpu) {
    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mem_bus.read_byte(address);
    let result = srl_n_helper(cpu, original_val);
    cpu.mem_bus.write_byte(address, result);
    cpu.pc += 2;
}
fn srl_n_helper(cpu: &mut Cpu, original_val: u8) -> u8 {
    let result = original_val >> 1;
    let bit_0 = original_val & 0x01;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::C, bit_0 == 1);

    result
}

// BIT b,r: Iff bit b in register r == 0, set Z flag = 1. Else, set Z flag = 0.
fn bit_b_r(cpu: &mut Cpu, b: usize, target: Target) {
    bit_b_r_helper(cpu, b, cpu.regs.get_reg(target));
}
fn bit_b_r_hl(cpu: &mut Cpu, b: usize) {
    let target_byte = cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL));
    bit_b_r_helper(cpu, b, target_byte)
}
fn bit_b_r_helper(cpu: &mut Cpu, b: usize, byte: u8) {
    let is_bit_zero = (byte & (0b1 << b)) == 0;
    cpu.regs.set_flag(RegFlag::Z, is_bit_zero);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, true);
    cpu.pc += 2;
}

// SET b,r: Set bit b in register r.
fn set_b_r(cpu: &mut Cpu, b: usize, target: Target) {
    let byte = cpu.regs.get_reg(target);
    cpu.regs.set_reg(target, byte | (0x01 << b));
    cpu.pc += 2;
}
fn set_b_r_hl(cpu: &mut Cpu, b: usize) {
    let address = cpu.regs.get_virt_reg(HL);
    let byte = cpu.mem_bus.read_byte(address);
    cpu.mem_bus.write_byte(address, byte | (0x01 << b));
    cpu.pc += 2;
}

// RES b,r: Reset bit b in register r.
fn res_b_r(cpu: &mut Cpu, b: usize, target: Target) {
    let byte = cpu.regs.get_reg(target);
    cpu.regs.set_reg(target, byte & !(0x01 << b));
    cpu.pc += 2;
}
fn res_b_r_hl(cpu: &mut Cpu, b: usize) {
    let address = cpu.regs.get_virt_reg(HL);
    let byte = cpu.mem_bus.read_byte(address);
    cpu.mem_bus.write_byte(address, byte & !(0x01 << b));
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
    jp_helper(cpu, cpu.regs.get_virt_reg(HL));
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

// RST n: Push current address to stack. Jump to address 0x0000 + n.
fn rst_n(cpu: &mut Cpu, n: u8) {
    cpu.push_stack(cpu.pc);
    cpu.pc = n as u16;
}

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
        assert_eq!(cpu.regs.get_virt_reg(BC), 0x3412);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0006);
        assert_eq!(cpu.regs.get_virt_reg(DE), 0x7856);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x0009);
        assert_eq!(cpu.regs.get_virt_reg(HL), 0xBC9A);

        cpu.cycle();
        assert_eq!(cpu.pc, 0x000C);
        assert_eq!(cpu.sp, 0xF0DE);
    }

    #[test]
    fn test_ld_hld_a() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(HL, 0x1234);
        cpu.regs.set_reg(A, 0xDC);
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
        cpu.regs.set_virt_reg(HL, 0x2050);
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
    fn test_push_pop_nn() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(BC, 0x1234);
        cpu.regs.set_virt_reg(DE, 0x5678);
        cpu.regs.set_virt_reg(HL, 0x9ABC);
        // Push BC, push DE, pop DE to AF, push HL, pop HL to BC, pop BC to HL
        let data = [0xC5, 0xD5, 0xF1, 0xE5, 0xC1, 0xE1];
        cpu.load(0x0000, &data);
        assert_eq!(cpu.sp, 0xFFFE);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFC);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFA);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFC);
        assert_eq!(cpu.regs.get_virt_reg(AF), 0x5678);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFA);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFC);
        assert_eq!(cpu.regs.get_virt_reg(BC), 0x9ABC);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFE);
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1234);

        cpu.regs.set_virt_reg(AF, 0xFEDC);
        // Push AF, pop AF to DE
        let data_2 = [0xF5, 0xD1];
        cpu.load(0x0006, &data_2);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFC);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFE);
        assert_eq!(cpu.regs.get_virt_reg(DE), 0xFEDC);
    }

    #[test]
    fn test_rst_n() {
        let mut cpu = Cpu::new();
        cpu.pc = 0xFEDC;
        // Push 0xFEDC to stack, jump to 0x0000.
        cpu.mem_bus.write_byte(0xFEDC, 0xC7);
        // Push 0x0000 to stack, jump to 0x0008.
        cpu.mem_bus.write_byte(0x0000, 0xCF);
        // Push 0x0008 to stack, jump to 0x0010.
        cpu.mem_bus.write_byte(0x0008, 0xD7);
        // Push 0x0010 to stack, jump to 0x0018.
        cpu.mem_bus.write_byte(0x0010, 0xDF);
        // Push 0x0018 to stack, jump to 0x0020.
        cpu.mem_bus.write_byte(0x0018, 0xE7);
        // Push 0x0020 to stack, jump to 0x0028.
        cpu.mem_bus.write_byte(0x0020, 0xEF);
        // Push 0x0028 to stack, jump to 0x0030.
        cpu.mem_bus.write_byte(0x0028, 0xF7);
        // Push 0x0030 to stack, jump to 0x0038.
        cpu.mem_bus.write_byte(0x0030, 0xFF);
        assert_eq!(cpu.sp, 0xFFFE);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFC);
        assert_eq!(cpu.pc, 0x0000);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFFA);
        assert_eq!(cpu.pc, 0x0008);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFF8);
        assert_eq!(cpu.pc, 0x0010);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFF6);
        assert_eq!(cpu.pc, 0x0018);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFF4);
        assert_eq!(cpu.pc, 0x0020);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFF2);
        assert_eq!(cpu.pc, 0x0028);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFF0);
        assert_eq!(cpu.pc, 0x0030);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFFEE);
        assert_eq!(cpu.pc, 0x0038);

        assert_eq!(cpu.pop_stack(), 0x0030);
        assert_eq!(cpu.pop_stack(), 0x0028);
        assert_eq!(cpu.pop_stack(), 0x0020);
        assert_eq!(cpu.pop_stack(), 0x0018);
        assert_eq!(cpu.pop_stack(), 0x0010);
        assert_eq!(cpu.pop_stack(), 0x0008);
        assert_eq!(cpu.pop_stack(), 0x0000);
    }

    #[test]
    fn test_ld_nn_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(B, 0x0E);
        cpu.regs.set_reg(C, 0x16);
        cpu.regs.set_reg(D, 0x1E);
        cpu.regs.set_reg(E, 0x26);
        cpu.regs.set_reg(H, 0x2E);
        cpu.regs.set_reg(L, 0x06);
        cpu.mem_bus.write_byte(0x0000, 0x06);
        cpu.pc = 0x0000;
        cpu.cycle();
        cpu.pc -= 1;
        assert_eq!(cpu.mem_bus.memory[cpu.pc as usize], 0x0E);
        cpu.cycle();
        cpu.pc -= 1;
        assert_eq!(cpu.mem_bus.memory[cpu.pc as usize], 0x16);
        cpu.cycle();
        cpu.pc -= 1;
        assert_eq!(cpu.mem_bus.memory[cpu.pc as usize], 0x1E);
        cpu.cycle();
        cpu.pc -= 1;
        assert_eq!(cpu.mem_bus.memory[cpu.pc as usize], 0x26);
        cpu.cycle();
        cpu.pc -= 1;
        assert_eq!(cpu.mem_bus.memory[cpu.pc as usize], 0x2E);
        cpu.cycle();
        cpu.pc -= 1;
        assert_eq!(cpu.mem_bus.memory[cpu.pc as usize], 0x06);
    }

    #[test]
    fn test_ld_r1_r2() {
        fn set_default_vals(cpu: &mut Cpu) {
            cpu.regs.set_reg(A, 0x12);
            cpu.regs.set_reg(B, 0x34);
            cpu.regs.set_reg(C, 0x56);
            cpu.regs.set_reg(D, 0x78);
            cpu.regs.set_reg(E, 0x9A);
            cpu.regs.set_reg(H, 0xBC);
            cpu.regs.set_reg(L, 0xDE);
        }

        let mut cpu = Cpu::new();
        cpu.pc = 0x0000;

        for r1 in Target::iter() {
            for r2 in Target::iter() {
                set_default_vals(&mut cpu);
                let prev_r2_val = cpu.regs.get_reg(r2);
                let prev_pc = cpu.pc;

                ld_r1_r2(&mut cpu, r1, r2);
                assert_eq!(cpu.pc, prev_pc + 1);
                assert_eq!(cpu.regs.get_reg(r1), cpu.regs.get_reg(r2));
                assert_eq!(cpu.regs.get_reg(r2), prev_r2_val);
            }
        }

        let test_byte: u8 = 0xFE;
        cpu.mem_bus.write_byte(cpu.regs.get_virt_reg(HL), test_byte);
        for r1 in Target::iter() {
            set_default_vals(&mut cpu);
            let prev_pc = cpu.pc;

            ld_r1_hl(&mut cpu, r1);
            assert_eq!(cpu.pc, prev_pc + 1);
            assert_eq!(cpu.regs.get_reg(r1), test_byte);
        }

        for r2 in Target::iter() {
            set_default_vals(&mut cpu);
            cpu.mem_bus.write_byte(cpu.regs.get_virt_reg(HL), test_byte);
            let prev_r2_val = cpu.regs.get_reg(r2);
            let prev_pc = cpu.pc;

            ld_hl_r2(&mut cpu, r2);
            assert_eq!(cpu.pc, prev_pc + 1);
            assert_eq!(
                cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
                cpu.regs.get_reg(r2)
            );
            assert_eq!(prev_r2_val, cpu.regs.get_reg(r2));
        }

        let data = [0x36, 0x2A];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x0002);
        assert_eq!(cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)), 0x2A);
    }

    #[test]
    fn test_ld_a_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0x01);
        cpu.regs.set_reg(B, 0x23);
        cpu.regs.set_reg(C, 0x45);
        cpu.regs.set_reg(D, 0x67);
        cpu.regs.set_reg(E, 0x89);
        cpu.regs.set_reg(H, 0xAB);
        cpu.regs.set_reg(L, 0xCD);
        cpu.mem_bus.write_byte(cpu.regs.get_virt_reg(BC), 0xFE);
        cpu.mem_bus.write_byte(cpu.regs.get_virt_reg(DE), 0xDC);
        cpu.mem_bus.write_byte(cpu.regs.get_virt_reg(HL), 0xBA);
        cpu.mem_bus.write_byte(0x7698, 0x2A);
        // Put values A, B, C, D, E, H, L, (BC), (DE), (HL), (nn), n into A.
        let data = [
            0x7F, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x0A, 0x1A, 0x7E, 0xFA, 0x98, 0x76, 0x3E,
            0x3A,
        ];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x01);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x23);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x45);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x67);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x89);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xAB);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xCD);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xDC);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xBA);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x2A);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x3A);
    }

    #[test]
    fn test_ld_n_a() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0xFF);
        cpu.regs.set_reg(B, 0x00);
        cpu.regs.set_reg(C, 0x00);
        cpu.regs.set_reg(D, 0x00);
        cpu.regs.set_reg(E, 0x00);
        cpu.regs.set_reg(H, 0x00);
        cpu.regs.set_reg(L, 0x00);
        // Put value A into A, B, C, D, E, H, L, (BC), (DE), (HL), (nn).
        let data = [
            0x7F, 0x47, 0x4F, 0x57, 0x5F, 0x67, 0x6F, 0x02, 0x12, 0x77, 0xEA, 0x77, 0x77,
        ];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFF);
        assert_eq!(cpu.regs.get_reg(B), 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0xFF);
        assert_eq!(cpu.regs.get_reg(C), 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0xFF);
        assert_eq!(cpu.regs.get_reg(D), 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0xFF);
        assert_eq!(cpu.regs.get_reg(E), 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0xFF);
        assert_eq!(cpu.regs.get_reg(H), 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0xFF);
        assert_eq!(cpu.regs.get_reg(L), 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0xFF);

        // Set regs to point to different addresses for testing
        cpu.regs.set_virt_reg(BC, 0x4444);
        cpu.regs.set_virt_reg(DE, 0x5555);
        cpu.regs.set_virt_reg(HL, 0x6666);

        // Make sure that the values at the addresses aren't already 0xFF.
        cpu.mem_bus.write_2_bytes(0x4444, 0x0000);
        cpu.mem_bus.write_2_bytes(0x5555, 0x0000);
        cpu.mem_bus.write_2_bytes(0x6666, 0x0000);
        cpu.mem_bus.write_2_bytes(0x7777, 0x0000);

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_2_bytes(0x4444), 0x00FF);
        assert_eq!(cpu.mem_bus.read_2_bytes(0x5555), 0x0000);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_2_bytes(0x5555), 0x00FF);
        assert_eq!(cpu.mem_bus.read_2_bytes(0x6666), 0x0000);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_2_bytes(0x6666), 0x00FF);
        assert_eq!(cpu.mem_bus.read_2_bytes(0x7777), 0x0000);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_2_bytes(0x7777), 0x00FF);
    }

    #[test]
    fn test_ld_a_c_c_a() {
        let mut cpu = Cpu::new();

        cpu.mem_bus.write_byte(0xFF12, 0xFF);
        cpu.mem_bus.write_byte(0xFF34, 0xEE);
        cpu.regs.set_reg(A, 0x00);
        cpu.regs.set_reg(C, 0x12);
        cpu.mem_bus.write_byte(0x0000, 0xF2);
        cpu.mem_bus.write_byte(0x0001, 0xE2);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFF);

        cpu.regs.set_reg(C, 0x34);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFF);
        assert_eq!(cpu.mem_bus.read_byte(0xFF34), 0xFF);
    }

    #[test]
    fn test_a_hld() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(HL, 0x1234);
        cpu.mem_bus.write_byte(0x1234, 0xFF);
        cpu.regs.set_reg(A, 0x00);
        cpu.mem_bus.write_byte(0x0000, 0x3A);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFF);
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1233);
    }

    #[test]
    fn test_a_hli_hli_a() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(HL, 0x1234);
        cpu.mem_bus.write_byte(0x1234, 0x00);
        cpu.mem_bus.write_byte(0x1235, 0xEE);
        cpu.mem_bus.write_byte(0x1236, 0x00);
        cpu.regs.set_reg(A, 0xFF);
        cpu.mem_bus.write_byte(0x0000, 0x22);
        cpu.mem_bus.write_byte(0x0001, 0x2A);
        cpu.mem_bus.write_byte(0x0002, 0x22);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0x1234), 0xFF);
        assert_eq!(cpu.regs.get_reg(A), 0xFF);
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1235);

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0x1235), 0xEE);
        assert_eq!(cpu.regs.get_reg(A), 0xEE);
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1236);

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0x1236), 0xEE);
        assert_eq!(cpu.regs.get_reg(A), 0xEE);
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1237);
    }

    #[test]
    fn test_ldh_n_a_a_n() {
        let mut cpu = Cpu::new();
        let data = [0xF0, 0x12, 0xE0, 0x34];
        cpu.mem_bus.write_byte(0xFF12, 0xFF);
        cpu.mem_bus.write_byte(0xFF34, 0x00);
        cpu.load(0x0000, &data);
        cpu.regs.set_reg(A, 0x00);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFF);

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0xFF34), 0xFF);
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(HL, 0x1234);
        cpu.sp = 0x0000;
        cpu.mem_bus.write_byte(0x0000, 0xF9);

        cpu.cycle();
        assert_eq!(cpu.sp, 0x1234);
    }

    #[test]
    fn test_ld_hl_sp_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        assert_eq!(cpu.sp, 0xFFFE);

        // Set HL = SP (0xFFFE) + 0x01.
        cpu.mem_bus.write_2_bytes(0x0000, 0x01F8);
        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0xFFFF);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0xF00E;
        // Set HL = SP (0xF00E) + 0x02; should set H flag.
        cpu.mem_bus.write_2_bytes(0x0002, 0x02F8);
        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0xF010);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0xF0D0;
        // Set HL = SP (0xF0D0) + 0x51; should set C flag.
        cpu.mem_bus.write_2_bytes(0x0004, 0x51F8);
        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0xF121);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0xF0DC;
        // Set HL = SP (0xF0DC) + 0x34; should set H & C flags.
        cpu.mem_bus.write_2_bytes(0x0006, 0x34F8);
        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0xF110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_ld_nn_sp() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x1234;
        // Put 0x1234 at (0x4321).
        let data = [0x08, 0x21, 0x43];
        cpu.load(0x0000, &data);
        cpu.mem_bus.write_2_bytes(0x1234, 0x0000);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0x4321), 0x34);
        assert_eq!(cpu.mem_bus.read_byte(0x4322), 0x12);
    }

    #[test]
    fn test_add_a_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0x00);
        cpu.regs.set_reg(B, 0x12);
        cpu.regs.set_reg(C, 0x34);
        cpu.regs.set_reg(D, 0x56);
        cpu.regs.set_reg(E, 0x78);
        cpu.regs.set_reg(H, 0x9A);
        cpu.regs.set_reg(L, 0xBC);
        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, false);
        cpu.regs.set_flag(RegFlag::C, false);
        cpu.mem_bus.write_byte(0x9ABC, 0x0E);
        // Add A, B, C, D, E, H, L, (HL), n to A
        let data = [0x87, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0xC6, 0x01];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x12);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x46);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x9C);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x14);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xAE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(A, 0xF0);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xAC);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xBA);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xBB);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_adc_a_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0x00);
        cpu.regs.set_reg(B, 0x01);
        cpu.regs.set_reg(C, 0xFF);
        cpu.regs.set_reg(D, 0x50);
        cpu.regs.set_reg(E, 0x0F);
        cpu.regs.set_reg(H, 0x9F);
        cpu.regs.set_reg(L, 0xFF);
        cpu.mem_bus.write_byte(0x9FFF, 0xFF);
        let data = [0x8F, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0xCE, 0x01];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x01);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x51);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x60);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFF);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_sub_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0x00);
        cpu.regs.set_reg(B, 0x0F);
        cpu.regs.set_reg(C, 0x01);
        cpu.regs.set_reg(D, 0xF0);
        cpu.regs.set_reg(E, 0x00);
        cpu.regs.set_reg(H, 0xCD);
        cpu.regs.set_reg(L, 0x67);
        cpu.mem_bus.write_byte(0xCD67, 0x02);
        let data = [0x97, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0xD6, 0x01, 0x96];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(A, 0xFE);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xEF);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xEE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x31);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xCA);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xC9);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xC7);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_sbc_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0x00);
        cpu.regs.set_reg(B, 0x0F);
        cpu.regs.set_reg(C, 0x01);
        cpu.regs.set_reg(D, 0xF0);
        cpu.regs.set_reg(E, 0xFF);
        cpu.regs.set_reg(H, 0xFF);
        cpu.regs.set_reg(L, 0x01);
        cpu.mem_bus.write_byte(0xFF01, 0x03);
        let data = [0x9F, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0xDE, 0x02, 0x9E];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(A, 0xFE);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xEF);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xEE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(A, 0x00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFE);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xFB);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0xF8);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_and_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0b1111_1011);
        cpu.regs.set_reg(B, 0b1111_1110);
        cpu.regs.set_reg(C, 0b1111_1001);
        cpu.regs.set_reg(D, 0b1111_0111);
        cpu.regs.set_reg(E, 0b1110_1111);
        cpu.regs.set_reg(H, 0b1110_0000);
        cpu.regs.set_reg(L, 0b0111_0110);
        cpu.mem_bus.write_byte(0b1110_0000_0111_0110, 0b0000_1111);
        let data = [
            0xA7,
            0xA0,
            0xA1,
            0xA2,
            0xA3,
            0xA4,
            0xA5,
            0xE6,
            0b0101_0101,
            0xA6,
        ];
        cpu.load(0x0000, &data);
        cpu.pc = 0;
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, false);
        cpu.regs.set_flag(RegFlag::C, true);

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1111_1011);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1111_1010);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1111_1000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1111_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1110_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1110_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0110_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0100_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
    }

    #[test]
    fn test_or_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.pc = 0x0000;
        cpu.regs.set_reg(A, 0b0000_0000);
        cpu.regs.set_reg(B, 0b1000_0001);
        cpu.regs.set_reg(C, 0b0000_1000);
        cpu.regs.set_reg(D, 0b0100_1000);
        cpu.regs.set_reg(E, 0b0010_1000);
        cpu.regs.set_reg(H, 0b0110_0000);
        cpu.regs.set_reg(L, 0b0000_0000);
        cpu.mem_bus.write_byte(0b0110_0000_0000_0000, 0b1111_0000);
        let data = [
            0xB7,
            0xB0,
            0xB1,
            0xB2,
            0xB3,
            0xB4,
            0xB5,
            0xF6,
            0b0000_0100,
            0xB6,
        ];
        cpu.load(0x0000, &data);

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1000_0001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1000_1001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1100_1001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1110_1001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1110_1001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1110_1001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1110_1101);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1111_1101);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
    }

    #[test]
    fn test_xor_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.pc = 0x0000;
        cpu.regs.set_reg(A, 0b1010_1010); // 0b0000_0000
        cpu.regs.set_reg(B, 0b0101_0101); // 0b0101_0101
        cpu.regs.set_reg(C, 0b0110_0110); // 0b0011_0011
        cpu.regs.set_reg(D, 0b0101_1011); // 0b0110_1000
        cpu.regs.set_reg(E, 0b0000_1010); // 0b0110_0010
        cpu.regs.set_reg(H, 0b1110_0100); // 0b1000_0110
        cpu.regs.set_reg(L, 0b0000_0011); // 0b1000_0101
        cpu.mem_bus.write_byte(0b1110_0100_0000_0011, 0b1111_0000); // 0b0111_0001
        let data = [
            0xAF,
            0xA8,
            0xA9,
            0xAA,
            0xAB,
            0xAC,
            0xAD,
            0xEE,
            0b0000_0100, // 0b1000_0001
            0xAE,
        ];
        cpu.load(0x0000, &data);

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0101_0101);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0011_0011);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0110_1000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0110_0010);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1000_0110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1000_0101);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1000_0001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0111_0001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
    }

    #[test]
    fn test_cp_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, false);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0x80);
        cpu.regs.set_reg(B, 0x40);
        cpu.regs.set_reg(C, 0x28);
        cpu.regs.set_reg(D, 0x80);
        cpu.regs.set_reg(E, 0xA0);
        cpu.regs.set_reg(H, 0xC8);
        cpu.regs.set_reg(L, 0xE0);
        let data = [0xBF, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xFE, 0x00, 0xBE];
        cpu.mem_bus.write_byte(0xC8E0, 0xF8);
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x80);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_inc_dec_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0x00);
        cpu.regs.set_reg(B, 0x10);
        cpu.regs.set_reg(C, 0x0F);
        cpu.regs.set_reg(D, 0xFF);
        cpu.regs.set_reg(E, 0x1F);
        cpu.regs.set_reg(H, 0xF0);
        cpu.regs.set_reg(L, 0x11);
        cpu.mem_bus.write_byte(0xF112, 0x23);
        let data = [
            0x3C, 0x04, 0x0C, 0x14, 0x1C, 0x24, 0x2C, 0x34, 0x3D, 0x05, 0x0D, 0x15, 0x1D, 0x25,
            0x2D, 0x35,
        ];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x01);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0x11);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0x10);
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0x20);
        assert!(cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0xF1);
        assert!(!cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0x12);
        assert!(!cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0xF112), 0x24);
        assert!(!cpu.regs.get_flag(RegFlag::H));

        // Start DEC n test
        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, false);
        cpu.regs.set_flag(RegFlag::H, false);
        cpu.regs.set_flag(RegFlag::C, false);

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0x10);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0x0F);
        assert!(cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0xFF);
        assert!(cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0x1F);
        assert!(cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0xF0);
        assert!(!cpu.regs.get_flag(RegFlag::H));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0x11);
        assert!(!cpu.regs.get_flag(RegFlag::H));

        cpu.regs.set_virt_reg(HL, 0xF112);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0xF112), 0x23);
        assert!(!cpu.regs.get_flag(RegFlag::H));
    }

    #[test]
    fn test_add_hl_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_virt_reg(HL, 0x0780);
        cpu.regs.set_virt_reg(BC, 0x0100);
        cpu.regs.set_virt_reg(DE, 0xFF01);
        cpu.sp = 0x1100;
        let data = [0x29, 0x09, 0x19, 0x39];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x0F00);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x0F01);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_virt_reg(HL, 0xFF00);
        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1000);
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_add_sp_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.sp = 0x0000;
        cpu.pc = 0x0000;
        let data = [
            0xE8,
            0x01,
            0xE8,
            0x0F,
            0xE8,
            0b1111_1111, // -1 / -0x01
            0xE8,
            0b1000_0001, // -127 / -0x7F
            0xE8,
            0b0111_1111, // 127 / 0x7F
            0xE8,
            0b1111_1111, // -1
        ];
        cpu.load(0x0000, &data);

        cpu.cycle();
        assert_eq!(cpu.sp, 0x0001);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0x0002;
        cpu.cycle();
        assert_eq!(cpu.sp, 0x0011);
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0x0030;
        cpu.cycle();
        assert_eq!(cpu.sp, 0x002F);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0x0091; // 145
        cpu.cycle(); // - 127
        assert_eq!(cpu.sp, 0x0012); // 18
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0xFFFE; // default
        cpu.cycle(); // + 127
        assert_eq!(cpu.sp, 0x007D); // 125
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.sp = 0x0000; // test underflow with subtraction
        cpu.cycle(); // -1
        assert_eq!(cpu.sp, 0xFFFF); // underflow
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_inc_dec_nn() {
        let mut cpu = Cpu::new();
        cpu.regs.set_virt_reg(BC, 0x001F);
        cpu.regs.set_virt_reg(DE, 0xFFFF);
        cpu.regs.set_virt_reg(HL, 0x0FFF);
        cpu.sp = 0xFF30;
        // INC BC, DE, HL, SP; DEC BC, DE, HL, SP.
        let data = [0x03, 0x13, 0x23, 0x33, 0x0B, 0x1B, 0x2B, 0x3B];
        cpu.load(0x0000, &data);

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(BC), 0x0020);

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(DE), 0x0000);

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x1000);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFF31);

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(BC), 0x001F);

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(DE), 0xFFFF);

        cpu.cycle();
        assert_eq!(cpu.regs.get_virt_reg(HL), 0x0FFF);

        cpu.cycle();
        assert_eq!(cpu.sp, 0xFF30);
    }

    #[test]
    fn test_swap_n() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0b0000_0000);
        cpu.regs.set_reg(B, 0b1010_1010);
        cpu.regs.set_reg(C, 0b1111_0000);
        cpu.regs.set_reg(D, 0b0000_1111);
        cpu.regs.set_reg(E, 0b0110_1001);
        cpu.regs.set_reg(H, 0b1010_0101);
        cpu.regs.set_reg(L, 0b0100_0000);
        cpu.mem_bus.write_byte(0b1010_0101_0100_0000, 0xAB);
        let data = [
            0xCB, 0x37, 0xCB, 0x30, 0xCB, 0x31, 0xCB, 0x32, 0xCB, 0x33, 0xCB, 0x34, 0xCB, 0x35,
            0xCB, 0x36,
        ];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;
        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0b1010_1010);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0b0000_1111);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0b1111_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0b1001_0110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0b0101_1010);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0b0000_0100);
        assert!(!cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.set_reg(H, 0b1010_0101);
        cpu.regs.set_reg(L, 0b0100_0000);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(0b1010_0101_0100_0000), 0xBA);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
    }

    #[test]
    fn test_daa() {
        let mut cpu = Cpu::new();
        let data = [0x27; 10];
        cpu.load(0x0000, &data);

        // Test convert 0x73 + 0x23 = 0x96; no change necessary.
        cpu.regs.reset_flags();
        cpu.regs.set_reg(A, 0x96);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x96);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x39 + 0x26 = 0x5F; want 0x65.
        cpu.regs.reset_flags();
        cpu.regs.set_reg(A, 0x5F);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x65);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x80 + 0x90 = 0x10 + carry; want 0x70 + carry.
        cpu.regs.reset_flags();
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0x10);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x70);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x08 + 0x09 = 0x11 + half carry; want 0x17.
        cpu.regs.reset_flags();
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_reg(A, 0x11);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x17);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x70 + 0x30 = 0xA0; want 0x00 + carry.
        cpu.regs.reset_flags();
        cpu.regs.set_reg(A, 0xA0);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x00);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x02 + 0x09 = 0x0B; want 0x11.
        cpu.regs.reset_flags();
        cpu.regs.set_reg(A, 0x0B);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x11);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Switch to testing subtraction.
        cpu.regs.reset_flags();
        cpu.regs.set_flag(RegFlag::N, true);

        // Test convert 0x19 - 0x07 = 0x12; no change necessary.
        cpu.regs.set_reg(A, 0x12);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x12);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x10 - 0x01 = 0x0F + half carry; want 0x09.
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_reg(A, 0x0F);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x09);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x00 - 0x01 = 0xFF + half carry + carry; want 0x99 + carry.
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0xFF);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x99);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test convert 0x03 - 0x10 = 0xF3 + carry; want 0x93 + carry.
        cpu.regs.set_flag(RegFlag::H, false);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0xF3);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0x93);
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_cpl() {
        let mut cpu = Cpu::new();
        let data = [0x2F, 0x2F, 0x2F, 0x2F];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, false);
        cpu.regs.set_flag(RegFlag::H, false);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0b0000_0000);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1111_1111);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::N));
        assert!(cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::C, false);
        cpu.regs.set_reg(A, 0b1111_1111);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0000_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_reg(A, 0b1010_1010);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0101_0101);

        cpu.regs.set_reg(A, 0b1001_0110);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0110_1001);
    }

    #[test]
    fn test_ccf_scf() {
        let mut cpu = Cpu::new();
        let data = [0x3F, 0x37, 0x37, 0x3F, 0x3F];
        cpu.load(0x0000, &data);
        cpu.pc = 0x0000;

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.cycle();
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::Z, false);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.cycle();
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.cycle();
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_good_stop() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0000;
        cpu.mem_bus.write_byte(0x0000, 0x10);
        cpu.mem_bus.write_byte(0x0001, 0x00);
        cpu.cycle();
    }

    #[test]
    #[should_panic]
    fn test_bad_stop() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0000;
        cpu.mem_bus.write_byte(0x0000, 0x10);
        cpu.mem_bus.write_byte(0x0001, 0xFF);
        cpu.cycle();
    }

    #[test]
    fn test_ei_di() {
        // TODO
        let mut cpu = Cpu::new();
        let data = [
            0xF3, 0x00, 0x00, 0x00, 0xFB, 0x00, 0x00, 0xF3, 0xFB, 0x00, 0x00,
        ];
        cpu.load(0x0000, &data);
        cpu.interrupts_enabled = true;
        cpu.pc = 0x0000;
        cpu.di_countdown = 0;
        cpu.ei_countdown = 0;

        cpu.cycle();
        assert!(cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 2);
        assert_eq!(cpu.ei_countdown, 0);
        cpu.cycle();
        assert!(cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 1);
        assert_eq!(cpu.ei_countdown, 0);
        cpu.cycle();
        assert!(!cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 0);
        cpu.cycle();
        assert!(!cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 0);
        cpu.cycle();
        assert!(!cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 2);
        cpu.cycle();
        assert!(!cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 1);
        cpu.cycle();
        assert!(cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 0);
        cpu.cycle();
        assert!(cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 2);
        assert_eq!(cpu.ei_countdown, 0);
        cpu.cycle();
        assert!(cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 1);
        assert_eq!(cpu.ei_countdown, 2);
        cpu.cycle();
        assert!(!cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 1);
        cpu.cycle();
        assert!(cpu.interrupts_enabled);
        assert_eq!(cpu.di_countdown, 0);
        assert_eq!(cpu.ei_countdown, 0);
    }

    #[test]
    fn test_ra() {
        let mut cpu = Cpu::new();
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.regs.set_reg(A, 0b0101_0011);
        cpu.pc = 0x0000;
        let data = [
            0x07, 0x07, 0x17, 0x17, 0x17, 0x0F, 0x0F, 0x1F, 0x1F, 0x1F, 0x1F, 0x07, 0x17, 0x0F,
            0x1F,
        ];
        cpu.load(0x0000, &data);

        // Test RLCA
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1010_0110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0100_1101);
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test RLA
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1001_1011);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0011_0110);
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0110_1101);
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test RRCA
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1011_0110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0101_1011);
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test RRA
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0010_1101);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1001_0110);
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1100_1011);
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0110_0101);
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test zero flag
        cpu.regs.reset_flags();
        cpu.regs.set_reg(A, 0x00);
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.reset_flags();
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.reset_flags();
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.reset_flags();
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));
    }

    #[test]
    #[allow(clippy::same_item_push)]
    fn test_rotates() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0000;
        // 2x RLC, 3x RL, 2x RRC, 4x RR, RLC, RL, RRC, RR
        let mut data: Vec<u8> = Vec::new();
        // Targets are denoted by the second digit of the opcode
        let target_num_l: [u8; 8] = [0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let target_num_r: [u8; 8] = [0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E];
        // For each target...
        for i in 0..8 {
            // 2x RLC
            for _ in 0..2 {
                data.push(0xCB);
                data.push(target_num_l[i]);
            }
            // 3x RL
            for _ in 0..3 {
                data.push(0xCB);
                data.push(0x10 | target_num_l[i]);
            }
            // 2x RRC
            for _ in 0..2 {
                data.push(0xCB);
                data.push(target_num_r[i]);
            }
            // 4x RR
            for _ in 0..4 {
                data.push(0xCB);
                data.push(0x10 | target_num_r[i])
            }
            // RLC
            data.push(0xCB);
            data.push(target_num_l[i]);
            // RL
            data.push(0xCB);
            data.push(0x10 | target_num_l[i]);
            // RRC
            data.push(0xCB);
            data.push(target_num_r[i]);
            // RR
            data.push(0xCB);
            data.push(0x10 | target_num_r[i])
        }

        cpu.load(0x0000, &data);
        for target in Target::iter() {
            cpu.regs.set_flag(RegFlag::Z, true);
            cpu.regs.set_flag(RegFlag::N, true);
            cpu.regs.set_flag(RegFlag::H, true);
            cpu.regs.set_flag(RegFlag::C, true);
            cpu.regs.set_reg(target, 0b0101_0011);

            // Test RLC
            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b1010_0110);
            assert!(!cpu.regs.get_flag(RegFlag::Z));
            assert!(!cpu.regs.get_flag(RegFlag::N));
            assert!(!cpu.regs.get_flag(RegFlag::H));
            assert!(!cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b0100_1101);
            assert!(cpu.regs.get_flag(RegFlag::C));

            // Test RL
            cpu.regs.set_flag(RegFlag::Z, true);
            cpu.regs.set_flag(RegFlag::N, true);
            cpu.regs.set_flag(RegFlag::H, true);
            cpu.regs.set_flag(RegFlag::C, true);
            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b1001_1011);
            assert!(!cpu.regs.get_flag(RegFlag::Z));
            assert!(!cpu.regs.get_flag(RegFlag::N));
            assert!(!cpu.regs.get_flag(RegFlag::H));
            assert!(!cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b0011_0110);
            assert!(cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b0110_1101);
            assert!(!cpu.regs.get_flag(RegFlag::C));

            // Test RRC
            cpu.regs.set_flag(RegFlag::Z, true);
            cpu.regs.set_flag(RegFlag::N, true);
            cpu.regs.set_flag(RegFlag::H, true);
            cpu.regs.set_flag(RegFlag::C, false);
            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b1011_0110);
            assert!(!cpu.regs.get_flag(RegFlag::Z));
            assert!(!cpu.regs.get_flag(RegFlag::N));
            assert!(!cpu.regs.get_flag(RegFlag::H));
            assert!(cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b0101_1011);
            assert!(!cpu.regs.get_flag(RegFlag::C));

            // Test RR
            cpu.regs.set_flag(RegFlag::Z, true);
            cpu.regs.set_flag(RegFlag::N, true);
            cpu.regs.set_flag(RegFlag::H, true);
            cpu.regs.set_flag(RegFlag::C, false);
            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b0010_1101);
            assert!(!cpu.regs.get_flag(RegFlag::Z));
            assert!(!cpu.regs.get_flag(RegFlag::N));
            assert!(!cpu.regs.get_flag(RegFlag::H));
            assert!(cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b1001_0110);
            assert!(cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b1100_1011);
            assert!(!cpu.regs.get_flag(RegFlag::C));

            cpu.cycle();
            assert_eq!(cpu.regs.get_reg(target), 0b0110_0101);
            assert!(cpu.regs.get_flag(RegFlag::C));

            // Test zero flag
            cpu.regs.reset_flags();
            cpu.regs.set_reg(target, 0x00);
            cpu.cycle();
            assert!(cpu.regs.get_flag(RegFlag::Z));

            cpu.regs.reset_flags();
            cpu.cycle();
            assert!(cpu.regs.get_flag(RegFlag::Z));

            cpu.regs.reset_flags();
            cpu.cycle();
            assert!(cpu.regs.get_flag(RegFlag::Z));

            cpu.regs.reset_flags();
            cpu.cycle();
            assert!(cpu.regs.get_flag(RegFlag::Z));
        }

        let address = cpu.regs.get_virt_reg(HL);

        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.mem_bus.write_byte(address, 0b0101_0011);

        // Test RLC
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b1010_0110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b0100_1101);
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test RL
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, true);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b1001_1011);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b0011_0110);
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b0110_1101);
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test RRC
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b1011_0110);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b0101_1011);
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // Test RR
        cpu.regs.set_flag(RegFlag::Z, true);
        cpu.regs.set_flag(RegFlag::N, true);
        cpu.regs.set_flag(RegFlag::H, true);
        cpu.regs.set_flag(RegFlag::C, false);
        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b0010_1101);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b1001_0110);
        assert!(cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b1100_1011);
        assert!(!cpu.regs.get_flag(RegFlag::C));

        cpu.cycle();
        assert_eq!(cpu.mem_bus.read_byte(address), 0b0110_0101);
        assert!(cpu.regs.get_flag(RegFlag::C));

        // Test zero flag
        cpu.regs.reset_flags();
        cpu.mem_bus.write_byte(address, 0x00);
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.reset_flags();
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.reset_flags();
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));

        cpu.regs.reset_flags();
        cpu.cycle();
        assert!(cpu.regs.get_flag(RegFlag::Z));
    }

    #[test]
    fn test_sla_sra_srl() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0000;
        fn setup(cpu: &mut Cpu) {
            cpu.regs.set_reg(A, 0b1000_0000);
            cpu.regs.set_reg(B, 0b0000_0000);
            cpu.regs.set_reg(C, 0b0000_0001);
            cpu.regs.set_reg(D, 0b1010_1010);
            cpu.regs.set_reg(E, 0b0101_0101);
            cpu.regs.set_reg(H, 0b0110_1001);
            cpu.regs.set_reg(L, 0b0111_1110);
            cpu.regs.set_flag(RegFlag::Z, false);
            cpu.regs.set_flag(RegFlag::N, true);
            cpu.regs.set_flag(RegFlag::H, true);
            cpu.regs.set_flag(RegFlag::C, false);
        }
        let data = [
            0xCB, 0x27, 0xCB, 0x20, 0xCB, 0x21, 0xCB, 0x22, 0xCB, 0x23, 0xCB, 0x24, 0xCB, 0x25,
            0xCB, 0x26, 0xCB, 0x2F, 0xCB, 0x28, 0xCB, 0x29, 0xCB, 0x2A, 0xCB, 0x2B, 0xCB, 0x2C,
            0xCB, 0x2D, 0xCB, 0x2E, 0xCB, 0x3F, 0xCB, 0x38, 0xCB, 0x39, 0xCB, 0x3A, 0xCB, 0x3B,
            0xCB, 0x3C, 0xCB, 0x3D, 0xCB, 0x3E,
        ];
        cpu.load(0x0000, &data);

        // SLA n
        setup(&mut cpu);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0b0000_0010);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0b0101_0100);
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0b1010_1010);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0b1101_0010);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0b1111_1100);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.mem_bus
            .write_byte(cpu.regs.get_virt_reg(HL), 0b1010_1010);
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b0101_0100
        );
        assert!(cpu.regs.get_flag(RegFlag::C));

        // SRA n
        setup(&mut cpu);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b1100_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0b1101_0101);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0b0010_1010);
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0b0011_0100);
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0b0011_1111);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.mem_bus
            .write_byte(cpu.regs.get_virt_reg(HL), 0b1010_1010);
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1101_0101
        );
        assert!(!cpu.regs.get_flag(RegFlag::C));

        // SRL n
        setup(&mut cpu);
        // cpu.regs.set_reg(A, 0b1000_0000);
        // cpu.regs.set_reg(B, 0b0000_0000);
        // cpu.regs.set_reg(C, 0b0000_0001);
        // cpu.regs.set_reg(D, 0b1010_1010);
        // cpu.regs.set_reg(E, 0b0101_0101);
        // cpu.regs.set_reg(H, 0b0110_1001);
        // cpu.regs.set_reg(L, 0b0111_1110);
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(A), 0b0100_0000);
        assert!(!cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::N));
        assert!(!cpu.regs.get_flag(RegFlag::H));
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(B), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(C), 0b0000_0000);
        assert!(cpu.regs.get_flag(RegFlag::Z));
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(D), 0b0101_0101);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(E), 0b0010_1010);
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(H), 0b0011_0100);
        assert!(cpu.regs.get_flag(RegFlag::C));
        cpu.cycle();
        assert_eq!(cpu.regs.get_reg(L), 0b0011_1111);
        assert!(!cpu.regs.get_flag(RegFlag::C));
        cpu.mem_bus
            .write_byte(cpu.regs.get_virt_reg(HL), 0b1010_1010);
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b0101_0101
        );
        assert!(!cpu.regs.get_flag(RegFlag::C));
    }

    #[test]
    fn test_set_b_r() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0b0000_0000);
        cpu.regs.set_reg(B, 0b1010_1010);
        cpu.regs.set_reg(C, 0b0101_0101);
        cpu.regs.set_reg(D, 0b0000_1111);
        cpu.regs.set_reg(E, 0b1111_0000);
        cpu.regs.set_reg(H, 0b1100_0011);
        cpu.regs.set_reg(L, 0b1111_1111);
        let mut data: Vec<u8> = Vec::new();
        let target_even_opcode_suffixes = [0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let target_odd_opcode_suffixes = [0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E];
        let opcode_prefixes = [0xC0, 0xD0, 0xE0, 0xF0];
        for i in 0..8 {
            for j in 0..8 {
                let prefix = opcode_prefixes[j / 2];
                let suffix = if j % 2 == 0 {
                    target_even_opcode_suffixes[i]
                } else {
                    target_odd_opcode_suffixes[i]
                };
                data.push(0xCB);
                data.push(prefix | suffix);
            }
        }
        cpu.pc = 0x0000;
        cpu.load(0x0000, &data);

        for target in Target::iter() {
            for i in 0..8 {
                cpu.cycle();
                assert_eq!((cpu.regs.get_reg(target) & 2_u8.pow(i)) >> i, 1);
            }
        }

        cpu.mem_bus
            .write_byte(cpu.regs.get_virt_reg(HL), 0b1010_1010);
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1011,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1011,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1111,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1111,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1011_1111,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1011_1111,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1111_1111,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1111_1111,
        );
    }

    #[test]
    fn test_res_b_r() {
        let mut cpu = Cpu::new();
        cpu.regs.set_reg(A, 0b0000_0000);
        cpu.regs.set_reg(B, 0b1010_1010);
        cpu.regs.set_reg(C, 0b0101_0101);
        cpu.regs.set_reg(D, 0b0000_1111);
        cpu.regs.set_reg(E, 0b1111_0000);
        cpu.regs.set_reg(H, 0b1100_0011);
        cpu.regs.set_reg(L, 0b1111_1111);
        let mut data: Vec<u8> = Vec::new();
        let target_even_opcode_suffixes = [0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let target_odd_opcode_suffixes = [0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E];
        let opcode_prefixes = [0x80, 0x90, 0xA0, 0xB0];
        for i in 0..8 {
            for j in 0..8 {
                let prefix = opcode_prefixes[j / 2];
                let suffix = if j % 2 == 0 {
                    target_even_opcode_suffixes[i]
                } else {
                    target_odd_opcode_suffixes[i]
                };
                data.push(0xCB);
                data.push(prefix | suffix);
            }
        }
        cpu.pc = 0x0000;
        cpu.load(0x0000, &data);

        for target in Target::iter() {
            for i in 0..8 {
                cpu.cycle();
                assert_eq!((cpu.regs.get_reg(target) & 2_u8.pow(i)) >> i, 0);
            }
        }

        cpu.mem_bus
            .write_byte(cpu.regs.get_virt_reg(HL), 0b1010_1010);
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1010,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1000,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_1000,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_0000,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1010_0000,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1000_0000,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b1000_0000,
        );
        cpu.cycle();
        assert_eq!(
            cpu.mem_bus.read_byte(cpu.regs.get_virt_reg(HL)),
            0b0000_0000,
        );
    }
}
