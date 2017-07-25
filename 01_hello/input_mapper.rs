use amethyst::{WindowEvent, Event};
use amethyst;
use remawin;
use time;
use remawin::raw::{RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use remawin::types::{DeviceType, WindowData};
use remawin::InputReMapper;

use std;

pub struct AmethystEventMapper<C, I>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone + remawin::types::ActionMetadata,
          I : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone {
    input_remapper : InputReMapper<C, I>,
    window_data : WindowData,
}

impl <C, I> AmethystEventMapper<C, I>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone + remawin::types::ActionMetadata,
          I : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone{

    pub fn new(current_size : (f64, f64)) -> AmethystEventMapper<C, I> {
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

    pub fn process(&mut self, events : &Vec<WindowEvent>) -> Vec<remawin::Event<C, I>> {
        let raw_input = self.process_events(events);
        self.input_remapper.process_raw_input(&raw_input)
    }

    pub fn remapper_mut(&mut self) -> &mut InputReMapper<C, I> {
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

fn map_keycode(keycode: &Option<amethyst::VirtualKeyCode>) -> remawin::types::KeyCode {
    match *keycode {
        Some(kc) => {
            match kc {
                amethyst::VirtualKeyCode::Key1 => remawin::types::KeyCode::Key1,
                amethyst::VirtualKeyCode::Key2 => remawin::types::KeyCode::Key2,
                amethyst::VirtualKeyCode::Key3 => remawin::types::KeyCode::Key3,
                amethyst::VirtualKeyCode::Key4 => remawin::types::KeyCode::Key4,
                amethyst::VirtualKeyCode::Key5 => remawin::types::KeyCode::Key5,
                amethyst::VirtualKeyCode::Key6 => remawin::types::KeyCode::Key6,
                amethyst::VirtualKeyCode::Key7 => remawin::types::KeyCode::Key7,
                amethyst::VirtualKeyCode::Key8 => remawin::types::KeyCode::Key8,
                amethyst::VirtualKeyCode::Key9 => remawin::types::KeyCode::Key9,
                amethyst::VirtualKeyCode::Key0 => remawin::types::KeyCode::Key0,
                amethyst::VirtualKeyCode::A => remawin::types::KeyCode::A,
                amethyst::VirtualKeyCode::B => remawin::types::KeyCode::B,
                amethyst::VirtualKeyCode::C => remawin::types::KeyCode::C,
                amethyst::VirtualKeyCode::D => remawin::types::KeyCode::D,
                amethyst::VirtualKeyCode::E => remawin::types::KeyCode::E,
                amethyst::VirtualKeyCode::F => remawin::types::KeyCode::F,
                amethyst::VirtualKeyCode::G => remawin::types::KeyCode::G,
                amethyst::VirtualKeyCode::H => remawin::types::KeyCode::H,
                amethyst::VirtualKeyCode::I => remawin::types::KeyCode::I,
                amethyst::VirtualKeyCode::J => remawin::types::KeyCode::J,
                amethyst::VirtualKeyCode::K => remawin::types::KeyCode::K,
                amethyst::VirtualKeyCode::L => remawin::types::KeyCode::L,
                amethyst::VirtualKeyCode::M => remawin::types::KeyCode::M,
                amethyst::VirtualKeyCode::N => remawin::types::KeyCode::N,
                amethyst::VirtualKeyCode::O => remawin::types::KeyCode::O,
                amethyst::VirtualKeyCode::P => remawin::types::KeyCode::P,
                amethyst::VirtualKeyCode::Q => remawin::types::KeyCode::Q,
                amethyst::VirtualKeyCode::R => remawin::types::KeyCode::R,
                amethyst::VirtualKeyCode::S => remawin::types::KeyCode::S,
                amethyst::VirtualKeyCode::T => remawin::types::KeyCode::T,
                amethyst::VirtualKeyCode::U => remawin::types::KeyCode::U,
                amethyst::VirtualKeyCode::V => remawin::types::KeyCode::V,
                amethyst::VirtualKeyCode::W => remawin::types::KeyCode::W,
                amethyst::VirtualKeyCode::X => remawin::types::KeyCode::X,
                amethyst::VirtualKeyCode::Y => remawin::types::KeyCode::Y,
                amethyst::VirtualKeyCode::Z => remawin::types::KeyCode::Z,
                amethyst::VirtualKeyCode::Escape => remawin::types::KeyCode::Escape,
                amethyst::VirtualKeyCode::F1 => remawin::types::KeyCode::F1,
                amethyst::VirtualKeyCode::F2 => remawin::types::KeyCode::F2,
                amethyst::VirtualKeyCode::F3 => remawin::types::KeyCode::F3,
                amethyst::VirtualKeyCode::F4 => remawin::types::KeyCode::F4,
                amethyst::VirtualKeyCode::F5 => remawin::types::KeyCode::F5,
                amethyst::VirtualKeyCode::F6 => remawin::types::KeyCode::F6,
                amethyst::VirtualKeyCode::F7 => remawin::types::KeyCode::F7,
                amethyst::VirtualKeyCode::F8 => remawin::types::KeyCode::F8,
                amethyst::VirtualKeyCode::F9 => remawin::types::KeyCode::F9,
                amethyst::VirtualKeyCode::F10 => remawin::types::KeyCode::F10,
                amethyst::VirtualKeyCode::F11 => remawin::types::KeyCode::F11,
                amethyst::VirtualKeyCode::F12 => remawin::types::KeyCode::F12,
                amethyst::VirtualKeyCode::F13 => remawin::types::KeyCode::F13,
                amethyst::VirtualKeyCode::F14 => remawin::types::KeyCode::F14,
                amethyst::VirtualKeyCode::F15 => remawin::types::KeyCode::F15,
                amethyst::VirtualKeyCode::Snapshot => remawin::types::KeyCode::Snapshot,
                amethyst::VirtualKeyCode::Scroll => remawin::types::KeyCode::Scroll,
                amethyst::VirtualKeyCode::Pause => remawin::types::KeyCode::Pause,
                amethyst::VirtualKeyCode::Insert => remawin::types::KeyCode::Insert,
                amethyst::VirtualKeyCode::Home => remawin::types::KeyCode::Home,
                amethyst::VirtualKeyCode::Delete => remawin::types::KeyCode::Delete,
                amethyst::VirtualKeyCode::End => remawin::types::KeyCode::End,
                amethyst::VirtualKeyCode::PageDown => remawin::types::KeyCode::PageDown,
                amethyst::VirtualKeyCode::PageUp => remawin::types::KeyCode::PageUp,
                amethyst::VirtualKeyCode::Left => remawin::types::KeyCode::Left,
                amethyst::VirtualKeyCode::Up => remawin::types::KeyCode::Up,
                amethyst::VirtualKeyCode::Right => remawin::types::KeyCode::Right,
                amethyst::VirtualKeyCode::Down => remawin::types::KeyCode::Down,
                amethyst::VirtualKeyCode::Back => remawin::types::KeyCode::Back,
                amethyst::VirtualKeyCode::Return => remawin::types::KeyCode::Return,
                amethyst::VirtualKeyCode::Space => remawin::types::KeyCode::Space,
                amethyst::VirtualKeyCode::Compose => remawin::types::KeyCode::Compose,
                amethyst::VirtualKeyCode::Numlock => remawin::types::KeyCode::Numlock,
                amethyst::VirtualKeyCode::Numpad0 => remawin::types::KeyCode::Numpad0,
                amethyst::VirtualKeyCode::Numpad1 => remawin::types::KeyCode::Numpad1,
                amethyst::VirtualKeyCode::Numpad2 => remawin::types::KeyCode::Numpad2,
                amethyst::VirtualKeyCode::Numpad3 => remawin::types::KeyCode::Numpad3,
                amethyst::VirtualKeyCode::Numpad4 => remawin::types::KeyCode::Numpad4,
                amethyst::VirtualKeyCode::Numpad5 => remawin::types::KeyCode::Numpad5,
                amethyst::VirtualKeyCode::Numpad6 => remawin::types::KeyCode::Numpad6,
                amethyst::VirtualKeyCode::Numpad7 => remawin::types::KeyCode::Numpad7,
                amethyst::VirtualKeyCode::Numpad8 => remawin::types::KeyCode::Numpad8,
                amethyst::VirtualKeyCode::Numpad9 => remawin::types::KeyCode::Numpad9,
                amethyst::VirtualKeyCode::AbntC1 => remawin::types::KeyCode::AbntC1,
                amethyst::VirtualKeyCode::AbntC2 => remawin::types::KeyCode::AbntC2,
                amethyst::VirtualKeyCode::Add => remawin::types::KeyCode::Add,
                amethyst::VirtualKeyCode::Apostrophe => remawin::types::KeyCode::Apostrophe,
                amethyst::VirtualKeyCode::Apps => remawin::types::KeyCode::Apps,
                amethyst::VirtualKeyCode::At => remawin::types::KeyCode::At,
                amethyst::VirtualKeyCode::Ax => remawin::types::KeyCode::Ax,
                amethyst::VirtualKeyCode::Backslash => remawin::types::KeyCode::Backslash,
                amethyst::VirtualKeyCode::Calculator => remawin::types::KeyCode::Calculator,
                amethyst::VirtualKeyCode::Capital => remawin::types::KeyCode::Capital,
                amethyst::VirtualKeyCode::Colon => remawin::types::KeyCode::Colon,
                amethyst::VirtualKeyCode::Comma => remawin::types::KeyCode::Comma,
                amethyst::VirtualKeyCode::Convert => remawin::types::KeyCode::Convert,
                amethyst::VirtualKeyCode::Decimal => remawin::types::KeyCode::Decimal,
                amethyst::VirtualKeyCode::Divide => remawin::types::KeyCode::Divide,
                amethyst::VirtualKeyCode::Equals => remawin::types::KeyCode::Equals,
                amethyst::VirtualKeyCode::Grave => remawin::types::KeyCode::Grave,
                amethyst::VirtualKeyCode::Kana => remawin::types::KeyCode::Kana,
                amethyst::VirtualKeyCode::Kanji => remawin::types::KeyCode::Kanji,
                amethyst::VirtualKeyCode::LAlt => remawin::types::KeyCode::LAlt,
                amethyst::VirtualKeyCode::LBracket => remawin::types::KeyCode::LBracket,
                amethyst::VirtualKeyCode::LControl => remawin::types::KeyCode::LControl,
                amethyst::VirtualKeyCode::LMenu => remawin::types::KeyCode::LMenu,
                amethyst::VirtualKeyCode::LShift => remawin::types::KeyCode::LShift,
                amethyst::VirtualKeyCode::LWin => remawin::types::KeyCode::LWin,
                amethyst::VirtualKeyCode::Mail => remawin::types::KeyCode::Mail,
                amethyst::VirtualKeyCode::MediaSelect => remawin::types::KeyCode::MediaSelect,
                amethyst::VirtualKeyCode::MediaStop => remawin::types::KeyCode::MediaStop,
                amethyst::VirtualKeyCode::Minus => remawin::types::KeyCode::Minus,
                amethyst::VirtualKeyCode::Multiply => remawin::types::KeyCode::Multiply,
                amethyst::VirtualKeyCode::Mute => remawin::types::KeyCode::Mute,
                amethyst::VirtualKeyCode::MyComputer => remawin::types::KeyCode::MyComputer,
                amethyst::VirtualKeyCode::NavigateForward => remawin::types::KeyCode::NavigateForward,
                amethyst::VirtualKeyCode::NavigateBackward => remawin::types::KeyCode::NavigateBackward,
                amethyst::VirtualKeyCode::NextTrack => remawin::types::KeyCode::NextTrack,
                amethyst::VirtualKeyCode::NoConvert => remawin::types::KeyCode::NoConvert,
                amethyst::VirtualKeyCode::NumpadComma => remawin::types::KeyCode::NumpadComma,
                amethyst::VirtualKeyCode::NumpadEnter => remawin::types::KeyCode::NumpadEnter,
                amethyst::VirtualKeyCode::NumpadEquals => remawin::types::KeyCode::NumpadEquals,
                amethyst::VirtualKeyCode::OEM102 => remawin::types::KeyCode::OEM102,
                amethyst::VirtualKeyCode::Period => remawin::types::KeyCode::Period,
                amethyst::VirtualKeyCode::PlayPause => remawin::types::KeyCode::PlayPause,
                amethyst::VirtualKeyCode::Power => remawin::types::KeyCode::Power,
                amethyst::VirtualKeyCode::PrevTrack => remawin::types::KeyCode::PrevTrack,
                amethyst::VirtualKeyCode::RAlt => remawin::types::KeyCode::RAlt,
                amethyst::VirtualKeyCode::RBracket => remawin::types::KeyCode::RBracket,
                amethyst::VirtualKeyCode::RControl => remawin::types::KeyCode::RControl,
                amethyst::VirtualKeyCode::RMenu => remawin::types::KeyCode::RMenu,
                amethyst::VirtualKeyCode::RShift => remawin::types::KeyCode::RShift,
                amethyst::VirtualKeyCode::RWin => remawin::types::KeyCode::RWin,
                amethyst::VirtualKeyCode::Semicolon => remawin::types::KeyCode::Semicolon,
                amethyst::VirtualKeyCode::Slash => remawin::types::KeyCode::Slash,
                amethyst::VirtualKeyCode::Sleep => remawin::types::KeyCode::Sleep,
                amethyst::VirtualKeyCode::Stop => remawin::types::KeyCode::Stop,
                amethyst::VirtualKeyCode::Subtract => remawin::types::KeyCode::Subtract,
                amethyst::VirtualKeyCode::Sysrq => remawin::types::KeyCode::Sysrq,
                amethyst::VirtualKeyCode::Tab => remawin::types::KeyCode::Tab,
                amethyst::VirtualKeyCode::Underline => remawin::types::KeyCode::Underline,
                amethyst::VirtualKeyCode::Unlabeled => remawin::types::KeyCode::Unlabeled,
                amethyst::VirtualKeyCode::VolumeDown => remawin::types::KeyCode::VolumeDown,
                amethyst::VirtualKeyCode::VolumeUp => remawin::types::KeyCode::VolumeUp,
                amethyst::VirtualKeyCode::Wake => remawin::types::KeyCode::Wake,
                amethyst::VirtualKeyCode::WebBack => remawin::types::KeyCode::WebBack,
                amethyst::VirtualKeyCode::WebFavorites => remawin::types::KeyCode::WebFavorites,
                amethyst::VirtualKeyCode::WebForward => remawin::types::KeyCode::WebForward,
                amethyst::VirtualKeyCode::WebHome => remawin::types::KeyCode::WebHome,
                amethyst::VirtualKeyCode::WebRefresh => remawin::types::KeyCode::WebRefresh,
                amethyst::VirtualKeyCode::WebSearch => remawin::types::KeyCode::WebSearch,
                amethyst::VirtualKeyCode::WebStop => remawin::types::KeyCode::WebStop,
                amethyst::VirtualKeyCode::Yen => remawin::types::KeyCode::Yen,
            }
        },
        None => remawin::types::KeyCode::None
    }
}
