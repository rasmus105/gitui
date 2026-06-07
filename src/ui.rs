mod changes;
mod common;
mod file_display;
mod log;
mod refs;

use crate::input_config::InputAction;
use crate::repo_state::RepoState;
use changes::Changes;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

#[derive(Default)]
enum Tab {
    /// [1] Repository summary, current HEAD, status counts, and key metadata.
    Overview,

    /// [2] Worktree/index changes, file list, and diffs.
    #[default]
    Changes,

    /// [3] Branches, remotes, tags, stashes, worktrees, and submodules.
    Refs,

    /// [4] Commit history, reflog, and commit details.
    Log,
}

#[derive(Default)]
pub struct Ui {
    tab: Tab,
    message: Option<String>,
    // ---  UI components ---
    changes: Changes,
}

impl Ui {
    pub fn draw(&mut self, frame: &mut Frame, repo: &RepoState) {
        let (content_area, message_area) = self.layout(frame.area());

        match self.tab {
            Tab::Changes => self.changes.draw(frame, content_area, repo),
            Tab::Refs => {}
            Tab::Log => {}
            Tab::Overview => {}
        }

        // Render bottom status bar.
        if let Some(message) = &self.message {
            frame.render_widget(Paragraph::new(message.as_str()), message_area);
        }
    }

    pub fn handle_action(&mut self, action: InputAction) -> bool {
        match self.tab {
            Tab::Changes => self.changes.handle_action(action),
            Tab::Refs | Tab::Log | Tab::Overview => false,
        }
    }

    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
    }

    fn layout(&self, area: Rect) -> (Rect, Rect) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        (areas[0], areas[1])
    }
}
