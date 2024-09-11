use std::env;

use color_eyre::eyre::{self, eyre};
use rbca_core::Cpu;
use text_io::read;

mod display;

use display::Display;

const INSTR_DEBUG: bool = true;
const USE_BOOT_ROM: bool = false;

fn main() -> eyre::Result<()> {
    let args: Vec<_> = env::args().collect();

    if USE_BOOT_ROM && args.len() != 3 {
        return Err(eyre!(
            "Incorrect usage.\r\nUsage: cargo r path/to/boot/rom path/to/gb/rom"
        ));
    } else if !USE_BOOT_ROM && args.len() != 2 {
        return Err(eyre!("Incorrect usage.\r\nUsage: cargo r path/to/gb/rom"));
    }

    // Load ROM
    let cpu = if USE_BOOT_ROM {
        Cpu::new_boot_cart(&args[2], &args[1])
    } else {
        Cpu::new_cart(&args[1])
    };

    println!("{}", cpu.emu_info());
    println!("Enter any text to start...");
    let _: String = read!();
    println!("-------");

    // Create desktop UI
    let mut desktop = Display::new(cpu)?;

    // Run desktop UI
    desktop.run()?;

    Ok(())
}
