use anyhow::Context;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) presets: Vec<ConfigPreset>,
    pub(crate) sound_directory: Option<PathBuf>,
}

impl Config {
    pub(crate) fn new() -> Config {
        Config {
            presets: Vec::new(),
            sound_directory: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigPreset {
    loops: Vec<ConfigLoop>,
}

#[derive(Serialize, Deserialize)]
struct ConfigLoop {
    file_name: String,
    volume_multiplier: f64,
}

/// Gets a PathBuf to config file. Creates directory there if needed.
fn get_config_path() -> anyhow::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "irsmsoso", "environment_tui").context("Failed to find project directories")?;

    if !project_dirs.config_dir().is_dir() {
        create_dir_all(project_dirs.config_dir()).context("Failed to create directory for config")?;
    }

    Ok(project_dirs.config_dir().join("config.json"))
}

pub(crate) fn get_config() -> anyhow::Result<Config> {
    let config_path = get_config_path()?;

    if !config_path.is_file() {
        let config = Config::new();
        let file = File::create(config_path).context("Failed to open config file")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &config).context("Failed to serialize to file")?;
        return Ok(config);
    }

    let file = File::open(config_path).context("Failed to open config file")?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).context("Failed to read config file")?)
}

pub(crate) fn save_config(config: &Config) -> anyhow::Result<()> {
    let config_path = get_config_path()?;

    let file = File::create(config_path).context("Failed to open config file")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, config).context("Failed to serialize to file")?;

    Ok(())
}