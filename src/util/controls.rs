use glfw::{Key, MouseButton, Action, Window};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum KeyState {
    Active,
    Inactive,
    Clicked,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum KeyAction {
    Forward,
    Back,
    Left,
    Right,
    Jump,
    Sprint,
    Crouch,
}

pub struct Binding {
    pub code: Option<Key>,
    pub m_button: Option<MouseButton>,
    pub action: KeyAction,
    pub state: KeyState,
}

pub struct Controls {
    bindings: Vec<Binding>,
    state_map: HashMap<KeyAction, KeyState>,
    pub cursor_speed: f32
}

impl Controls {
    pub fn default() -> Controls {
        let bindings: Vec<Binding> = vec![
            Binding { code: Some(Key::W), m_button: None, action: KeyAction::Forward, state: KeyState::Inactive },
            Binding { code: Some(Key::S), m_button: None, action: KeyAction::Back, state: KeyState::Inactive },
            Binding { code: Some(Key::A), m_button: None, action: KeyAction::Left, state: KeyState::Inactive },
            Binding { code: Some(Key::D), m_button: None, action: KeyAction::Right, state: KeyState::Inactive },
            Binding { code: Some(Key::Space), m_button: None, action: KeyAction::Jump, state: KeyState::Inactive },
            Binding { code: Some(Key::LeftShift), m_button: None, action: KeyAction::Sprint, state: KeyState::Inactive },
            Binding { code: Some(Key::LeftControl), m_button: None, action: KeyAction::Crouch, state: KeyState::Inactive },
        ];

        Controls { bindings, cursor_speed: 0.05, state_map: HashMap::new() }
    }

    pub fn key_state(&self, action: KeyAction) -> &KeyState {
        self.state_map.get(&action).unwrap_or(&KeyState::Inactive)
    }

    pub fn update(&mut self, window: &Window) {
        let ref mut map = self.state_map;
        self.bindings.iter_mut().for_each(|b| {
            let result = if b.code == None {
                window.get_mouse_button(b.m_button.unwrap())
            } else {
                window.get_key(b.code.unwrap())
            };
            if result == Action::Press {
                b.state = KeyState::Active;
            } else if b.state == KeyState::Active {
                b.state = KeyState::Clicked;
            } else {
                b.state = KeyState::Inactive;
            }
            map.insert(b.action, b.state.clone());
        });
    }
}