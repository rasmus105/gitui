use crate::repo_state::{FileChange, FileStatus};
use devicons::FileIcon;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub(crate) fn file_change_line(file: &FileChange) -> Line<'static> {
    let icon = FileIcon::from(&file.path);

    let mut spans = vec![
        status_span(file.index_status),
        status_span(file.worktree_status),
        Span::raw(" "),
        Span::styled(
            icon.icon.to_string(),
            Style::default().fg(hex_color(icon.color).unwrap_or(Color::Gray)),
        ),
        Span::raw(" "),
        Span::raw(file.path.display().to_string()),
    ];

    if let Some(old_path) = &file.old_path {
        spans.extend([
            Span::raw(" "),
            Span::styled("from", Style::default().fg(Color::DarkGray)),
            Span::raw(" "),
            Span::styled(
                old_path.display().to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        ]);
    }

    Line::from(spans)
}

fn status_span(status: Option<FileStatus>) -> Span<'static> {
    match status {
        Some(status) => Span::styled(
            status_short_label(status).to_string(),
            Style::default()
                .fg(status_color(status))
                .add_modifier(Modifier::BOLD),
        ),
        None => Span::raw(" "),
    }
}

fn status_short_label(status: FileStatus) -> char {
    match status {
        FileStatus::Added => 'A',
        FileStatus::Modified => 'M',
        FileStatus::Deleted => 'D',
        FileStatus::Renamed => 'R',
        FileStatus::Copied => 'C',
        FileStatus::TypeChanged => 'T',
        FileStatus::Untracked => '?',
        FileStatus::Conflicted => 'U',
    }
}

fn status_color(status: FileStatus) -> Color {
    match status {
        FileStatus::Added => Color::Green,
        FileStatus::Modified => Color::Yellow,
        FileStatus::Deleted => Color::Red,
        FileStatus::Renamed => Color::Cyan,
        FileStatus::Copied => Color::Cyan,
        FileStatus::TypeChanged => Color::Magenta,
        FileStatus::Untracked => Color::Blue,
        FileStatus::Conflicted => Color::Red,
    }
}

fn hex_color(hex: &str) -> Option<Color> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    if hex.len() != 6 {
        return None;
    }

    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::Rgb(red, green, blue))
}
