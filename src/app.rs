use crate::r#async::{GitEvent, GitOperation, GitWorker};
use crate::input_config::{InputAction, InputConfig};
use crate::input_worker::InputWorker;
use crate::state::{Loadable, State};
use crate::ui::Ui;

pub struct App {
    state: State,
    ui: Ui,
    input_config: InputConfig,
    async_git: GitWorker,
    input: InputWorker,
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
            input: InputWorker::spawn(),
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
            self.handle_input_events();
            terminal.draw(|frame| self.ui.draw(frame, &self.state))?;

            // TODO: use either deltatime calculation or tick to keep more consistent updating rate.
            std::thread::sleep(std::time::Duration::from_millis(16));
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

    fn handle_input_events(&mut self) {
        let keys: Vec<_> = self.input.drain().collect();
        for key in keys {
            match self.input_config.action_for(key) {
                Some(action) => self.handle_action(action),
                None => self.ui.set_message("unmapped key"),
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
