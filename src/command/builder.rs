use std::process::Command;

pub struct Builder(Command);

impl Builder {
    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.0
    }
}
