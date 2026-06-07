use crate::input_config::InputAction;
use crate::repo_state::ChangedFile;
use ratatui::{Frame, layout::Rect};

#[derive(Default)]
pub struct FileList {
    file_idx: usize,
}

impl FileList {
    pub fn draw(&mut self, _frame: &mut Frame, _area: Rect, _files: &[ChangedFile]) {}

    pub fn handle_action(&mut self, action: InputAction) -> bool {
        match action {
            InputAction::Up => {
                self.file_idx = self.file_idx.saturating_sub(1);
                true
            }
            InputAction::Down => {
                self.file_idx += 1;
                true
            }
            _ => false,
        }
    }
}
