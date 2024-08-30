use std::{env, fs::File, io::Read};

use rbca_core::*;

fn main() {
    // TODO temporary test ROM
    let args: Vec<_> = env::args().collect();
    // if args.len() > 2 {
    //     eprintln!("Usage: cargo run path/to/gb/rom");
    //     return;
    // }
    //
    // let mut cpu = Cpu::new();
    // cpu.load(0x0000, &DMG_BOOT);
    //
    // let mut buffer = Vec::new();
    //
    // if args.len() == 2 {
    //     let mut rom = File::open(&args[1]).expect("Unable to open file.");
    //     rom.read_to_end(&mut buffer).unwrap();
    //     cpu.load((DMG_BOOT_SIZE as u16) + 1, &buffer);
    // }
    //
    // while (cpu.pc as usize) < (DMG_BOOT_SIZE + buffer.len()) {
    //     cpu.cycle();
    //     std::thread::sleep_ms(100);
    // }

    if args.len() != 2 {
        eprintln!("Usage: cargo r path/to/gb/rom");
        return;
    }

    let mut cpu = Cpu::new();

    let mut rom_buffer = Vec::new();
    let mut rom_file = File::open(&args[1]).expect("Unable to open file.");
    rom_file.read_to_end(&mut rom_buffer).unwrap();
    cpu.load(0x0000, &rom_buffer);
    cpu.pc = 0x0100;

    while (cpu.pc as usize) < (0x0100 + rom_buffer.len()) {
        cpu.cycle();
        if cpu.mem_bus.read_byte(0xFF02) == 0x81 {
            let c = cpu.mem_bus.read_byte(0xFF01) as char;
            // println!("{}", c);
            cpu.mem_bus.write_byte(0xFF02, 0x00);
        }
    }

    println!("done");
}
