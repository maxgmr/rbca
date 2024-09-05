use std::{
    thread,
    time::{Duration, Instant},
};

use rbca_core::{Cpu, RegFlag};

// Example usage: cargo t 01 -- --nocapture --ignored
#[test]
#[ignore]
fn test_cpu_01() {
    let mut cpu = Cpu::new();
    cpu.mem_bus
        .load_cart("../roms/gb-test-roms/cpu_instrs/individual/01-special.gb");

    let cpu_cart = cpu.mem_bus.cart.as_ref().unwrap();
    println!("{}", cpu_cart.header_info());

    let mut t_cycles = 0;
    loop {
        let t_start = Instant::now();
        t_cycles += cpu.cycle();
        println!("{} | {}", cpu.regs.regs_string(), cpu.regs.flags_string());
        // thread::sleep(Duration::from_millis(1));
        if t_cycles >= 17476 {
            t_cycles %= 17476;
            while t_start.elapsed().as_millis() < 8 {}
        }
    }
}
