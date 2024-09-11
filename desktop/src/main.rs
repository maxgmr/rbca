use std::env;

use color_eyre::eyre::{self, eyre};
use rbca_core::Cpu;
use sdl2::keyboard::Keycode;
use text_io::read;

mod emulator;

use emulator::Emulator;

const INSTR_DEBUG: bool = false;
const BTN_DEBUG: bool = false;
const USE_BOOT_ROM: bool = true;

const BTN_UP: Keycode = Keycode::W;
const BTN_DOWN: Keycode = Keycode::S;
const BTN_LEFT: Keycode = Keycode::A;
const BTN_RIGHT: Keycode = Keycode::D;
const BTN_A: Keycode = Keycode::Comma;
const BTN_B: Keycode = Keycode::Period;
const BTN_START: Keycode = Keycode::Return;
const BTN_SELECT: Keycode = Keycode::Backspace;

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

    // Create desktop emulator
    let mut desktop = Emulator::new(cpu)?;

    // Run desktop emulator
    desktop.run()?;

    Ok(())
}
