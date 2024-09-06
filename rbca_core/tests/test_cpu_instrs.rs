#![cfg(test)]

use std::time::Instant;

use rbca_core::Cpu;
use text_io::read;

fn test_common(rom_name: &str) {
    const BREAKPOINTS: bool = true;
    const SLOW: bool = false;
    const WAIT_MS: u64 = 10;

    fn is_breakpoint(pc: u16, total_cycles: u128) -> bool {
        pc == 0xC747
    }

    let mut cpu = Cpu::new();
    cpu.mem_bus.load_cart(
        &format!("../roms/gb-test-roms/cpu_instrs/individual/{}", rom_name),
        false,
    );

    let cpu_cart = cpu.mem_bus.cart.as_ref().unwrap();
    println!("{}", cpu_cart.header_info());
    cpu.pc = 0x0100;

    let mut t_cycles = 0;
    let mut total_cycles: u128 = 0;
    loop {
        let t_start = Instant::now();
        t_cycles += cpu.cycle();
        total_cycles += cpu.cycle() as u128;

        // breakpoints
        if BREAKPOINTS & is_breakpoint(cpu.pc, total_cycles) {
            return;
            println!(" - BREAK - ");
            let _: String = read!();
        }

        // blargg output
        if cpu.mem_bus.read_byte(0xFF02) == 0x81 {
            let c: char = cpu.mem_bus.read_byte(0xFF01).into();
            print!("{}", c);
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
