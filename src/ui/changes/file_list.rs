use super::super::common;
use crate::input_config::InputAction;
use crate::repo_state::FileChange;
use crate::ui::file_display;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
};

#[derive(Default)]
pub struct FileList {
    list_state: ListState,
}

impl FileList {
    fn sync(&mut self, file_count: usize) {
        if file_count == 0 {
            self.list_state.select(None);
            return;
        }

        let selected = self.list_state.selected().unwrap_or(0).min(file_count - 1);
        self.list_state.select(Some(selected));
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, files: &[FileChange], focused: bool) {
        self.sync(files.len());

        let block = common::block("Files", focused);

        if files.is_empty() {
            frame.render_widget(List::new([ListItem::new("No changes")]).block(block), area);
            return;
        }

        let items: Vec<_> = files
            .iter()
            .map(|file| ListItem::new(file_display::file_change_line(file)))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    pub fn handle_action(&mut self, action: InputAction) -> bool {
        match action {
            InputAction::Up => {
                let selected = self.list_state.selected().unwrap_or(0).saturating_sub(1);
                self.list_state.select(Some(selected));
                true
            }
            InputAction::Down => {
                let selected = self.list_state.selected().unwrap_or(0).saturating_add(1);
                self.list_state.select(Some(selected));
                true
            }
            _ => false,
        }
    }
}
