use std::process::Command;

pub struct Builder(Command);

impl Builder {
    pub fn new(ytdl: &str) -> Self {
        Self(Command::new(ytdl))
    }

    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.0
    }
}
