#![cfg(test)]

use std::time::Instant;

use rbca_core::Cpu;

#[test]
#[ignore]
fn test_boot_rom() {
    const SLOW: bool = false;
    const WAIT_MS: u64 = 10;

    let mut cpu = Cpu::new();
    cpu.mem_bus.load_cart("../roms/tetris.gb", true);

    let cpu_cart = cpu.mem_bus.cart.as_ref().unwrap();
    println!("{}", cpu_cart.header_info());

    let mut t_cycles = 0;
    'testloop: loop {
        let t_start = Instant::now();
        t_cycles += cpu.cycle();

        if cpu.pc == 0x0100 {
            break 'testloop;
        }

        if SLOW {
            std::thread::sleep(std::time::Duration::from_millis(WAIT_MS));
        } else if t_cycles >= 17476 {
            t_cycles %= 17476;
            while t_start.elapsed().as_millis() < 16 {}
        }
    }

    println!("Done!");
}
