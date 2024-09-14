use std::default::Default;

use camino::Utf8PathBuf;
use color_eyre::eyre::{self, eyre};
use config::{Config, File};
use sdl2::keyboard::Scancode;
use serde::Deserialize;

use crate::{
    palette::{Palette, PresetPalette},
    scancodes, utils,
};

const DEFAULT_CONFIG_DIR: &str = "~/.config/rbca_desktop/";

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct KeyBindings {
    #[serde(with = "scancodes")]
    pub up: Scancode,
    #[serde(with = "scancodes")]
    pub down: Scancode,
    #[serde(with = "scancodes")]
    pub left: Scancode,
    #[serde(with = "scancodes")]
    pub right: Scancode,
    #[serde(with = "scancodes")]
    pub a: Scancode,
    #[serde(with = "scancodes")]
    pub b: Scancode,
    #[serde(with = "scancodes")]
    pub start: Scancode,
    #[serde(with = "scancodes")]
    pub select: Scancode,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct PathSettings {
    pub boot_rom_path: Option<Utf8PathBuf>,
    pub saves_dir: Utf8PathBuf,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct PaletteSettings {
    pub preset_palette: PresetPalette,
    pub custom_palette: CustomPalette,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct CustomPalette {
    pub enabled: bool,
    pub palette: Palette,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct DebugSettings {
    pub general_debug: bool,
    pub instr_debug: bool,
    pub btn_debug: bool,
    pub fps_debug: u32,
    pub config_debug: bool,
    pub breakpoints: bool,
    pub history: usize,
    pub continue_key: char,
    pub step_forward_key: char,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Breakpoints {
    pub program_counter: Vec<u16>,
    pub instr_name: Vec<String>,
    pub opcode_1_byte: Vec<u8>,
    pub opcode_2_byte: Vec<u16>,
    pub opcode_3_byte: Vec<u32>,
    pub a_reg: Vec<u8>,
    pub b_reg: Vec<u8>,
    pub c_reg: Vec<u8>,
    pub d_reg: Vec<u8>,
    pub e_reg: Vec<u8>,
    pub h_reg: Vec<u8>,
    pub l_reg: Vec<u8>,
}

/// The user configuration settings.
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct UserConfig {
    pub key_bindings: KeyBindings,
    pub path_settings: PathSettings,
    pub palette_settings: PaletteSettings,
    pub debug_settings: DebugSettings,
    pub breakpoints: Breakpoints,
}
impl UserConfig {
    /// Load a new [UserConfig], overwriting default values with any custom-set values.
    pub fn new() -> eyre::Result<Self> {
        let config_dir = match utils::config_dir() {
            Ok(config_dir) => config_dir,
            Err(_) => {
                eprintln!("Warning: failed to load configuration directory. Using default.");
                utils::expand_path(Utf8PathBuf::from(DEFAULT_CONFIG_DIR))?
            }
        };

        let user_config = Config::builder()
            .add_source(
                File::with_name(
                    [&config_dir, &"default".into()]
                        .iter()
                        .collect::<Utf8PathBuf>()
                        .as_str(),
                )
                .required(true),
            )
            // Add in the local configuration file.
            .add_source(
                File::with_name(
                    [&config_dir, &"config".into()]
                        .iter()
                        .collect::<Utf8PathBuf>()
                        .as_str(),
                )
                .required(false),
            )
            .build()?;

        match user_config.try_deserialize::<UserConfig>() {
            Ok(mut result) => {
                result.expand_file_paths()?;
                Ok(result)
            }
            Err(e) => Err(eyre!("{e}")),
        }
    }

    /// Directly access the "Up" scancode.
    pub fn up_code(&self) -> &Scancode {
        &self.key_bindings.up
    }

    /// Directly access the "Down" scancode.
    pub fn down_code(&self) -> &Scancode {
        &self.key_bindings.down
    }

    /// Directly access the "Left" scancode.
    pub fn left_code(&self) -> &Scancode {
        &self.key_bindings.left
    }

    /// Directly access the "Right" scancode.
    pub fn right_code(&self) -> &Scancode {
        &self.key_bindings.right
    }

    /// Directly access the "A" scancode.
    pub fn a_code(&self) -> &Scancode {
        &self.key_bindings.a
    }

    /// Directly access the "B" scancode.
    pub fn b_code(&self) -> &Scancode {
        &self.key_bindings.b
    }

    /// Directly access the "Start" scancode.
    pub fn start_code(&self) -> &Scancode {
        &self.key_bindings.start
    }

    /// Directly access the "Select" scancode.
    pub fn select_code(&self) -> &Scancode {
        &self.key_bindings.select
    }

    /// Directly access the boot ROM path.
    pub fn boot_rom_path(&self) -> &Option<Utf8PathBuf> {
        &self.path_settings.boot_rom_path
    }

    /// Directly access the saves directory.
    pub fn saves_dir(&self) -> &Utf8PathBuf {
        &self.path_settings.saves_dir
    }

    /// Directly access the selected [Palette].
    pub fn palette(&self) -> &Palette {
        if self.palette_settings.custom_palette.enabled {
            &self.palette_settings.custom_palette.palette
        } else {
            self.palette_settings.preset_palette.get()
        }
    }

    /// Directly access general_debug.
    pub fn general_debug(&self) -> bool {
        self.debug_settings.general_debug
    }

    /// Directly access instr_debug.
    pub fn instr_debug(&self) -> bool {
        self.debug_settings.instr_debug
    }

    /// Directly access btn_debug.
    pub fn btn_debug(&self) -> bool {
        self.debug_settings.btn_debug
    }

    /// Directly access fps_debug.
    pub fn fps_debug(&self) -> u32 {
        self.debug_settings.fps_debug
    }

    /// Directly access config_debug.
    pub fn config_debug(&self) -> bool {
        self.debug_settings.config_debug
    }

    /// Directly access the breakpoints boolean.
    pub fn breakpoints_enabled(&self) -> bool {
        self.debug_settings.breakpoints
    }

    /// Directly access the history size.
    pub fn history(&self) -> usize {
        self.debug_settings.history
    }

    /// Directly access the continue key.
    pub fn continue_key(&self) -> &char {
        &self.debug_settings.continue_key
    }

    /// Directly access the step forward key.
    pub fn step_forward_key(&self) -> &char {
        &self.debug_settings.step_forward_key
    }

    /// Directly access the breakpoints.
    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    // Expand any given file paths.
    fn expand_file_paths(&mut self) -> eyre::Result<()> {
        if let Some(boot_rom_path) = &self.path_settings.boot_rom_path {
            self.path_settings.boot_rom_path = Some(utils::expand_path(boot_rom_path)?);
        }
        self.path_settings.saves_dir = utils::expand_path(&self.path_settings.saves_dir)?;
        Ok(())
    }
}
