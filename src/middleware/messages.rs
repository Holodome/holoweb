use actix_web_flash_messages::IncomingFlashMessages;

pub enum MessageLevel {
    Info,
    Success,
    Warning,
    Error,
    Other,
}

pub struct Message {
    pub level: MessageLevel,
    pub title: String,
    pub contents: String,
}

pub struct Messages {
    messages: Vec<Message>,
}

impl From<IncomingFlashMessages> for Messages {
    fn from(msg: IncomingFlashMessages) -> Self {
        Self {
            messages: msg
                .iter()
                .map(|it| {
                    let level = match it.level() {
                        actix_web_flash_messages::Level::Info => MessageLevel::Info,
                        actix_web_flash_messages::Level::Success => MessageLevel::Success,
                        actix_web_flash_messages::Level::Warning => MessageLevel::Warning,
                        actix_web_flash_messages::Level::Error => MessageLevel::Error,
                        _ => MessageLevel::Other,
                    };
                    let title = match level {
                        MessageLevel::Info => "Info",
                        MessageLevel::Success => "Success",
                        MessageLevel::Warning => "Warning",
                        MessageLevel::Error => "Error",
                        _ => "",
                    }
                    .to_string();
                    Message {
                        level,
                        title,
                        contents: it.content().to_string(),
                    }
                })
                .collect(),
        }
    }
}

impl Messages {
    pub fn empty() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &Message> {
        self.messages.iter()
    }
}
