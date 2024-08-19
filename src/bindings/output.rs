use std::str::FromStr;

use crate::ParseError;

use super::{Message, Progress};

pub enum Output {
    Progress(Progress),
    Message(Message),
}

impl FromStr for Output {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().is_some_and(|c| c == '{') {
            todo!("handle json")
        } else {
            Ok(Self::Message(Message::from_str(s)?))
        }
    }
}
