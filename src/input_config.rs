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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Binding {
    action: InputAction,
    shortcuts: Vec<Shortcut>,
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
    bindings: Vec<Binding>,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            bindings: vec![
                Binding::new(
                    InputAction::Up,
                    vec![
                        Shortcut::char('k'),
                        Shortcut::new(KeyCode::Up, KeyModifiers::NONE),
                    ],
                ),
                Binding::new(
                    InputAction::Down,
                    vec![
                        Shortcut::char('j'),
                        Shortcut::new(KeyCode::Down, KeyModifiers::NONE),
                    ],
                ),
                Binding::new(
                    InputAction::Left,
                    vec![
                        Shortcut::char('h'),
                        Shortcut::new(KeyCode::Left, KeyModifiers::NONE),
                    ],
                ),
                Binding::new(
                    InputAction::Right,
                    vec![
                        Shortcut::char('l'),
                        Shortcut::new(KeyCode::Right, KeyModifiers::NONE),
                    ],
                ),
                Binding::new(
                    InputAction::Select,
                    vec![Shortcut::new(KeyCode::Enter, KeyModifiers::NONE)],
                ),
                Binding::new(
                    InputAction::Back,
                    vec![Shortcut::new(KeyCode::Esc, KeyModifiers::NONE)],
                ),
                Binding::new(InputAction::Quit, vec![Shortcut::char('q')]),
            ],
        }
    }
}

impl Binding {
    fn new(action: InputAction, shortcuts: Vec<Shortcut>) -> Self {
        Self { action, shortcuts }
    }
}

impl InputConfig {
    pub fn action_for(&self, event: KeyEvent) -> Option<InputAction> {
        self.bindings.iter().find_map(|binding| {
            binding
                .shortcuts
                .iter()
                .any(|shortcut| shortcut.matches(event))
                .then_some(binding.action)
        })
    }
}
