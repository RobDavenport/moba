use crate::engine::input_command::InputCommand;

//#[derive(Debug, PartialEq)]
pub struct ReceiveInput {
    pub command: Option<InputCommand>,
    pub next_command: Option<InputCommand>,
}

impl ReceiveInput {
    pub fn new() -> Self {
        Self {
            command: None,
            next_command: None,
        }
    }
}
