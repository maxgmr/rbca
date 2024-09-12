//! General utilities used by the frontend.
use std::{env, fs};

use camino::Utf8PathBuf;
use color_eyre::eyre::{self, eyre};
use directories::ProjectDirs;

/// String displaying the package version, build date, & system OS version.
const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")",
    env!("VERGEN_SYSINFO_OS_VERSION"),
);

/// String displaying the total memory used on the system to run the build.
const TOTAL_MEMORY: &str = env!("VERGEN_SYSINFO_TOTAL_MEMORY");

/// Get the version, author info, and directories of the package.
pub fn info() -> String {
    let authors = clap::crate_authors!();
    format!(
        "{VERSION_MESSAGE}
Authors:\t\t\t{authors}
Configuration Directory:\t{}
Saves Directory:\t\t{}
Total Memory:\t\t{}",
        config_dir().unwrap(),
        saves_dir().unwrap(),
        TOTAL_MEMORY,
    )
}

/// Ensure directories are properly set up.
pub fn setup_dirs() -> eyre::Result<()> {
    // Create the directory where configuration data is stored if it doesn't already exist.
    if fs::metadata(config_dir()?).is_err() {
        fs::create_dir_all(config_dir()?)?;
    }

    // TODO load config

    // Create the directory where game saves are stored if it doesn't already exist.
    if fs::metadata(saves_dir()?).is_err() {
        fs::create_dir_all(saves_dir()?)?;
    }

    Ok(())
}

pub fn config_dir() -> eyre::Result<Utf8PathBuf> {
    if let Some(path) = get_env_var_path("CONFIG") {
        // Prioritise user-set path in env var.
        Ok(path)
    } else if let Some(proj_dirs) = project_directory() {
        // Next priority: XDG-standardised local directory.
        match Utf8PathBuf::from_path_buf(proj_dirs.config_local_dir().to_path_buf()) {
            Ok(utf8_path_buf) => Ok(utf8_path_buf),
            Err(_) => Err(eyre!(
                "Path to config directory contains non-UTF-8 content."
            )),
        }
    } else {
        // Last priority: .config folder relative to CWD
        Ok(Utf8PathBuf::from(".").join(".config"))
    }
}

pub fn saves_dir() -> eyre::Result<Utf8PathBuf> {
    if let Some(path) = get_env_var_path("SAVES") {
        // Prioritise user-set path in env var.
        Ok(path)
    // } else if {
    // Next priority: saves dir defined in config.
    // TODO
    } else if let Some(proj_dirs) = project_directory() {
        // Next priority: XDG-standardised local directory.
        match Utf8PathBuf::from_path_buf(proj_dirs.data_local_dir().to_path_buf()) {
            Ok(utf8_path_buf) => Ok(utf8_path_buf),
            Err(path_buf) => Err(eyre!(
                "Path to data directory {:?} contains non-UTF-8 content.",
                path_buf
            )),
        }
    } else {
        // Last priority: .saves folder relative to CWD
        Ok(Utf8PathBuf::from(".").join(".saves"))
    }
}

// Helper function to prepend the crate name to the env var.
fn get_env_var_path(suffix: &str) -> Option<Utf8PathBuf> {
    env::var(format!("{}_{}", pkg_name_constant_case(), suffix))
        .ok()
        .map(Utf8PathBuf::from)
}

/// Get the package name in CONSTANT_CASE.
pub fn pkg_name_constant_case() -> String {
    env!("CARGO_PKG_NAME").to_uppercase().to_string()
}

/// Get the directory of this project.
pub fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("ca", "maxgmr", env!("CARGO_PKG_NAME"))
}
