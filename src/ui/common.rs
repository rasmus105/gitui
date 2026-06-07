use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub(crate) fn block(title: &'static str, focused: bool) -> Block<'static> {
    let border_style = if focused {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    Block::new()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style)
}
