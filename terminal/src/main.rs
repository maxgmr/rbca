use std::{env, fs::File, io::Read};

use rbca_core::*;

fn main() {
    // TODO temporary test ROM
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run path/to/gb/rom");
        return;
    }

    let mut cpu = Cpu::new();

    let mut rom = File::open(&args[1]).expect("Unable to open file.");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    cpu.execute(&buffer);
}
