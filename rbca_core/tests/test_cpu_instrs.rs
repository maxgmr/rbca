#![cfg(test)]

use std::time::Instant;

mod common;

use common::CpuState;

use rbca_core::{
    Cpu, RegFlag,
    Target::{A, B, C, D, E, H, L},
    DEBUG_INSTRUCTIONS,
};
use text_io::read;

fn test_common(rom_name: &str) {
    // const BREAKPOINTS: bool = true;
    const BREAKPOINTS: bool = false;
    // const SLOW: bool = true;
    const SLOW: bool = false;
    const WAIT_MS: u64 = 10;

    #[allow(unused_variables)]
    fn is_breakpoint(cpu: &Cpu, last_state: &CpuState, total_cycles: u128) -> bool {
        cpu.regs.get_reg(D) != last_state.regs.get_reg(D)
        // (cpu.regs.get_flag(RegFlag::C) != last_state.regs.get_flag(RegFlag::C)) || (cpu.pc > 0xC000)
    }

    let mut cpu = Cpu::new();
    cpu.mem_bus.load_cart(
        &format!("../roms/gb-test-roms/cpu_instrs/individual/{}", rom_name),
        false,
    );

    let cpu_cart = cpu.mem_bus.cart.as_ref().unwrap();
    println!("{}", cpu_cart.header_info());
    println!(
        "Breakpoints: {} Slow: {}",
        BREAKPOINTS,
        if SLOW {
            format!("{} ms", WAIT_MS)
        } else {
            "false".to_owned()
        }
    );
    println!("Enter any text to continue...");
    let _: String = read!();
    println!("-------");

    cpu.pc = 0x0100;
    cpu.sp = 0xFFFE;
    cpu.regs.set_reg(A, 0x01);
    cpu.regs.set_reg(B, 0x00);
    cpu.regs.set_reg(C, 0x13);
    cpu.regs.set_reg(D, 0x00);
    cpu.regs.set_reg(E, 0xD8);
    cpu.regs.set_reg(H, 0x01);
    cpu.regs.set_reg(L, 0x4D);
    cpu.regs.set_flag(RegFlag::Z, true);
    cpu.regs.set_flag(RegFlag::N, false);
    cpu.regs.set_flag(RegFlag::H, true);
    cpu.regs.set_flag(RegFlag::C, true);

    let mut last_state = CpuState::save_state(&cpu);

    let mut t_cycles = 0;
    let mut total_cycles: u128 = 0;

    let mut blargg_out = String::new();

    loop {
        last_state.update(&cpu);
        let t_start = Instant::now();
        let cycles = cpu.cycle();
        cpu.mem_bus.cycle(cycles);
        t_cycles += cycles;
        total_cycles += cycles as u128;

        if DEBUG_INSTRUCTIONS {
            println!("blargg_out: {blargg_out}");
        }

        // breakpoints
        if BREAKPOINTS & is_breakpoint(&cpu, &last_state, total_cycles) {
            println!(" - BREAK - ");
            let _: String = read!();
        }

        // blargg output
        if cpu.mem_bus.read_byte(0xFF02) == 0x81 {
            let c: char = cpu.mem_bus.read_byte(0xFF01).into();
            blargg_out.push(c);
            if !DEBUG_INSTRUCTIONS {
                print!("{c}");
            }
            cpu.mem_bus.write_byte(0xFF02, 0x00);
        }

        if SLOW {
            std::thread::sleep(std::time::Duration::from_millis(WAIT_MS));
        } else if t_cycles >= 17476 {
            t_cycles %= 17476;
            while t_start.elapsed().as_millis() < 16 {}
        }
    }
}

// Example usage: cargo t 01 -- --nocapture --ignored
#[test]
#[ignore]
fn test_cpu_01() {
    test_common("01-special.gb");
}

#[test]
#[ignore]
fn test_cpu_02() {
    test_common("02-interrupts.gb");
}

#[test]
#[ignore]
fn test_cpu_03() {
    test_common("03-op sp,hl.gb");
}

#[test]
#[ignore]
fn test_cpu_04() {
    test_common("04-op r,imm.gb");
}

#[test]
#[ignore]
fn test_cpu_05() {
    test_common("05-op rp.gb");
}

#[test]
#[ignore]
fn test_cpu_06() {
    test_common("06-ld r,r.gb");
}

#[test]
#[ignore]
fn test_cpu_07() {
    test_common("07-jr,jp,call,ret,rst.gb");
}

#[test]
#[ignore]
fn test_cpu_08() {
    test_common("08-misc instrs.gb");
}

#[test]
#[ignore]
fn test_cpu_09() {
    test_common("09-op r,r.gb");
}

#[test]
#[ignore]
fn test_cpu_10() {
    test_common("10-bit ops.gb");
}

#[test]
#[ignore]
fn test_cpu_11() {
    test_common("11-op a,(hl).gb");
}
