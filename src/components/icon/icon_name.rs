use std::string::ToString;

#[derive(Clone, PartialEq)]
pub enum IconName {
    Discord,
    GitHub,
    Telegram,
}

impl ToString for IconName {
    fn to_string(&self) -> String {
        match self {
            IconName::Discord => String::from("discord-icon"),
            IconName::GitHub => String::from("github-icon"),
            IconName::Telegram => String::from("telegram-icon"),
        }
    }
}
