use std::str::FromStr;
use remawin::{ActionMetadata, ActionArgument, MappedType};
use input_mapper::AmethystEventMapper;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputContext {
    Default
}

impl FromStr for InputContext {
    type Err = ();

    fn from_str(s: &str) -> Result<InputContext, ()> {
        match s {
            "Default" => Ok(InputContext::Default),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "Exit" => Ok(Action::Exit),
            "LeftPaddleUp" => Ok(Action::LeftPaddleUp),
            "LeftPaddleDown" => Ok(Action::LeftPaddleDown),
            "RightPaddleUp" => Ok(Action::RightPaddleUp),
            "RightPaddleDown" => Ok(Action::RightPaddleDown),
            "StartRound" => Ok(Action::StartRound),
            _ => Err(())
        }
    }
}

pub fn init_input_system(size : (f64, f64),
                         input_path: &str) -> AmethystEventMapper<Action, InputContext> {
    let mut event_mapper = AmethystEventMapper::<Action, InputContext>::new(size);
    event_mapper.remapper_mut()
        .with_bindings_file(input_path)
        .activate_context(&InputContext::Default, 1);
    event_mapper
}