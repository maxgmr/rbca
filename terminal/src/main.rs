use std::env;

use rbca_core::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo r path/to/gb/rom");
        return;
    }

    let mut cpu = Cpu::new();
    cpu.mem_bus.load_rom(&args[1]);

    println!("done");
}
