use std::str::FromStr;

use crate::{ParseError, ParseErrorVariant};

pub struct Message {
    r#type: MessageType,
    tag: String,
    content: String,
}

pub enum MessageType {
    Warning,
    Error,
    Neutral,
}

impl FromStr for Message {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r#type = String::new();
        let mut tag = String::new();
        let mut content = String::new();

        // 0 - type
        // 1 - between type and tag
        // 2 - tag
        // 3 - content
        let mut stage: u8 = 0;

        for c in s.chars() {
            match stage {
                0 if c == ':' => stage = 1,
                0 | 1 if c == '[' => stage = 2,
                0 => r#type.push(c),
                1 => {
                    return Err(ParseError::new(
                        s.to_string(),
                        ParseErrorVariant::NonspaceBeforeTags,
                    ))
                }
                2 if c == ']' => stage = 3,
                2 => tag.push(c),
                3 => content.push(c),
                _ => unreachable!(),
            }
        }

        Ok(Self::new(
            MessageType::from_str(&r#type).map_err(|e| e.replace(s.to_string()))?,
            tag,
            content,
        ))
    }
}

impl FromStr for MessageType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "WARNING" => Self::Warning,
            "ERROR" => Self::Error,
            "" => Self::Neutral,
            _ => {
                return Err(ParseError::new(
                    s.to_string(),
                    ParseErrorVariant::UnknownMessageType(s.to_string()),
                ))
            }
        })
    }
}

impl Message {
    pub fn new(r#type: MessageType, tag: String, content: String) -> Self {
        Self {
            r#type,
            tag,
            content,
        }
    }
}
