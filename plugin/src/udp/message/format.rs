use crate::error::MessageFormatError;
use std::str::FromStr;

pub(crate) struct MessageFormat {
    pub(crate) category: String,
    pub(crate) action: String,
    pub(crate) dtype: String,
    pub(crate) data: String,
}

impl MessageFormat {
    pub(crate) const MESSAGE_PARTS_SEPARATOR: &'static str = "|";

    const MESSAGE_PARTS_LEN: usize = 4;

    pub(crate) fn parse_message_handler_selector(&self) -> String {
        format!("{}|{}|{}", self.category, self.action, self.dtype)
    }
}

impl FromStr for MessageFormat {
    type Err = MessageFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message_parts = s.split(Self::MESSAGE_PARTS_SEPARATOR).collect::<Vec<&str>>();
        if message_parts.len() != Self::MESSAGE_PARTS_LEN {
            return Err(MessageFormatError::InvalidMessageFormat { message: s.to_string() });
        }
        Ok(Self {
            category: message_parts[0].to_string(),
            action: message_parts[1].to_string(),
            dtype: message_parts[2].to_string(),
            data: message_parts[3].to_string(),
        })
    }
}
