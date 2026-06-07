use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    // --- Generic Actions (implemented by multiple components) ---
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Quit,
    // --- Special Actions (implemented by a single components) ---
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Shortcut {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Shortcut {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    pub const fn char(char: char) -> Self {
        Self {
            code: KeyCode::Char(char),
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn matches(self, event: KeyEvent) -> bool {
        self.code == event.code && self.modifiers == event.modifiers
    }
}

#[derive(Debug, Clone)]
pub struct InputConfig {
    bindings: Vec<(Shortcut, InputAction)>,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            bindings: vec![
                (Shortcut::char('k'), InputAction::Up),
                (
                    Shortcut::new(KeyCode::Up, KeyModifiers::NONE),
                    InputAction::Up,
                ),
                (Shortcut::char('j'), InputAction::Down),
                (
                    Shortcut::new(KeyCode::Down, KeyModifiers::NONE),
                    InputAction::Down,
                ),
                (Shortcut::char('h'), InputAction::Left),
                (
                    Shortcut::new(KeyCode::Left, KeyModifiers::NONE),
                    InputAction::Left,
                ),
                (Shortcut::char('l'), InputAction::Right),
                (
                    Shortcut::new(KeyCode::Right, KeyModifiers::NONE),
                    InputAction::Right,
                ),
                (
                    Shortcut::new(KeyCode::Enter, KeyModifiers::NONE),
                    InputAction::Select,
                ),
                (
                    Shortcut::new(KeyCode::Esc, KeyModifiers::NONE),
                    InputAction::Back,
                ),
                (Shortcut::char('q'), InputAction::Quit),
            ],
        }
    }
}

impl InputConfig {
    pub fn action_for(&self, event: KeyEvent) -> Option<InputAction> {
        self.bindings
            .iter()
            .find_map(|(shortcut, action)| shortcut.matches(event).then_some(*action))
    }
}
