use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::collections::HashMap;

macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,

    Left,
    Right,
    Up,
    Down,

    Enter,
    Exit,

    TimerToggle,
    TimerStartRelease,
}

pub type RawKeyBindings = HashMap<Action, Vec<String>>;

/// Parses a single raw keybinding, returning its associated KeyEvent if valid.
/// * `binding`: Raw keybinding represented as a string
fn parse_keybind(binding: &str) -> Option<KeyEvent> {
    let mut code: Option<KeyCode> = None;
    let mut modifiers = KeyModifiers::NONE;
    let mut kind = KeyEventKind::Press;

    let binding = binding.trim();
    let binding = if let Some((prefix, rest)) = binding.split_once(':') {
        match prefix.to_lowercase().as_str() {
            "release" => kind = KeyEventKind::Release,
            "press" => kind = KeyEventKind::Press,
            _ => (),
        }

        rest.trim()
    } else {
        binding
    };

    let parts: Vec<&str> = binding.split('+').collect();

    // TODO: Log warnings when parts are not matched/invalid

    for part in parts {
        match part.trim().to_lowercase().as_str() {
            // Match modifiers
            "alt" => modifiers |= KeyModifiers::ALT,
            "ctrl" => modifiers |= KeyModifiers::CONTROL,
            "meta" => modifiers |= KeyModifiers::META,
            "shift" => modifiers |= KeyModifiers::SHIFT,

            // Match named keys
            "backspace" => code = Some(KeyCode::Backspace),
            "enter" => code = Some(KeyCode::Enter),
            "left" => code = Some(KeyCode::Left),
            "right" => code = Some(KeyCode::Right),
            "up" => code = Some(KeyCode::Up),
            "down" => code = Some(KeyCode::Down),
            "home" => code = Some(KeyCode::Home),
            "end" => code = Some(KeyCode::End),
            "pageup" | "page_up" => code = Some(KeyCode::PageUp),
            "pagedown" | "page_down" => code = Some(KeyCode::PageDown),
            "tab" => code = Some(KeyCode::Tab),
            "backtab" => code = Some(KeyCode::BackTab),
            "delete" => code = Some(KeyCode::Delete),
            "insert" => code = Some(KeyCode::Insert),
            "null" => code = Some(KeyCode::Null),
            "esc" | "escape" => code = Some(KeyCode::Esc),
            "pause" => code = Some(KeyCode::Pause),
            "menu" => code = Some(KeyCode::Menu),
            "space" => code = Some(KeyCode::Char(' ')),
            "none" => return None,

            // Match single characters, ex. 'c' or '?'
            part if part.len() == 1 => {
                code = Some(KeyCode::Char(part.chars().next().unwrap()));
            }

            // Match function keys
            part if part.starts_with('f') => {
                if let Ok(n) = part.parse::<u8>() {
                    code = Some(KeyCode::F(n));
                }
            }

            _ => return None,
        }
    }

    code.map(|code| KeyEvent::new_with_kind(code, modifiers, kind))
}

/// A bidirectional mapping of all (action, KeyEvent) pairs.
/// Will be used in the future with the config to allow editable KeyEvents.
pub struct ActionMap {
    action_to_keys: HashMap<Action, Vec<KeyEvent>>,
    key_to_action: HashMap<KeyEvent, Action>,
}

impl ActionMap {
    pub fn get_action(&self, key_event: KeyEvent) -> Option<&Action> {
        self.key_to_action.get(&key_event)
    }

    /// Creates an ActionMap from raw keybinds.
    /// * `raw_bindings`: Map of actions to keybind strings.
    pub fn from_raw_bindings(raw_bindings: &RawKeyBindings) -> Self {
        let mut action_to_keys: HashMap<Action, Vec<KeyEvent>> = HashMap::new();
        let mut key_to_action: HashMap<KeyEvent, Action> = HashMap::new();

        for (action, raw_keybinds) in raw_bindings {
            let mut keybinds: Vec<KeyEvent> = Vec::new();

            for raw_keybind in raw_keybinds {
                if let Some(keybind) = parse_keybind(raw_keybind) {
                    keybinds.push(keybind);
                    key_to_action.insert(keybind, *action);
                }
            }

            action_to_keys.insert(*action, keybinds);
        }

        ActionMap {
            action_to_keys,
            key_to_action,
        }
    }
}

// Default key bindings
impl Default for ActionMap {
    fn default() -> Self {
        ActionMap::from_raw_bindings(&HashMap::from([
            (Action::Quit, string_vec!["q"]),
            (Action::Left, string_vec!["left", "h"]),
            (Action::Right, string_vec!["right", "l"]),
            (Action::Up, string_vec!["up", "k"]),
            (Action::Down, string_vec!["down", "j"]),
            (Action::TimerToggle, string_vec!["space"]),
            (Action::TimerStartRelease, string_vec!["release:space"]),
        ]))
    }
}
