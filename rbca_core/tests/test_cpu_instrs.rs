use std::{
    thread,
    time::{Duration, Instant},
};

use rbca_core::Cpu;

#[test]
#[ignore]
fn test_cpu_01() {
    let mut cpu = Cpu::new();
    cpu.mem_bus
        .load_cart("../roms/gb-test-roms/cpu_instrs/individual/01-special.gb");

    let cpu_cart = cpu.mem_bus.cart.as_ref().unwrap();
    println!("{}", cpu_cart.header_info());

    loop {
        let t_start = Instant::now();
        let mut t_cycles = 0;
        t_cycles += cpu.cycle();
        thread::sleep(Duration::from_millis(10));
    }
}
