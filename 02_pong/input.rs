use remawin::{ActionMetadata, ActionArgument, MappedType, Context};
use remawin::types::{RawType, RawArgs, KeyCode};
use input_mapper::AmethystEventMapper;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum InputContext {
    Default
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Action {
    Exit,
    LeftPaddleUp,
    LeftPaddleDown,
    RightPaddleUp,
    RightPaddleDown,
    StartRound
}

impl ActionMetadata for Action {
    fn mapped_type(&self) -> MappedType {
        match self {
            &Action::Exit => MappedType::Action,
            &Action::LeftPaddleUp => MappedType::State,
            &Action::LeftPaddleDown => MappedType::State,
            &Action::RightPaddleUp => MappedType::State,
            &Action::RightPaddleDown => MappedType::State,
            &Action::StartRound => MappedType::Action
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        Vec::default()
    }
}

pub fn init_input_system(size : (f64, f64),
                         input_path: &str) -> AmethystEventMapper<Action, InputContext> {
    let mut event_mapper = AmethystEventMapper::<Action, InputContext>::new(size);
    event_mapper.remapper_mut()
        .with_bindings_from_file(input_path)
        /*.with_context(Context::new(InputContext::Default)
            .with_mapping(RawType::Key, RawArgs::new().with_keycode(KeyCode::Escape), Action::Exit)
            .with_mapping(RawType::Key, RawArgs::new().with_keycode(KeyCode::W), Action::LeftPaddleUp)
            .with_mapping(RawType::Key, RawArgs::new().with_keycode(KeyCode::S), Action::LeftPaddleDown)
            .with_mapping(RawType::Key, RawArgs::new().with_keycode(KeyCode::Up), Action::RightPaddleUp)
            .with_mapping(RawType::Key, RawArgs::new().with_keycode(KeyCode::Down), Action::RightPaddleDown)
            .with_mapping(RawType::Key, RawArgs::new().with_keycode(KeyCode::Space), Action::StartRound))*/
        .activate_context(&InputContext::Default, 1);
    event_mapper
}