use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::collections::HashMap;

pub enum Action {
    Quit,

    Left,
    Right,
    Up,
    Down,

    Enter,
    Exit,

    TimerStart,
    TimerStartRelease,
    TimerStop,
}

#[derive(PartialEq, Eq, Hash)]
pub struct KeyBind {
    pub key: KeyCode,
    pub kind: KeyEventKind,
    pub modifiers: KeyModifiers,
}

impl KeyBind {
    fn new(key: KeyCode) -> Self {
        Self {
            key,
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::empty(),
        }
    }

    fn with_kind(key: KeyCode, kind: KeyEventKind) -> Self {
        Self {
            key,
            kind,
            modifiers: KeyModifiers::empty(),
        }
    }

    fn with_modifiers(key: KeyCode, modifiers: KeyModifiers) -> Self {
        Self {
            key,
            kind: KeyEventKind::Press,
            modifiers,
        }
    }
}

impl From<KeyCode> for KeyBind {
    fn from(key: KeyCode) -> Self {
        Self::new(key)
    }
}

impl From<KeyEvent> for KeyBind {
    fn from(key_event: KeyEvent) -> Self {
        Self {
            key: key_event.code,
            kind: key_event.kind,
            modifiers: key_event.modifiers,
        }
    }
}

// A mapping of all KeyBind -> Action pairs
// Will be used in the future with the config to allow editable keybinds
pub struct ActionMap {
    keybinds: HashMap<KeyBind, Action>,
}

impl ActionMap {
    pub fn action(&self, key_event: KeyEvent) -> Option<&Action> {
        self.keybinds.get(&key_event.into())
    }
}

// Default keybinds
impl Default for ActionMap {
    fn default() -> Self {
        Self {
            keybinds: HashMap::from([
                (KeyCode::Char('q').into(), Action::Quit),
                (KeyCode::Left.into(), Action::Left),
                (KeyCode::Right.into(), Action::Right),
                (KeyCode::Up.into(), Action::Up),
                (KeyCode::Down.into(), Action::Down),
                (KeyCode::Char(' ').into(), Action::TimerStart),
                (
                    KeyBind::with_kind(KeyCode::Char(' '), KeyEventKind::Release),
                    Action::TimerStartRelease,
                ),
                (KeyCode::Char(' ').into(), Action::TimerStop),
            ]),
        }
    }
}
