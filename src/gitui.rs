use crate::input_config::{InputAction, InputConfig};
use crate::repo_state::RepoState;
use crate::ui::Ui;
use crossterm::event::{self, Event, KeyEventKind};

#[derive(Default)]
pub struct Gitui {
    repo_state: RepoState,
    ui: Ui,
    input_config: InputConfig,
    quit: bool,
}

impl Gitui {
    pub fn init() -> Self {
        Gitui::default()
    }

    pub fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
    ) -> Result<(), anyhow::Error>
    where
        <B as ratatui::backend::Backend>::Error: 'static + Send + Sync,
    {
        while !self.quit {
            terminal.draw(|frame| self.ui.draw(frame, &self.repo_state))?;

            // handle key events
            if let Event::Key(event) = event::read()? {
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
