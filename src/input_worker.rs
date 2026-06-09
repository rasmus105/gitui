use crossbeam::channel::{Receiver, unbounded};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

pub struct InputWorker {
    events_rx: Receiver<KeyEvent>,
}

impl InputWorker {
    pub fn spawn() -> Self {
        let (tx, rx) = unbounded();

        std::thread::Builder::new()
            .name("gitui-input".into())
            .spawn(move || {
                loop {
                    match event::read() {
                        Ok(Event::Key(key))
                            if key.kind == KeyEventKind::Press && tx.send(key).is_err() =>
                        {
                            break;
                        }
                        Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {}
                        Err(_) => break,
                        _ => {}
                    }
                }
            })
            .expect("failed to spawn input thread");

        Self { events_rx: rx }
    }

    pub fn drain(&self) -> impl Iterator<Item = KeyEvent> + '_ {
        std::iter::from_fn(|| self.events_rx.try_recv().ok())
    }
}
