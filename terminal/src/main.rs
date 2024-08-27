use std::{env, fs::File, io::Read};

use rbca_core::*;

fn main() {
    // TODO temporary test ROM
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: cargo run path/to/gb/rom");
        return;
    }

    let mut cpu = Cpu::new();
    cpu.load(0x0000, &DMG_BOOT);

    let mut buffer = Vec::new();

    if args.len() == 2 {
        let mut rom = File::open(&args[1]).expect("Unable to open file.");
        rom.read_to_end(&mut buffer).unwrap();
        cpu.load((DMG_BOOT_SIZE as u16) + 1, &buffer);
    }

    while (cpu.pc as usize) < (DMG_BOOT_SIZE + buffer.len()) {
        cpu.cycle();
    }
}
