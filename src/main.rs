use crate::cli::{Cli, Commands};
use crate::config::{get_config, save_config};
use anyhow::Context;
use clap::Parser;
use normalize_path::NormalizePath;
use std::env::current_dir;
use std::path::PathBuf;

mod terminal;
mod config;
mod cli;
mod app;

fn resolve_path(mut path: PathBuf) -> std::io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        path = current_dir()?.join(path);
        path = path.normalize();
        Ok(path)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {}
        Some(command) => match command {
            Commands::SetDirectory { directory } => {
                let mut config = get_config()?;
                config.sound_directory = Some(resolve_path(directory)?);
                save_config(&config)?;
            }
        }
    }

    Ok(())
}
