use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};

pub struct Builder(Command);

impl Builder {
    pub fn new(ytdl: &str) -> Self {
        Self(Command::new(ytdl))
    }

    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.0
    }

    pub fn spawn(&mut self) -> Result<Child, std::io::Error> {
        self.0.spawn()
    }

    pub fn run(&mut self) -> Result<(ChildStdout, ChildStderr), std::io::Error> {
        let process = self
            .0
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let stdout = process.stdout.unwrap();
        let stderr = process.stderr.unwrap();

        Ok((stdout, stderr))
    }

    pub fn url(&mut self, s: &str) -> &mut Self {
        self.0.arg(s);
        self
    }
}
