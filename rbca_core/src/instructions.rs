//! All functionality related to the CPU instructions.
use crate::{
    Cpu, EmuState, RegFlag,
    Target::{self, A, B, C, D, E, H, L},
    VirtTarget::{self, AF, BC, DE, HL},
};

/// Execute a given opcode. Return the amount of cycles the instruction takes and any debug info.
pub fn execute_opcode(
    cpu: &mut Cpu,
    opcode: u8,
    debug: bool,
    get_state: bool,
) -> (u32, Option<EmuState>) {
    let mut emu_state = if get_state {
        Some(EmuState::new(cpu))
    } else {
        None
    };

    let (size, cycles, instruction_string) = match opcode {
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
        0xCD => call_nn(cpu),

        // CALL cc,nn
        0xC4 => call_cc_nn(cpu, RegFlag::Z, false),
        0xCC => call_cc_nn(cpu, RegFlag::Z, true),
        0xD4 => call_cc_nn(cpu, RegFlag::C, false),
        0xDC => call_cc_nn(cpu, RegFlag::C, true),

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
        0xC9 => ret(cpu),

        // RET cc
        0xC0 => ret_cc(cpu, RegFlag::Z, false),
        0xC8 => ret_cc(cpu, RegFlag::Z, true),
        0xD0 => ret_cc(cpu, RegFlag::C, false),
        0xD8 => ret_cc(cpu, RegFlag::C, true),

        // RETI
        0xD9 => reti(cpu),

        // CB-Opcodes
        0xCB => {
            let ext_opcode = cpu.mmu.read_byte(cpu.pc + 1);
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

        // Illegal opcodes
        0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
            panic!("Illegal opcode {:#02X} at {:#04X}", opcode, cpu.pc)
        }

        // Unimplemented instruction
        #[allow(unreachable_patterns)]
        _ => panic!("Unimplemented opcode {:#02X} at {:#04X}", opcode, cpu.pc),
    };

    if debug {
        debug_print(cpu, size, cycles, &instruction_string);
    }

    if let Some(ref mut es) = emu_state {
        es.update(size, cycles, instruction_string);
    }

    (cycles, emu_state)
}

fn debug_print(cpu: &Cpu, size: u16, _cycles: u32, instruction_string: &str) {
    let mut data = format!("{:#04X}", cpu.mmu.read_byte(cpu.pc));
    if size > 1 {
        data.push_str(&format!(" {:#04X}", cpu.mmu.read_byte(cpu.pc + 1)));
    }
    if size > 2 {
        data.push_str(&format!(" {:#04X}", cpu.mmu.read_byte(cpu.pc + 2)));
    }

    let flags: [char; 4] = [
        if cpu.regs.get_flag(RegFlag::Z) {
            'Z'
        } else {
            '-'
        },
        if cpu.regs.get_flag(RegFlag::N) {
            'N'
        } else {
            '-'
        },
        if cpu.regs.get_flag(RegFlag::H) {
            'H'
        } else {
            '-'
        },
        if cpu.regs.get_flag(RegFlag::C) {
            'C'
        } else {
            '-'
        },
    ];

    println!(
        "{:#06X} | {:<14} | {:<10} | A: {:02X} F: {} BC: {:04X} DE: {:04X} HL: {:04X}",
        cpu.pc,
        data,
        instruction_string,
        cpu.regs.get_reg(A),
        flags.iter().collect::<String>(),
        cpu.regs.get_virt_reg(BC),
        cpu.regs.get_virt_reg(DE),
        cpu.regs.get_virt_reg(HL),
    );
}

fn cc_print(flag: RegFlag, expected_value: bool) -> String {
    match (flag, expected_value) {
        (RegFlag::Z, false) => "NZ".to_string(),
        (RegFlag::Z, true) => "Z".to_string(),
        (RegFlag::C, false) => "NC".to_string(),
        (RegFlag::C, true) => "C".to_string(),
        _ => String::new(),
    }
}

// ----------------------------------------------------
// FUNCTIONS
// ----------------------------------------------------

// LD nn,n: Set nn = 8-bit immediate value n.
fn ld_nn_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("LD {target},n");

    cpu.regs.set_reg(target, cpu.mmu.read_byte(cpu.pc + 1));
    cpu.pc += size;
    (size, cycles, instruction_string)
}

// LD r1,r2: Set r1 = r2.
fn ld_r1_r2(cpu: &mut Cpu, r1: Target, r2: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("LD {r1},{r2}");

    cpu.regs.set_reg(r1, cpu.regs.get_reg(r2));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_r1_hl(cpu: &mut Cpu, r1: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("LD {r1},(HL)");

    let address = cpu.regs.get_virt_reg(HL);
    let value = cpu.mmu.read_byte(address);
    cpu.regs.set_reg(r1, value);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_hl_r2(cpu: &mut Cpu, r2: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("LD (HL),{r2}");

    let address = cpu.regs.get_virt_reg(HL);
    cpu.mmu.write_byte(address, cpu.regs.get_reg(r2));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_hl_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 12;
    let instruction_string = "LD (HL),n";

    let address = cpu.regs.get_virt_reg(HL);
    let value = cpu.get_next_byte();
    cpu.mmu.write_byte(address, value);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD A,n: Set A = n.
fn ld_a_vr(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("LD A,{target}");

    let address = cpu.regs.get_virt_reg(target);
    let value = cpu.mmu.read_byte(address);
    ld_a_n_helper(cpu, value);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_a_nn(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 3;
    let cycles = 16;
    let instruction_string = "LD A,nn";

    let address = cpu.get_next_2_bytes();
    let value = cpu.mmu.read_byte(address);
    ld_a_n_helper(cpu, value);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn ld_a_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "LD A,n";

    let value = cpu.get_next_byte();
    ld_a_n_helper(cpu, value);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn ld_a_n_helper(cpu: &mut Cpu, value: u8) {
    cpu.regs.set_reg(A, value);
}

// LD n,A: Set n = A.
fn ld_r_a(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("LD {target},A");

    cpu.regs.set_reg(target, cpu.regs.get_reg(A));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_vr_a(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("LD {target},A");

    let address = cpu.regs.get_virt_reg(target);
    cpu.mmu.write_byte(address, cpu.regs.get_reg(A));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_nn_a(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 3;
    let cycles = 16;
    let instruction_string = "LD nn,A";

    let address = cpu.get_next_2_bytes();
    cpu.mmu.write_byte(address, cpu.regs.get_reg(A));

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD A,(C): Set A = (0xFF00 + C).
fn ld_a_c(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD A,(C)";

    let address = 0xFF00 | (cpu.regs.get_reg(C) as u16);
    cpu.regs.set_reg(A, cpu.mmu.read_byte(address));

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD (C),A: Set (0xFF00 + C) = A.
fn ld_c_a(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD (C),A";

    let address = 0xFF00 | (cpu.regs.get_reg(C) as u16);
    cpu.mmu.write_byte(address, cpu.regs.get_reg(A));

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD A,(HLD): Set A = (HL). HL -= 1.
fn ld_a_hld(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD A,(HLD)";

    ld_a_hl_helper(cpu, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD (HLD),A: Set (HL) = A. HL -= 1.
fn ld_hld_a(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD (HLD),A";

    ld_hl_a_helper(cpu, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD A,(HLI): Set A = (HL). HL += 1.
fn ld_a_hli(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD A,(HLI)";

    ld_a_hl_helper(cpu, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD (HLI),A: Set (HL) = A. HL += 1.
fn ld_hli_a(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD (HLI),A";

    ld_hl_a_helper(cpu, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

fn ld_a_hl_helper(cpu: &mut Cpu, is_inc: bool) {
    let address = cpu.regs.get_virt_reg(HL);
    cpu.regs.set_reg(A, cpu.mmu.read_byte(address));

    let new_val = if is_inc { address + 1 } else { address - 1 };
    cpu.regs.set_virt_reg(HL, new_val);
}
fn ld_hl_a_helper(cpu: &mut Cpu, is_inc: bool) {
    let address = cpu.regs.get_virt_reg(HL);
    cpu.mmu.write_byte(address, cpu.regs.get_reg(A));

    let new_val = if is_inc { address + 1 } else { address - 1 };
    cpu.regs.set_virt_reg(HL, new_val);
}

// LDH (n),A: Set (0xFF00 + n) = A.
fn ldh_n_a(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 12;
    let instruction_string = "LDH (n),A";

    let address = 0xFF00 | (cpu.get_next_byte() as u16);
    cpu.mmu.write_byte(address, cpu.regs.get_reg(A));

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LDH A,(n): Set A = (0xFF00 + n).
fn ldh_a_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 12;
    let instruction_string = "LDH A,(n)";

    let address = 0xFF00 | (cpu.get_next_byte() as u16);
    cpu.regs.set_reg(A, cpu.mmu.read_byte(address));

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD n,nn: Set n = nn.
fn ld_n_nn(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 3;
    let cycles = 12;
    let instruction_string = format!("LD {target},n");

    let nn = cpu.get_next_2_bytes();
    cpu.regs.set_virt_reg(target, nn);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn ld_n_nn_sp(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 3;
    let cycles = 12;
    let instruction_string = "LD SP,nn";

    let nn = cpu.get_next_2_bytes();
    cpu.sp = nn;

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD SP,HL: Set SP = HL.
fn ld_sp_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "LD SP,HL";

    cpu.sp = cpu.regs.get_virt_reg(HL);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// LD HL,SP+n: Set HL = SP + n.
// Iff n is positive, set H iff carry on lowest nibble.
// Iff n is positive, set C iff carry on lowest byte.
// Iff n is negative, set H iff lowest nibble is decreased.
// Iff n is negative, set C iff lowest byte is decreased.
fn ld_hl_sp_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 12;
    let instruction_string = "LD HL,SP+n";

    let result = sp_n_helper(cpu);
    cpu.regs.set_virt_reg(HL, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn ld_nn_sp(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 3;
    let cycles = 20;
    let instruction_string = "LD (nn),SP";

    let address = cpu.get_next_2_bytes();
    cpu.mmu.write_2_bytes(address, cpu.sp);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// PUSH nn: Push virtual register nn to stack.
fn push_nn(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 16;
    let instruction_string = format!("PUSH {target}");

    cpu.push_stack(cpu.regs.get_virt_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}

// POP nn: Pop 2 bytes off stack into virtual register nn.
fn pop_nn(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 12;
    let instruction_string = format!("POP {target}");

    let popped_val = cpu.pop_stack();
    cpu.regs.set_virt_reg(target, popped_val);

    cpu.pc += size;
    (size, cycles, instruction_string)
}

// ADD A,n: A += n.
fn add_a_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("ADD A,{target}");

    add_a_n_helper(cpu, cpu.regs.get_reg(target), false);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn add_a_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "ADD A,(HL)";

    let n = cpu.mmu.read_byte(cpu.regs.get_virt_reg(HL));
    add_a_n_helper(cpu, n, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn add_a_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "ADD A,n";

    let n = cpu.get_next_byte();
    add_a_n_helper(cpu, n, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn adc_a_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("ADC A,{target}");

    add_a_n_helper(cpu, cpu.regs.get_reg(target), true);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn adc_a_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "ADC A,(HL)";

    let n = cpu.mmu.read_byte(cpu.regs.get_virt_reg(HL));
    add_a_n_helper(cpu, n, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn adc_a_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "ADC A,n";

    let n = cpu.get_next_byte();
    add_a_n_helper(cpu, n, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// SUB n: A -= n.
fn sub_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("SUB {target}");

    sub_n_helper(cpu, cpu.regs.get_reg(target), false);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn sub_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "SUB (HL)";

    let n = cpu.mmu.read_byte(cpu.regs.get_virt_reg(HL));
    sub_n_helper(cpu, n, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn sub_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "SUB n";

    let n = cpu.get_next_byte();
    sub_n_helper(cpu, n, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn sbc_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("SBC A,{target}");

    sub_n_helper(cpu, cpu.regs.get_reg(target), true);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn sbc_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "SBC A,(HL)";

    let n = cpu.mmu.read_byte(cpu.regs.get_virt_reg(HL));
    sub_n_helper(cpu, n, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn sbc_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "SBC A,n";

    let n = cpu.get_next_byte();
    sub_n_helper(cpu, n, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// AND n: Set A = A AND n.
fn and_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("AND {target}");

    and_n_helper(cpu, cpu.regs.get_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn and_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "AND (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mmu.read_byte(address);
    and_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn and_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "AND n";

    let n = cpu.get_next_byte();
    and_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn and_n_helper(cpu: &mut Cpu, n: u8) {
    let result = cpu.regs.get_reg(A) & n;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::H, true);

    cpu.regs.set_reg(A, result);
}

// OR n: Set A = A OR n.
fn or_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("OR {target}");

    or_n_helper(cpu, cpu.regs.get_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn or_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "OR (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mmu.read_byte(address);
    or_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn or_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "OR n";

    let n = cpu.get_next_byte();
    or_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn or_n_helper(cpu: &mut Cpu, n: u8) {
    let result = cpu.regs.get_reg(A) | n;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);

    cpu.regs.set_reg(A, result);
}

// XOR n: Set A = A XOR n.
fn xor_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("XOR {target}");

    xor_n_helper(cpu, cpu.regs.get_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn xor_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "XOR (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mmu.read_byte(address);
    xor_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn xor_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "XOR n";

    let n = cpu.get_next_byte();
    xor_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn xor_n_helper(cpu: &mut Cpu, n: u8) {
    let result = cpu.regs.get_reg(A) ^ n;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);

    cpu.regs.set_reg(A, result);
}

// CP n: Compare A with n.
fn cp_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("CP {target}");

    cp_n_helper(cpu, cpu.regs.get_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn cp_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "CP (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let n = cpu.mmu.read_byte(address);
    cp_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn cp_n_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = "CP n";

    let n = cpu.get_next_byte();
    cp_n_helper(cpu, n);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn cp_n_helper(cpu: &mut Cpu, n: u8) {
    let a_val = cpu.regs.get_reg(A);
    sub_n_helper(cpu, n, false);
    cpu.regs.set_reg(A, a_val);
}

// INC n: n += 1.
fn inc_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("INC {target}");

    let reg_val = cpu.regs.get_reg(target);
    let result = reg_val.wrapping_add(1);

    inc_n_set_flags(cpu, reg_val, result);

    cpu.regs.set_reg(target, result);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn inc_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 12;
    let instruction_string = "INC (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let val = cpu.mmu.read_byte(address);
    let result = val.wrapping_add(1);

    inc_n_set_flags(cpu, val, result);

    cpu.mmu.write_byte(address, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn inc_n_set_flags(cpu: &mut Cpu, val: u8, result: u8) {
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, ((0x0F & val) + 1) > 0x0F);
}

// DEC n: n -= 1.
fn dec_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = format!("DEC {target}");

    let reg_val = cpu.regs.get_reg(target);
    let result = reg_val.wrapping_sub(1);

    dec_n_set_flags(cpu, reg_val, result);

    cpu.regs.set_reg(target, result);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn dec_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 12;
    let instruction_string = "DEC (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let val = cpu.mmu.read_byte(address);
    let result = val.wrapping_sub(1);

    dec_n_set_flags(cpu, val, result);

    cpu.mmu.write_byte(address, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn dec_n_set_flags(cpu: &mut Cpu, val: u8, result: u8) {
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    cpu.regs.set_flag(RegFlag::N, true);
    cpu.regs.set_flag(RegFlag::H, (0x0F & val) == 0);
}

// ADD HL,n: HL += n.
fn add_hl_n(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("ADD HL,{target}");

    add_hl_n_helper(cpu, cpu.regs.get_virt_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn add_hl_n_sp(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "ADD HL,SP";

    add_hl_n_helper(cpu, cpu.sp);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn add_sp_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "ADD SP,n";

    let result = sp_n_helper(cpu);
    cpu.sp = result;

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// INC nn: nn += 1.
fn inc_nn(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("INC {target}");

    let val = cpu.regs.get_virt_reg(target);
    cpu.regs.set_virt_reg(target, val.wrapping_add(1));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn inc_nn_sp(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "INC SP";

    cpu.sp = cpu.sp.wrapping_add(1);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// DEC nn: nn -= 1.
fn dec_nn(cpu: &mut Cpu, target: VirtTarget) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = format!("DEC {target}");

    let val = cpu.regs.get_virt_reg(target);
    cpu.regs.set_virt_reg(target, val.wrapping_sub(1));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn dec_nn_sp(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 8;
    let instruction_string = "DEC SP";

    cpu.sp = cpu.sp.wrapping_sub(1);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// SWAP n: Swap upper & lower nibbles of n.
fn swap_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("SWAP {target}");

    let val = cpu.regs.get_reg(target);
    let result = swap_n_helper(cpu, val);
    cpu.regs.set_reg(target, result);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn swap_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "SWAP (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let val = cpu.mmu.read_byte(address);
    let result = swap_n_helper(cpu, val);
    cpu.mmu.write_byte(address, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}
fn swap_n_helper(cpu: &mut Cpu, val: u8) -> u8 {
    let upper_nibble = (0xF0 & val) >> 4;
    let lower_nibble = 0x0F & val;
    let result = (lower_nibble << 4) | upper_nibble;

    cpu.regs.reset_flags();
    cpu.regs.set_flag(RegFlag::Z, result == 0);
    result
}

// DAA: Adjust register A such that the correct representation of Binary Coded Decimal is
// obtained.
// Existing flag vals:
// N = 1 iff the previous operation was a subtraction.
// H = 1 iff there was a carry from bit 4 to 5
// C = 1 iff there was a carry from bit 8
//
// Implementation:
// Iff not subtracting && unit digit > 9, or there was a half carry, add 0x06 to A.
// Iff not subtracting && A > 0x99, or there was a full carry, add 0x60 to A.
fn daa(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "DAA";

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

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// CPL: Complement A register.
fn cpl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "CPL";

    cpu.regs.set_flag(RegFlag::N, true);
    cpu.regs.set_flag(RegFlag::H, true);
    cpu.regs.set_reg(A, cpu.regs.get_reg(A) ^ 0xFF);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// CCF: Complement carry flag.
fn ccf(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "CCF";

    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, false);

    if cpu.regs.get_flag(RegFlag::C) {
        cpu.regs.set_flag(RegFlag::C, false);
    } else {
        cpu.regs.set_flag(RegFlag::C, true);
    }

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// SCF: Set carry flag.
fn scf(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "SCF";

    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, false);
    cpu.regs.set_flag(RegFlag::C, true);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// NOP: Do nothing.
fn nop(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "NOP";

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// HALT: Power down CPU until interrupt.
fn halt(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "HALT";

    cpu.is_halted = true;

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// STOP: Halt CPU & LCD display until button pressed.
fn stop(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 4;
    let instruction_string = "STOP";

    cpu.is_stopped = true;

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// DI: Disable interrupts after the instruction after DI is executed.
fn di(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "DI";

    cpu.di_countdown = 2;

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// EI: Enable interrupts after the instruction after EI is executed.
fn ei(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "EI";

    cpu.ei_countdown = 2;

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// RLCA: Rotate A left; set carry flag to original bit 7 in A.
fn rlca(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "RLCA";

    let original_val = cpu.regs.get_reg(A);
    let rotated_l = rlc_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_l);
    cpu.regs.set_flag(RegFlag::Z, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// RLA: Rotate A left through carry flag.
fn rla(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "RLA";

    let original_val = cpu.regs.get_reg(A);
    let rotated_l = rl_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_l);
    cpu.regs.set_flag(RegFlag::Z, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// RRCA: Rotate A right; set carry flag to original bit 0 in A.
fn rrca(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "RRCA";

    let original_val = cpu.regs.get_reg(A);
    let rotated_r = rrc_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_r);
    cpu.regs.set_flag(RegFlag::Z, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// RRA: Rotate A right through carry flag.
fn rra(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "RRA";

    let original_val = cpu.regs.get_reg(A);
    let rotated_r = rr_n_helper(cpu, original_val);
    cpu.regs.set_reg(A, rotated_r);
    cpu.regs.set_flag(RegFlag::Z, false);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
}

// RLC n: Rotate n left; set carry flag to original bit 7 in n.
fn rlc_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("RLC {target}");

    let original_val = cpu.regs.get_reg(target);
    let rotated_l = rlc_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_l);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn rlc_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "RLC (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let rotated_l = rlc_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, rotated_l);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn rl_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("RL {target}");

    let original_val = cpu.regs.get_reg(target);
    let rotated_l = rl_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_l);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn rl_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "RL (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let rotated_l = rl_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, rotated_l);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn rrc_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("RRC {target}");

    let original_val = cpu.regs.get_reg(target);
    let rotated_r = rrc_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_r);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn rrc_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "RRC (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let rotated_r = rrc_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, rotated_r);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn rr_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("RR {target}");

    let original_val = cpu.regs.get_reg(target);
    let rotated_r = rr_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, rotated_r);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn rr_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "RR (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let rotated_r = rr_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, rotated_r);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn sla_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("SLA {target}");

    let original_val = cpu.regs.get_reg(target);
    let result = sla_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, result);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn sla_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "SLA (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let result = sla_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn sra_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("SRA {target}");

    let original_val = cpu.regs.get_reg(target);
    let result = sra_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, result);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn sra_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "SRA (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let result = sra_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn srl_n(cpu: &mut Cpu, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("SRL {target}");

    let original_val = cpu.regs.get_reg(target);
    let result = srl_n_helper(cpu, original_val);
    cpu.regs.set_reg(target, result);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn srl_n_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = "SRL (HL)";

    let address = cpu.regs.get_virt_reg(HL);
    let original_val = cpu.mmu.read_byte(address);
    let result = srl_n_helper(cpu, original_val);
    cpu.mmu.write_byte(address, result);

    cpu.pc += size;
    (size, cycles, String::from(instruction_string))
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
fn bit_b_r(cpu: &mut Cpu, b: usize, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("BIT {b},{target}");

    bit_b_r_helper(cpu, b, cpu.regs.get_reg(target));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn bit_b_r_hl(cpu: &mut Cpu, b: usize) -> (u16, u32, String) {
    let size = 2;
    let cycles = 12;
    let instruction_string = format!("BIT {b},(HL)");

    let target_byte = cpu.mmu.read_byte(cpu.regs.get_virt_reg(HL));
    bit_b_r_helper(cpu, b, target_byte);

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn bit_b_r_helper(cpu: &mut Cpu, b: usize, byte: u8) {
    let is_bit_zero = (byte & (0b1 << b)) == 0;
    cpu.regs.set_flag(RegFlag::Z, is_bit_zero);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, true);
}

// SET b,r: Set bit b in register r.
fn set_b_r(cpu: &mut Cpu, b: usize, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("SET {b},{target}");

    let byte = cpu.regs.get_reg(target);
    cpu.regs.set_reg(target, byte | (0x01 << b));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn set_b_r_hl(cpu: &mut Cpu, b: usize) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = format!("SET {b},(HL)");

    let address = cpu.regs.get_virt_reg(HL);
    let byte = cpu.mmu.read_byte(address);
    cpu.mmu.write_byte(address, byte | (0x01 << b));

    cpu.pc += size;
    (size, cycles, instruction_string)
}

// RES b,r: Reset bit b in register r.
fn res_b_r(cpu: &mut Cpu, b: usize, target: Target) -> (u16, u32, String) {
    let size = 2;
    let cycles = 8;
    let instruction_string = format!("RES {b},{target}");

    let byte = cpu.regs.get_reg(target);
    cpu.regs.set_reg(target, byte & !(0x01 << b));

    cpu.pc += size;
    (size, cycles, instruction_string)
}
fn res_b_r_hl(cpu: &mut Cpu, b: usize) -> (u16, u32, String) {
    let size = 2;
    let cycles = 16;
    let instruction_string = format!("RES {b},(HL)");

    let address = cpu.regs.get_virt_reg(HL);
    let byte = cpu.mmu.read_byte(address);
    cpu.mmu.write_byte(address, byte & !(0x01 << b));

    cpu.pc += size;
    (size, cycles, instruction_string)
}

// JP nn: Jump to address nn.
fn jp_nn(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 3;
    let cycles = 16;
    let instruction_string = "JP nn";

    let nn = cpu.get_next_2_bytes();

    jp_helper(cpu, nn);
    (size, cycles, String::from(instruction_string))
}

// JP cc,nn: Iff C/Z flag == true/false, jump to address nn.
fn jp_cc_nn(cpu: &mut Cpu, flag: RegFlag, expected_value: bool) -> (u16, u32, String) {
    let size = 3;
    let cycles;
    let instruction_string = format!("JP {},nn", cc_print(flag, expected_value));

    let test_val = match flag {
        RegFlag::Z | RegFlag::C => cpu.regs.get_flag(flag),
        _ => panic!("jr_cc_n: Cannot use flag {:?}. C or Z flags only.", flag),
    };

    if test_val == expected_value {
        cycles = 16;
        let nn = cpu.get_next_2_bytes();
        jp_helper(cpu, nn);
    } else {
        cycles = 12;
        cpu.pc += size;
    }
    (size, cycles, instruction_string)
}

// JP (HL): Jump to address contained in (HL).
fn jp_hl(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 4;
    let instruction_string = "JP (HL)";

    jp_helper(cpu, cpu.regs.get_virt_reg(HL));
    (size, cycles, String::from(instruction_string))
}

// JR n: Add n to current address & jump to it.
fn jr_n(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 2;
    let cycles = 12;
    let instruction_string = "JR n";

    let n = cpu.get_next_byte() as i8;

    cpu.pc += size;
    jp_helper(cpu, ((cpu.pc as u32 as i32) + (n as i32)) as u16);
    (size, cycles, String::from(instruction_string))
}

// JR cc,n: Iff C/Z flag == true/false, add n to current address & jump to it.
fn jr_cc_n(cpu: &mut Cpu, flag: RegFlag, expected_value: bool) -> (u16, u32, String) {
    let size = 2;
    let cycles;
    let instruction_string = format!("JR {},n", cc_print(flag, expected_value));

    let test_val = match flag {
        RegFlag::Z | RegFlag::C => cpu.regs.get_flag(flag),
        _ => panic!("jr_cc_n: Cannot use flag {:?}. C or Z flags only.", flag),
    };

    if test_val == expected_value {
        let n = cpu.get_next_byte() as i8;
        cycles = 12;
        cpu.pc += size;
        jp_helper(cpu, ((cpu.pc as u32 as i32) + (n as i32)) as u16);
    } else {
        cycles = 8;
        cpu.pc += size;
    }
    (size, cycles, instruction_string)
}

// Helper function for jumps
fn jp_helper(cpu: &mut Cpu, address: u16) {
    cpu.pc = address;
}

// CALL nn: Push address of next instruction onto stack. Jump to address nn.
fn call_nn(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 3;
    let cycles = 24;
    let instruction_string = "CALL nn";

    cpu.push_stack(cpu.pc + 3);

    cpu.pc = cpu.get_next_2_bytes();
    (size, cycles, String::from(instruction_string))
}

// CALL cc,nn: Iff condition cc == true, push address of next instruction to stack & jump to address
// nn.
fn call_cc_nn(cpu: &mut Cpu, flag: RegFlag, expected_value: bool) -> (u16, u32, String) {
    let size = 3;
    let cycles;
    let instruction_string = format!("CALL {},nn", cc_print(flag, expected_value));

    let test_val = match flag {
        RegFlag::Z | RegFlag::C => cpu.regs.get_flag(flag),
        _ => panic!("call_cc_nn: Cannot use flag {:?}. C or Z flags only.", flag),
    };
    if test_val == expected_value {
        cycles = 24;
        cpu.push_stack(cpu.pc + size);
        cpu.pc = cpu.get_next_2_bytes();
    } else {
        cycles = 12;
        cpu.pc += size;
    }

    (size, cycles, instruction_string)
}

// RST n: Push current address to stack. Jump to address 0x0000 + n.
fn rst_n(cpu: &mut Cpu, n: u8) -> (u16, u32, String) {
    let size = 1;
    let cycles = 16;
    let instruction_string = "RST n";

    cpu.pc += size;
    cpu.push_stack(cpu.pc);

    cpu.pc = n as u16;
    (size, cycles, String::from(instruction_string))
}

// RET: Pop two bytes from the stack. Jump to that address.
fn ret(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 16;
    let instruction_string = "RET";

    cpu.pc = cpu.pop_stack();
    (size, cycles, String::from(instruction_string))
}

// RET cc: Iff condition cc == true, pop two bytes from the stack & jump to that address.
fn ret_cc(cpu: &mut Cpu, flag: RegFlag, expected_value: bool) -> (u16, u32, String) {
    let size = 1;
    let cycles;
    let instruction_string = format!("RET {}", cc_print(flag, expected_value));

    let test_val = match flag {
        RegFlag::Z | RegFlag::C => cpu.regs.get_flag(flag),
        _ => panic!("call_cc_nn: Cannot use flag {:?}. C or Z flags only.", flag),
    };
    if test_val == expected_value {
        cycles = 20;
        cpu.pc = cpu.pop_stack();
    } else {
        cycles = 8;
        cpu.pc += 1;
    }

    (size, cycles, instruction_string)
}

// RETI: Pop two bytes from stack. Jump to the address. Enable interrupts.
fn reti(cpu: &mut Cpu) -> (u16, u32, String) {
    let size = 1;
    let cycles = 16;
    let instruction_string = "RETI";

    cpu.pc = cpu.pop_stack();
    cpu.ei_countdown = 1;
    (size, cycles, String::from(instruction_string))
}

#[cfg(test)]
mod test_instructions;
