use amethyst::{WindowEvent, Event};
use amethyst;
use time;
use remawin;
use remawin::raw::{RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use remawin::types::{DeviceType, WindowData, ActionMetadata, KeyCode};
use remawin::InputReMapper;

use serde::de::DeserializeOwned;

use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::clone::Clone;
use std::default::Default;

pub struct AmethystEventMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone,
          ID: Hash + Eq + Clone + Debug {
    input_remapper : InputReMapper<ACTION, ID>,
    window_data : WindowData,
}

impl <ACTION, ID> AmethystEventMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone + ActionMetadata + Debug + DeserializeOwned,
          ID: Hash + Eq + Clone + Debug + DeserializeOwned {

    pub fn new(current_size : (f64, f64)) -> AmethystEventMapper<ACTION, ID> {
        AmethystEventMapper {
            input_remapper : InputReMapper::new(),
            window_data : WindowData {
                size : current_size,
                cursor_position : None
            }
        }
    }

    pub fn process_events(&mut self, events : &Vec<WindowEvent>) -> Vec<RawInput> {
        let mut next = self.window_data.clone();
        let raw = events.iter().flat_map(|e| process_event(&e.payload, &mut next)).collect();
        self.window_data = next;
        raw
    }

    pub fn process(&mut self, events : &Vec<WindowEvent>) -> Vec<remawin::Event<ACTION, ID>> {
        let raw_input = self.process_events(events);
        self.input_remapper.process_raw_input(&raw_input)
    }

    pub fn remapper_mut(&mut self) -> &mut InputReMapper<ACTION, ID> {
        &mut self.input_remapper
    }
}

fn process_event(event : &Event, next: &mut WindowData) -> Vec<RawInput> {
    let t = time::precise_time_s();
    match event {
        &Event::Closed => {
            vec![RawInput::new(t, DeviceType::Window, 0, RawInputEvent::Close)]
        },
        &Event::Resized(x, y) => {
            next.size = (x as f64, y as f64);
            vec![RawInput::new(t, DeviceType::Window, 0,
                               RawInputEvent::Resize(x as u32, y as u32))]
        },
        &Event::Focused(b) => {
            vec![RawInput::new(t, DeviceType::Window, 0,
                               RawInputEvent::Focus(b))]
        },
        &Event::ReceivedCharacter(ch) => {
            vec![RawInput::new(t, DeviceType::Keyboard, 0,
                               RawInputEvent::Char(ch))]
        },
        &Event::KeyboardInput(state, _ , virtual_keycode) => {
            vec![RawInput::new(t, DeviceType::Keyboard, 0,
                               RawInputEvent::Key(map_keycode(&virtual_keycode),
                                                  map_action(&state),
                                                  RawInputModifiers::empty()))]
        },
        &Event::MouseInput(state, button) => {
            vec![RawInput::new(t, DeviceType::Mouse, 0,
                               RawInputEvent::Button(map_mouse_button(&button),
                                                     match next.cursor_position {
                                                         Some(position) => position,
                                                         None => (0.0, 0.0)
                                                     },
                                                     map_action(&state),
                                                     RawInputModifiers::empty()))]
        },
        &Event::MouseMoved(x, y) => {
            let mut raw = Vec::new();
            let (x, y) = (x as f64, y as f64);
            raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                                   RawInputEvent::CursorPosition(x/next.size.0,
                                                                 y/next.size.1)));
            match next.cursor_position {
                Some((px, py)) => raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                                                         RawInputEvent::Motion((x-px)/next.size.0,
                                                                               (y-py)/next.size.1))),
                None => ()
            };
            next.cursor_position = Some((x, y));
            raw
        },
        _ => Vec::default()
    }
}

fn map_action(element_state: &amethyst::ElementState) -> RawInputAction {
    match element_state{
        &amethyst::ElementState::Pressed => RawInputAction::Press,
        &amethyst::ElementState::Released => RawInputAction::Release,
    }
}

/*fn map_modifiers(modifiers: &glutin::ModifiersState) -> RawInputModifiers {
    let mut m = RawInputModifiers::empty();
    m.set(remawin::raw::SHIFT, modifiers.shift);
    m.set(remawin::raw::CONTROL, modifiers.ctrl);
    m.set(remawin::raw::ALT, modifiers.alt);
    m.set(remawin::raw::SUPER, modifiers.logo);
    m
}*/

fn map_mouse_button(button: &amethyst::MouseButton) -> u32 {
    match button {
        &amethyst::MouseButton::Left => 1,
        &amethyst::MouseButton::Right => 2,
        &amethyst::MouseButton::Middle => 3,
        &amethyst::MouseButton::Other(b) => b as u32,
    }
}

fn map_keycode(keycode: &Option<amethyst::VirtualKeyCode>) -> KeyCode {
    match *keycode {
        Some(kc) => {
            match kc {
                amethyst::VirtualKeyCode::Key1 => KeyCode::Key1,
                amethyst::VirtualKeyCode::Key2 => KeyCode::Key2,
                amethyst::VirtualKeyCode::Key3 => KeyCode::Key3,
                amethyst::VirtualKeyCode::Key4 => KeyCode::Key4,
                amethyst::VirtualKeyCode::Key5 => KeyCode::Key5,
                amethyst::VirtualKeyCode::Key6 => KeyCode::Key6,
                amethyst::VirtualKeyCode::Key7 => KeyCode::Key7,
                amethyst::VirtualKeyCode::Key8 => KeyCode::Key8,
                amethyst::VirtualKeyCode::Key9 => KeyCode::Key9,
                amethyst::VirtualKeyCode::Key0 => KeyCode::Key0,
                amethyst::VirtualKeyCode::A => KeyCode::A,
                amethyst::VirtualKeyCode::B => KeyCode::B,
                amethyst::VirtualKeyCode::C => KeyCode::C,
                amethyst::VirtualKeyCode::D => KeyCode::D,
                amethyst::VirtualKeyCode::E => KeyCode::E,
                amethyst::VirtualKeyCode::F => KeyCode::F,
                amethyst::VirtualKeyCode::G => KeyCode::G,
                amethyst::VirtualKeyCode::H => KeyCode::H,
                amethyst::VirtualKeyCode::I => KeyCode::I,
                amethyst::VirtualKeyCode::J => KeyCode::J,
                amethyst::VirtualKeyCode::K => KeyCode::K,
                amethyst::VirtualKeyCode::L => KeyCode::L,
                amethyst::VirtualKeyCode::M => KeyCode::M,
                amethyst::VirtualKeyCode::N => KeyCode::N,
                amethyst::VirtualKeyCode::O => KeyCode::O,
                amethyst::VirtualKeyCode::P => KeyCode::P,
                amethyst::VirtualKeyCode::Q => KeyCode::Q,
                amethyst::VirtualKeyCode::R => KeyCode::R,
                amethyst::VirtualKeyCode::S => KeyCode::S,
                amethyst::VirtualKeyCode::T => KeyCode::T,
                amethyst::VirtualKeyCode::U => KeyCode::U,
                amethyst::VirtualKeyCode::V => KeyCode::V,
                amethyst::VirtualKeyCode::W => KeyCode::W,
                amethyst::VirtualKeyCode::X => KeyCode::X,
                amethyst::VirtualKeyCode::Y => KeyCode::Y,
                amethyst::VirtualKeyCode::Z => KeyCode::Z,
                amethyst::VirtualKeyCode::Escape => KeyCode::Escape,
                amethyst::VirtualKeyCode::F1 => KeyCode::F1,
                amethyst::VirtualKeyCode::F2 => KeyCode::F2,
                amethyst::VirtualKeyCode::F3 => KeyCode::F3,
                amethyst::VirtualKeyCode::F4 => KeyCode::F4,
                amethyst::VirtualKeyCode::F5 => KeyCode::F5,
                amethyst::VirtualKeyCode::F6 => KeyCode::F6,
                amethyst::VirtualKeyCode::F7 => KeyCode::F7,
                amethyst::VirtualKeyCode::F8 => KeyCode::F8,
                amethyst::VirtualKeyCode::F9 => KeyCode::F9,
                amethyst::VirtualKeyCode::F10 => KeyCode::F10,
                amethyst::VirtualKeyCode::F11 => KeyCode::F11,
                amethyst::VirtualKeyCode::F12 => KeyCode::F12,
                amethyst::VirtualKeyCode::F13 => KeyCode::F13,
                amethyst::VirtualKeyCode::F14 => KeyCode::F14,
                amethyst::VirtualKeyCode::F15 => KeyCode::F15,
                amethyst::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
                amethyst::VirtualKeyCode::Scroll => KeyCode::Scroll,
                amethyst::VirtualKeyCode::Pause => KeyCode::Pause,
                amethyst::VirtualKeyCode::Insert => KeyCode::Insert,
                amethyst::VirtualKeyCode::Home => KeyCode::Home,
                amethyst::VirtualKeyCode::Delete => KeyCode::Delete,
                amethyst::VirtualKeyCode::End => KeyCode::End,
                amethyst::VirtualKeyCode::PageDown => KeyCode::PageDown,
                amethyst::VirtualKeyCode::PageUp => KeyCode::PageUp,
                amethyst::VirtualKeyCode::Left => KeyCode::Left,
                amethyst::VirtualKeyCode::Up => KeyCode::Up,
                amethyst::VirtualKeyCode::Right => KeyCode::Right,
                amethyst::VirtualKeyCode::Down => KeyCode::Down,
                amethyst::VirtualKeyCode::Back => KeyCode::Back,
                amethyst::VirtualKeyCode::Return => KeyCode::Return,
                amethyst::VirtualKeyCode::Space => KeyCode::Space,
                amethyst::VirtualKeyCode::Compose => KeyCode::Compose,
                amethyst::VirtualKeyCode::Numlock => KeyCode::Numlock,
                amethyst::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
                amethyst::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
                amethyst::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
                amethyst::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
                amethyst::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
                amethyst::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
                amethyst::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
                amethyst::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
                amethyst::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
                amethyst::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
                amethyst::VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
                amethyst::VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
                amethyst::VirtualKeyCode::Add => KeyCode::Add,
                amethyst::VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
                amethyst::VirtualKeyCode::Apps => KeyCode::Apps,
                amethyst::VirtualKeyCode::At => KeyCode::At,
                amethyst::VirtualKeyCode::Ax => KeyCode::Ax,
                amethyst::VirtualKeyCode::Backslash => KeyCode::Backslash,
                amethyst::VirtualKeyCode::Calculator => KeyCode::Calculator,
                amethyst::VirtualKeyCode::Capital => KeyCode::Capital,
                amethyst::VirtualKeyCode::Colon => KeyCode::Colon,
                amethyst::VirtualKeyCode::Comma => KeyCode::Comma,
                amethyst::VirtualKeyCode::Convert => KeyCode::Convert,
                amethyst::VirtualKeyCode::Decimal => KeyCode::Decimal,
                amethyst::VirtualKeyCode::Divide => KeyCode::Divide,
                amethyst::VirtualKeyCode::Equals => KeyCode::Equals,
                amethyst::VirtualKeyCode::Grave => KeyCode::Grave,
                amethyst::VirtualKeyCode::Kana => KeyCode::Kana,
                amethyst::VirtualKeyCode::Kanji => KeyCode::Kanji,
                amethyst::VirtualKeyCode::LAlt => KeyCode::LAlt,
                amethyst::VirtualKeyCode::LBracket => KeyCode::LBracket,
                amethyst::VirtualKeyCode::LControl => KeyCode::LControl,
                amethyst::VirtualKeyCode::LMenu => KeyCode::LMenu,
                amethyst::VirtualKeyCode::LShift => KeyCode::LShift,
                amethyst::VirtualKeyCode::LWin => KeyCode::LWin,
                amethyst::VirtualKeyCode::Mail => KeyCode::Mail,
                amethyst::VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
                amethyst::VirtualKeyCode::MediaStop => KeyCode::MediaStop,
                amethyst::VirtualKeyCode::Minus => KeyCode::Minus,
                amethyst::VirtualKeyCode::Multiply => KeyCode::Multiply,
                amethyst::VirtualKeyCode::Mute => KeyCode::Mute,
                amethyst::VirtualKeyCode::MyComputer => KeyCode::MyComputer,
                amethyst::VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
                amethyst::VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
                amethyst::VirtualKeyCode::NextTrack => KeyCode::NextTrack,
                amethyst::VirtualKeyCode::NoConvert => KeyCode::NoConvert,
                amethyst::VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
                amethyst::VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                amethyst::VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
                amethyst::VirtualKeyCode::OEM102 => KeyCode::OEM102,
                amethyst::VirtualKeyCode::Period => KeyCode::Period,
                amethyst::VirtualKeyCode::PlayPause => KeyCode::PlayPause,
                amethyst::VirtualKeyCode::Power => KeyCode::Power,
                amethyst::VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
                amethyst::VirtualKeyCode::RAlt => KeyCode::RAlt,
                amethyst::VirtualKeyCode::RBracket => KeyCode::RBracket,
                amethyst::VirtualKeyCode::RControl => KeyCode::RControl,
                amethyst::VirtualKeyCode::RMenu => KeyCode::RMenu,
                amethyst::VirtualKeyCode::RShift => KeyCode::RShift,
                amethyst::VirtualKeyCode::RWin => KeyCode::RWin,
                amethyst::VirtualKeyCode::Semicolon => KeyCode::Semicolon,
                amethyst::VirtualKeyCode::Slash => KeyCode::Slash,
                amethyst::VirtualKeyCode::Sleep => KeyCode::Sleep,
                amethyst::VirtualKeyCode::Stop => KeyCode::Stop,
                amethyst::VirtualKeyCode::Subtract => KeyCode::Subtract,
                amethyst::VirtualKeyCode::Sysrq => KeyCode::Sysrq,
                amethyst::VirtualKeyCode::Tab => KeyCode::Tab,
                amethyst::VirtualKeyCode::Underline => KeyCode::Underline,
                amethyst::VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
                amethyst::VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
                amethyst::VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
                amethyst::VirtualKeyCode::Wake => KeyCode::Wake,
                amethyst::VirtualKeyCode::WebBack => KeyCode::WebBack,
                amethyst::VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
                amethyst::VirtualKeyCode::WebForward => KeyCode::WebForward,
                amethyst::VirtualKeyCode::WebHome => KeyCode::WebHome,
                amethyst::VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
                amethyst::VirtualKeyCode::WebSearch => KeyCode::WebSearch,
                amethyst::VirtualKeyCode::WebStop => KeyCode::WebStop,
                amethyst::VirtualKeyCode::Yen => KeyCode::Yen,
            }
        },
        None => KeyCode::None
    }
}
