pub struct ParseError {
    content: String,
    error: ParseErrorVariant,
}

impl ParseError {
    pub fn new(content: String, error: ParseErrorVariant) -> Self {
        Self { content, error }
    }

    pub fn replace(mut self, content: String) -> Self {
        self.content = content;
        self
    }
}

pub enum ParseErrorVariant {
    /// There are non whitespace characters between type of a message and the tag of a message,
    /// e.g.
    /// - `ERROR: [youtube] error message` is okay, but
    /// - `ERROR: text [youtube] error message` is not
    NonspaceBeforeTags,

    /// The message type cannot be parsed,
    /// e.g.
    /// - `ERROR: [youtube] error message` is okay, but
    /// - `DONNOWHA: [youtube] error message` is not
    UnknownMessageType(String),
}
