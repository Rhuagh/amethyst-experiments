use remawin::{ActionMetadata, ActionArgument, MappedType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum InputContext {
    Default
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
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
