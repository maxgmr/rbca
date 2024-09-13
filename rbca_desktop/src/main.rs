use clap::Parser;
use color_eyre::eyre;
use rbca_core::Cpu;
use text_io::read;

mod arg_parser;
mod config;
mod emulator;
mod palette;
mod scancodes;
mod utils;

use arg_parser::Args;
use config::UserConfig;
use emulator::Emulator;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // Load args
    let args = Args::parse();

    // Load config & set up file dirs
    let config: UserConfig = utils::setup()?;

    // Load ROM
    let cpu = match (config.boot_rom_path(), &args.rom_path) {
        (Some(boot_path), Some(rom_path)) => Cpu::new_boot_cart(rom_path, boot_path),
        (Some(boot_path), None) => Cpu::new_boot(boot_path),
        (None, Some(rom_path)) => Cpu::new_cart(rom_path),
        (None, None) => Cpu::new(),
    };

    if config.config_debug() {
        // Pretty print the config
        println!("RBCA CONFIG");
        println!("{:#?}", config);
        println!("-------");
    }

    if config.general_debug() {
        // Display cart info & wait to start
        println!("{}", cpu.emu_info());
        println!("Enter any text to start...");
        let _: String = read!();
        println!("-------");
    }

    // Create desktop emulator
    let mut desktop = Emulator::new(cpu, &config)?;

    // Run desktop emulator
    desktop.run()?;

    Ok(())
}
