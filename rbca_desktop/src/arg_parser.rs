//! Parse command-line arguments.
use camino::Utf8PathBuf;
use clap::Parser;

use crate::utils;

#[derive(Parser, Debug)]
#[command(name = "rbca")]
#[command(author)]
#[command(version = utils::info())]
#[command(about = "Game Boy emulator.")]
pub struct Args {
    /// Path to ROM.
    ///
    /// If no ROM is provided, it will boot into the boot ROM. If no boot ROM is provided, it will
    /// still boot but do nothing.
    pub rom_path: Option<Utf8PathBuf>,
}
