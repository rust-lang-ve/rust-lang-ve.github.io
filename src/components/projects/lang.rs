pub enum Lang {
  Rust,
  CSS,
  NoLanguage
}

impl Lang {
  /// Retrieve the class and the text for the repository
  /// language field as a tuple where `0` is the class name
  /// and `1` the label name
  pub fn markup_resources(&self) -> (String, String) {
    match self {
      Lang::Rust => (String::from("rust"), String::from("Rust")),
      Lang::CSS => (String::from("css"), String::from("CSS")),
      Lang::NoLanguage => (String::default(), String::from("No language")),
    }
  }
}

impl From<String> for Lang {
  fn from(string: String) -> Self {
    match string.to_lowercase().as_str() {
      "rust" => Lang::Rust,
      "css" => Lang::CSS,
      _ => Lang::NoLanguage
    }
  }
}
