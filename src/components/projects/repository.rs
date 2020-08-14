use crate::components::projects::lang::Lang;
use yew::prelude::*;

pub struct Repository {
  pub description: String,
  pub language: Lang,
  pub name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub description: String,
  pub language: Option<String>,
  pub name: String,
}

impl Component for Repository {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    let mut lang: Lang = Lang::NoLanguage;

    if let Some(language) = props.language {
      lang = Lang::from(language);
    }

    Self {
      description: props.description,
      language: lang,
      name: props.name,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let (class_name, language_text) = self.language.markup_resources();

    html! {
      <li class="repository">
        <main>
          <strong>{ self.name.clone() }</strong>
            <p>{ self.description.clone() }</p>
          </main>
        <footer>
          <span class=class_name>{ language_text }</span>
        </footer>
      </li>
    }
  }
}
