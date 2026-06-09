mod diffs;
mod file_list;

use crate::input_config::InputAction;
use crate::state::{Loadable, State};
use diffs::DiffView;
use file_list::FileList;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

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
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, state: &State) {
        let [files_area, diff_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);

        let files = match &state.status {
            Loadable::Loaded(status) => status.files_changed.as_slice(),
            _ => &[],
        };

        self.file_list.draw(frame, files_area, files, self.focus == Focus::FileList);
        self.diff_view.draw(frame, diff_area, self.focus == Focus::Diff);
    }

    pub fn handle_action(&mut self, action: InputAction) -> bool {
        self.file_list.handle_action(action)
    }
}
