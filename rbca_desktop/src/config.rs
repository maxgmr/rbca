use std::default::Default;

use camino::Utf8PathBuf;
use color_eyre::eyre::{self, eyre};
use config::{Config, File};

use crate::{
    palette::{Palette, PresetPalette},
    utils,
};

const DEFAULT_CONFIG_DIR: &str = "~/.config/rbca_desktop/";

#[derive(Debug, Default, serde::Deserialize)]
#[allow(unused)]
pub struct PathSettings {
    pub boot_rom_path: Option<Utf8PathBuf>,
    pub saves_dir: Utf8PathBuf,
}

#[derive(Debug, Default, serde::Deserialize)]
#[allow(unused)]
pub struct PaletteSettings {
    pub preset_palette: PresetPalette,
    pub custom_palette: CustomPalette,
}

#[derive(Debug, Default, serde::Deserialize)]
#[allow(unused)]
pub struct CustomPalette {
    pub enabled: bool,
    pub palette: Palette,
}

#[derive(Debug, Default, serde::Deserialize)]
#[allow(unused)]
pub struct DebugSettings {
    pub general_debug: bool,
    pub instr_debug: bool,
    pub btn_debug: bool,
    pub fps_debug: u32,
    pub config_debug: bool,
}

/// The user configuration settings.
#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
pub struct UserConfig {
    pub path_settings: PathSettings,
    pub palette_settings: PaletteSettings,
    pub debug_settings: DebugSettings,
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

    // Expand any given file paths.
    fn expand_file_paths(&mut self) -> eyre::Result<()> {
        if let Some(boot_rom_path) = &self.path_settings.boot_rom_path {
            self.path_settings.boot_rom_path = Some(utils::expand_path(boot_rom_path)?);
        }
        self.path_settings.saves_dir = utils::expand_path(&self.path_settings.saves_dir)?;
        Ok(())
    }
}
