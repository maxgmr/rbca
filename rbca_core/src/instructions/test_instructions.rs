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
        0xC2, 0xFF, 0xFF, 0xCA, 0xFF, 0xFF, 0xD2, 0xFF, 0xFF, 0xDA, 0xFF, 0xFF, 0xC3, 0x00, 0x20,
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
        0x7F, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x0A, 0x1A, 0x7E, 0xFA, 0x98, 0x76, 0x3E, 0x3A,
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
        0x3C, 0x04, 0x0C, 0x14, 0x1C, 0x24, 0x2C, 0x34, 0x3D, 0x05, 0x0D, 0x15, 0x1D, 0x25, 0x2D,
        0x35,
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
        0xCB, 0x37, 0xCB, 0x30, 0xCB, 0x31, 0xCB, 0x32, 0xCB, 0x33, 0xCB, 0x34, 0xCB, 0x35, 0xCB,
        0x36,
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
        0x07, 0x07, 0x17, 0x17, 0x17, 0x0F, 0x0F, 0x1F, 0x1F, 0x1F, 0x1F, 0x07, 0x17, 0x0F, 0x1F,
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
        0xCB, 0x27, 0xCB, 0x20, 0xCB, 0x21, 0xCB, 0x22, 0xCB, 0x23, 0xCB, 0x24, 0xCB, 0x25, 0xCB,
        0x26, 0xCB, 0x2F, 0xCB, 0x28, 0xCB, 0x29, 0xCB, 0x2A, 0xCB, 0x2B, 0xCB, 0x2C, 0xCB, 0x2D,
        0xCB, 0x2E, 0xCB, 0x3F, 0xCB, 0x38, 0xCB, 0x39, 0xCB, 0x3A, 0xCB, 0x3B, 0xCB, 0x3C, 0xCB,
        0x3D, 0xCB, 0x3E,
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

#[test]
fn test_call_nn() {
    let mut cpu = Cpu::new();
    let data = [0xCD, 0x34, 0x12];
    cpu.load(0x0000, &data);
    cpu.pc = 0x0000;
    cpu.cycle();
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.pop_stack(), 0x0003);
}

#[test]
fn test_call_cc_nn() {
    let mut cpu = Cpu::new();
    let data = [
        0xC4, 0xFF, 0xFF, 0xCC, 0xFF, 0xFF, 0xD4, 0xFF, 0xFF, 0xDC, 0xFF, 0xFF, 0xC4, 0x00, 0x01,
    ];
    cpu.load(0x0000, &data);
    cpu.mem_bus.write_byte(0x0100, 0xCC);
    cpu.mem_bus.write_2_bytes(0x0101, 0x0200);
    cpu.mem_bus.write_byte(0x0200, 0xD4);
    cpu.mem_bus.write_2_bytes(0x0201, 0x0300);
    cpu.mem_bus.write_byte(0x0300, 0xDC);
    cpu.mem_bus.write_2_bytes(0x0301, 0x0400);
    cpu.pc = 0x0000;

    // Conditions not met- should not jump.
    cpu.regs.set_flag(RegFlag::Z, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0003);
    cpu.regs.set_flag(RegFlag::Z, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0006);
    cpu.regs.set_flag(RegFlag::C, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0009);
    cpu.regs.set_flag(RegFlag::C, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x000C);

    // Conditions met- should jump!
    cpu.regs.set_flag(RegFlag::Z, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0100);
    cpu.regs.set_flag(RegFlag::Z, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0200);
    cpu.regs.set_flag(RegFlag::C, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0300);
    cpu.regs.set_flag(RegFlag::C, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0400);

    assert_eq!(cpu.pop_stack(), 0x0303);
    assert_eq!(cpu.pop_stack(), 0x0203);
    assert_eq!(cpu.pop_stack(), 0x0103);
    assert_eq!(cpu.pop_stack(), 0x000F);
}

#[test]
fn test_ret() {
    let mut cpu = Cpu::new();
    cpu.push_stack(0x9ABC);
    cpu.push_stack(0x5678);
    cpu.push_stack(0x1234);
    cpu.mem_bus.write_byte(0x0000, 0xC9);
    cpu.mem_bus.write_byte(0x1234, 0xC9);
    cpu.mem_bus.write_byte(0x5678, 0xC9);
    cpu.pc = 0x0000;

    cpu.cycle();
    assert_eq!(cpu.pc, 0x1234);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x5678);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x9ABC);
}

#[test]
fn test_ret_cc() {
    let mut cpu = Cpu::new();
    let data = [0xC0, 0xC8, 0xD0, 0xD8, 0xC0];
    cpu.push_stack(0x0400);
    cpu.push_stack(0x0300);
    cpu.push_stack(0x0200);
    cpu.push_stack(0x0100);
    cpu.load(0x0000, &data);
    cpu.mem_bus.write_byte(0x0100, 0xC8);
    cpu.mem_bus.write_byte(0x0200, 0xD0);
    cpu.mem_bus.write_byte(0x0300, 0xD8);
    cpu.pc = 0x0000;

    // Conditions not met- should not jump.
    cpu.regs.set_flag(RegFlag::Z, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0001);
    cpu.regs.set_flag(RegFlag::Z, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0002);
    cpu.regs.set_flag(RegFlag::C, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0003);
    cpu.regs.set_flag(RegFlag::C, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0004);

    // Conditions met- should jump!
    cpu.regs.set_flag(RegFlag::Z, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0100);
    cpu.regs.set_flag(RegFlag::Z, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0200);
    cpu.regs.set_flag(RegFlag::C, false);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0300);
    cpu.regs.set_flag(RegFlag::C, true);
    cpu.cycle();
    assert_eq!(cpu.pc, 0x0400);
}

#[test]
fn test_reti() {
    let mut cpu = Cpu::new();
    cpu.push_stack(0x1234);
    cpu.interrupts_enabled = false;
    cpu.mem_bus.write_byte(0x0000, 0xD9);
    cpu.pc = 0x0000;

    let initial_sp = cpu.sp;
    cpu.cycle();
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.sp - 2, initial_sp);
    assert_eq!(cpu.ei_countdown, 1);
    assert!(!cpu.interrupts_enabled);
    cpu.cycle();
    assert_eq!(cpu.ei_countdown, 0);
    assert!(cpu.interrupts_enabled);
}

#[test]
#[should_panic]
fn illegal_opcode() {
    let mut cpu = Cpu::new();
    let data = [0xD3];
    cpu.load(0x0000, &data);
    cpu.pc = 0x0000;
    cpu.cycle();
}
