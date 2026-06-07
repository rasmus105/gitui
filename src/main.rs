use anyhow::Result;

mod gitui;
mod input_config;
mod repo_state;
mod ui;

use gitui::Gitui;

fn main() -> Result<()> {
    let mut gitui = Gitui::init();

    ratatui::run(|terminal| gitui.run(terminal))?;

    Ok(())
}
