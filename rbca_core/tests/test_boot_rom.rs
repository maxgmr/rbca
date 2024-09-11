#![cfg(test)]

use std::time::Instant;

use rbca_core::Cpu;
use text_io::read;

#[test]
#[ignore]
fn test_boot_rom() {
    const INSTR_DEBUG: bool = false;
    const SLOW: bool = false;
    const WAIT_MS: u64 = 10;

    let mut cpu = Cpu::new_boot_cart("../roms/test_mbc1.gb", "../dmg-boot.bin");
    println!("{}", cpu.mmu.cart.header_info());
    println!(
        "Instruction Debug: {}, Slow: {}",
        INSTR_DEBUG,
        if SLOW {
            format!("{} ms", WAIT_MS)
        } else {
            "false".to_owned()
        },
    );
    println!("Enter any text to continue...");
    let _: String = read!();
    println!("-------");

    let mut t_cycles = 0;
    'testloop: loop {
        let t_start = Instant::now();
        t_cycles += cpu.cycle(INSTR_DEBUG);

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
