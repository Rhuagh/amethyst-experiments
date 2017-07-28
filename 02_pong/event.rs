use remawin;
use shrev;

use input::{Action, InputContext};

#[derive(Clone, Debug, PartialEq)]
pub struct IWindowEvent {
    pub payload : remawin::WindowEvent
}

impl IWindowEvent {
    pub fn new(event: remawin::WindowEvent) -> IWindowEvent {
        IWindowEvent {
            payload : event
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControllerEvent {
    pub payload : remawin::ControllerEvent<Action, InputContext>
}

impl ControllerEvent {
    pub fn new(event: remawin::ControllerEvent<Action, InputContext>) -> ControllerEvent {
        ControllerEvent {
            payload : event
        }
    }
}

impl shrev::Event for ControllerEvent {}
impl shrev::Event for IWindowEvent {}

pub fn init_event_system() -> shrev::EventHandler {
    let mut event_handler = shrev::EventHandler::new();
    event_handler.register::<IWindowEvent>();
    event_handler.register::<ControllerEvent>();
    event_handler
}