#![cfg(test)]

use std::time::Instant;

mod common;

use common::CpuState;

use rbca_core::{
    Cpu, RegFlag,
    Target::{self, A, B, C, D, E, H, L},
};
use text_io::read;

fn test_common(rom_name: &str) {
    const BREAKPOINTS: bool = true;
    // const BREAKPOINTS: bool = false;
    // const SLOW: bool = true;
    const SLOW: bool = false;
    const WAIT_MS: u64 = 10;

    fn is_breakpoint(cpu: &Cpu, last_state: &CpuState, total_cycles: u128) -> bool {
        (cpu.regs.get_flag(RegFlag::C) != last_state.regs.get_flag(RegFlag::C)) || (cpu.pc > 0xC000)
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
    cpu.regs.set_reg(D, 0xC0);
    cpu.regs.set_reg(E, 0xD8);
    cpu.regs.set_reg(H, 0x01);
    cpu.regs.set_reg(L, 0x4D);

    let mut last_state = CpuState::save_state(&cpu);

    let mut t_cycles = 0;
    let mut total_cycles: u128 = 0;

    let mut blargg_out = String::new();

    loop {
        last_state.update(&cpu);
        let t_start = Instant::now();
        t_cycles += cpu.cycle();
        println!("[{}]blargg_out: {}", blargg_out.len(), blargg_out);
        total_cycles += cpu.cycle() as u128;

        // breakpoints
        if BREAKPOINTS & is_breakpoint(&cpu, &last_state, total_cycles) {
            println!(" - BREAK - ");
            let _: String = read!();
        }

        // blargg output
        if cpu.mem_bus.read_byte(0xFF02) == 0x81 {
            let c: char = cpu.mem_bus.read_byte(0xFF01).into();
            blargg_out.push(c);
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
