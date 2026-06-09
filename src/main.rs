use anyhow::Result;

mod app;
mod r#async;
mod git;
mod input_config;
mod input_worker;
mod state;
mod ui;

use app::App;

fn main() -> Result<()> {
    // TODO: setup panic handlers and stuff here

    let mut gitui = App::init()?;

    ratatui::run(|terminal| gitui.run(terminal))?;

    Ok(())
}
