use actix_web_flash_messages::{IncomingFlashMessages, Level};

pub struct FlashMessages {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub successes: Vec<String>,
    pub infos: Vec<String>,
}

impl From<IncomingFlashMessages> for FlashMessages {
    fn from(msg: IncomingFlashMessages) -> Self {
        Self {
            errors: extract_flash_messages_level(&msg, Level::Error),
            warnings: extract_flash_messages_level(&msg, Level::Warning),
            successes: extract_flash_messages_level(&msg, Level::Success),
            infos: extract_flash_messages_level(&msg, Level::Info),
        }
    }
}

pub fn extract_flash_messages_level(
    flash_messages: &IncomingFlashMessages,
    level: Level,
) -> Vec<String> {
    flash_messages
        .iter()
        .filter(|m| m.level() == level)
        .map(|m| m.content().to_string())
        .collect::<Vec<_>>()
}
