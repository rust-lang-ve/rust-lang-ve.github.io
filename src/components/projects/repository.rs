use yew::prelude::*;

pub struct Repository {
  link: ComponentLink<Self>,
  pub title: String,
  pub description: String,
  pub lang: String,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub title: String,
  pub description: String,
  pub lang: String,
}

impl Component for Repository {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      title: props.title,
      description: props.description,
      lang: props.lang,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <li class="repository">
        <main>
          <strong>{self.title.clone()}</strong>
          <p>{self.description.clone()}</p>
        </main>
        <footer>
          <span>{self.lang.clone()}</span>
        </footer>
      </li>
    }
  }
}
