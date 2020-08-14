use std::string::ToString;

#[derive(Clone, PartialEq)]
pub enum IconName {
  GitHub,
  Telegram
}

impl ToString for IconName {
  fn to_string(&self) -> String {
    match self {
      IconName::GitHub => String::from("github-icon"),
      IconName::Telegram => String::from("telegram-icon"),
    }
  }
}
