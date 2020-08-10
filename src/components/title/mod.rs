use yew::prelude::*;

pub struct Title {
  link: ComponentLink<Self>,
  href: String,
  text: String,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub href: String,
  pub text: String,
}

impl Component for Title {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      href: props.href,
      text: props.text,
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
      <h2 class="title">
        <a href={format!("#{}", self.href.clone())} target="_blank" rel="noopener noreferrer">
          {self.text.clone()}
        </a>
      </h2>
    }
  }
}
