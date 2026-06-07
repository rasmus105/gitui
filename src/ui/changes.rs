mod diffs;
mod file_list;

use crate::input_config::InputAction;
use crate::repo_state::RepoState;
use diffs::DiffView;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use file_list::FileList;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Focus {
    #[default]
    FileList,
    Diff,
}

#[derive(Default)]
pub struct Changes {
    focus: Focus,
    file_list: FileList,
    diff_view: DiffView,
}

impl Changes {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &RepoState) {
        let [files_area, diff_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);

        self.file_list.draw(
            frame,
            files_area,
            &repo.file_changes,
            self.focus == Focus::FileList,
        );
        self.diff_view
            .draw(frame, diff_area, self.focus == Focus::Diff);
    }

    pub fn handle_action(&mut self, action: InputAction) -> bool {
        self.file_list.handle_action(action)
    }
}
