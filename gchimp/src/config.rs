//! Parses config file

// TODO move this whole thing out of GUI because CLI can benefit from this as well
use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};

use std::env;

use eyre::eyre;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub studiomdl: String,
    pub crowbar: String,
    #[cfg(target_os = "linux")]
    pub wineprefix: Option<String>,
    pub theme: String,
}

pub static CONFIG_FILE_NAME: &str = "config.toml";

/// Parse `config.toml` in the same folder as the binary
pub fn parse_config() -> eyre::Result<Config> {
    let path = match env::current_exe() {
        Ok(path) => path.parent().unwrap().join(CONFIG_FILE_NAME),
        Err(_) => PathBuf::from(CONFIG_FILE_NAME),
    };

    if !path.exists() {
        return Err(eyre!(
            "Cannot find config.toml in the same folder as gchimp"
        ));
    }

    parse_config_from_file(path.as_path())
}

pub fn parse_config_from_file(path: &Path) -> eyre::Result<Config> {
    let mut file = OpenOptions::new().read(true).open(path.as_os_str())?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let config: Config = toml::from_str(&buffer)?;

    let root = path.parent().unwrap();

    let studiomdl = PathBuf::from(config.studiomdl);
    let studiomdl = if studiomdl.is_relative() {
        root.join(studiomdl)
    } else {
        studiomdl
    }
    .canonicalize();

    if studiomdl.is_err() {
        return Err(eyre!("Cannot find studiomdl binary"));
    }

    let studiomdl = studiomdl.unwrap().display().to_string();

    let crowbar = PathBuf::from(config.crowbar);
    let crowbar = if crowbar.is_relative() {
        root.join(crowbar)
    } else {
        crowbar
    }
    .canonicalize();

    if crowbar.is_err() {
        return Err(eyre!("Cannot find crowbar binary"));
    }

    let crowbar = crowbar.unwrap().display().to_string();

    Ok(Config {
        studiomdl,
        crowbar,
        #[cfg(target_os = "linux")]
        wineprefix: config.wineprefix,
        theme: config.theme,
    })
}
