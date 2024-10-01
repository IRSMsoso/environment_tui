use crate::config::{get_config, Config};
use crate::terminal::{init_terminal, restore_terminal};
use anyhow::Context;
use hodaun::{default_output, OutputDeviceMixer, Stereo};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use std::path::PathBuf;

fn run_app() -> anyhow::Result<()> {
    let mut terminal = init_terminal().context("Failed to initialize terminal")?;


    restore_terminal(terminal).context("Failed to initialize terminal")?;
    Ok(())
}


struct App {
    sounds: Vec<Loop>,
    config: Config,
}

impl App {
    fn new() -> anyhow::Result<App> {

        // TODO: This needs to be fixed. Having a hardcoded values is no good.
        let mut output: OutputDeviceMixer<Stereo> = default_output()?;

        Ok(App {
            sounds: Vec::new(),
            config: get_config()?,
        })
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}

struct Loop {
    filename: PathBuf,

}

impl Loop {
    fn get_friendly_name(&self) -> String {
        self.filename.with_extension("").to_string_lossy().to_string()
    }
}
