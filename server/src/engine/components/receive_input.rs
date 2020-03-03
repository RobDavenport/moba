use crate::engine::input_command::InputCommand;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReceiveInput {
    pub command: Option<InputCommand>,
}
