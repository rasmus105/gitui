use super::super::common;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

#[derive(Default)]
pub struct DiffView;

impl DiffView {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        frame.render_widget(
            Paragraph::new("TODO diff").block(common::block("Diff", focused)),
            area,
        );
    }
}
