use std::str::FromStr;
use remawin::{ActionMetadata, ActionArgument, MappedType};

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
    MoveForward,
    FireAbility1,
    RotateDirection
}

impl ActionMetadata for Action {
    fn mapped_type(&self) -> MappedType {
        match self {
            &Action::Exit => MappedType::Action,
            &Action::MoveForward => MappedType::State,
            &Action::FireAbility1 => MappedType::Action,
            &Action::RotateDirection => MappedType::Range
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match *self {
            Action::FireAbility1 => vec![ActionArgument::CursorPosition],
            _ => Vec::default()
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "Exit" => Ok(Action::Exit),
            "MoveForward" => Ok(Action::MoveForward),
            "FireAbility1" => Ok(Action::FireAbility1),
            "RotateDirection" => Ok(Action::RotateDirection),
            _ => Err(())
        }
    }
}
