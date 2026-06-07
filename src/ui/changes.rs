mod file_list;

use crate::input_config::InputAction;
use crate::repo_state::RepoState;
use ratatui::{Frame, layout::Rect};

pub use file_list::FileList;

#[derive(Default)]
enum Focus {
    #[default]
    FileList,
}

#[derive(Default)]
pub struct Changes {
    focus: Focus,
    file_list: FileList,
}

impl Changes {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &RepoState) {
        match self.focus {
            Focus::FileList => self.file_list.draw(frame, area, &repo.changed_files),
        }
    }

    pub fn handle_action(&mut self, action: InputAction) -> bool {
        self.file_list.handle_action(action)
    }
}
