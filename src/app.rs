use crate::r#async::{GitEvent, GitOperation, GitWorker};
use crate::input_config::{InputAction, InputConfig};
use crate::state::{Loadable, State};
use crate::ui::Ui;
use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

pub struct App {
    state: State,
    ui: Ui,
    input_config: InputConfig,
    async_git: GitWorker,
    quit: bool,
}

impl App {
    pub fn init() -> anyhow::Result<Self> {
        let repo_path = std::env::current_dir()?;
        let async_git = GitWorker::spawn(repo_path)?;
        async_git.request(GitOperation::Status);

        Ok(App {
            state: State::default(),
            ui: Ui::default(),
            input_config: InputConfig::default(),
            async_git,
            quit: false,
        })
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
    ) -> Result<(), anyhow::Error>
    where
        <B as ratatui::backend::Backend>::Error: 'static + Send + Sync,
    {
        while !self.quit {
            self.handle_git_events();
            terminal.draw(|frame| self.ui.draw(frame, &self.state))?;

            if event::poll(Duration::from_millis(50))?
                && let Event::Key(event) = event::read()?
            {
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                match self.input_config.action_for(event) {
                    Some(action) => self.handle_action(action),
                    None => self.ui.set_message("unmapped key"),
                }
            }
        }

        Ok(())
    }

    fn handle_git_events(&mut self) {
        for event in self.async_git.drain() {
            match event {
                GitEvent::Status(status) => {
                    self.ui
                        .set_message(format!("{} changed", status.files_changed.len()));
                    self.state.status = Loadable::Loaded(status);
                }
                GitEvent::Failed { operation, error } => {
                    self.ui.set_message(format!("git error: {error}"));
                    match operation {
                        GitOperation::Status => self.state.status = Loadable::Failed(error),
                    }
                }
            }
        }
    }

    fn handle_action(&mut self, action: InputAction) {
        match action {
            InputAction::Quit => self.quit = true,
            _ => {
                if !self.ui.handle_action(action) {
                    self.ui.set_message("invalid action");
                }
            }
        }
    }
}
