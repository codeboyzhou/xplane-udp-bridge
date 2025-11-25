use std::str::FromStr;

pub(crate) struct MessageSpec {
    pub(crate) category: String,
    pub(crate) action: String,
    pub(crate) dtype: String,
    pub(crate) payload: String,
}

impl MessageSpec {
    const MESSAGE_PARTS_SEPARATOR: char = '|';

    const MESSAGE_PARTS_LEN: usize = 4;

    pub(crate) fn get_message_handler_selector(&self) -> String {
        format!("{}|{}|{}", self.category, self.action, self.dtype)
    }
}

impl FromStr for MessageSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message_parts = s.split(Self::MESSAGE_PARTS_SEPARATOR).collect::<Vec<&str>>();
        if message_parts.len() != Self::MESSAGE_PARTS_LEN {
            return Err(format!("invalid message spec: {}", s));
        }
        Ok(Self {
            category: message_parts[0].to_string(),
            action: message_parts[1].to_string(),
            dtype: message_parts[2].to_string(),
            payload: message_parts[3].to_string(),
        })
    }
}
