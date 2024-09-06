use std::env;

use color_eyre::eyre::{self, eyre};
use rbca_core::Cpu;

mod display;

use display::Display;

fn main() -> eyre::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        return Err(eyre!("Incorrect usage.\r\nUsage: cargo r path/to/gb/rom"));
    }

    // Load ROM
    let mut cpu = Cpu::new();
    cpu.mem_bus.load_cart(&args[1], true);

    // Create desktop UI
    let mut desktop = Display::new()?;

    // Run desktop UI
    desktop.run()?;

    Ok(())
}
