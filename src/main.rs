use ratatui::{DefaultTerminal, Frame};

struct App {
    repo: gix::Repository,
}

impl App {
    pub fn new(repo: gix::Repository) -> Self {
        App { repo }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let repo = gix::discover(".")?;
    let a = App::new(repo);

    ratatui::run(|terminal| app(terminal, a))?;

    Ok(())
}

fn app(terminal: &mut DefaultTerminal, mut app: App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, app: &mut App) {
    let text = format!("Git repo: {}\n", app.repo.path().display(),);
    frame.render_widget(text, frame.area());
}
