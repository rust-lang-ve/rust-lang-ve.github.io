use yew::prelude::*;

pub struct Button {
  link: ComponentLink<Self>,
  text: String,
  href: String,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub text: String,
  pub href: String,
}

impl Component for Button {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Button {
      link,
      text: props.text,
      href: props.href,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    if self.href != String::default() {
      html! {
        <a
          href=self.href.clone()
          target="_blank".to_string()
          rel="noopener noreferrer".to_string()
          class="bg-blue-500 hover:bg-blue-800 text-white font-bold py-2 my-4 px-4 rounded"
        >
          {self.text.clone()}
        </a>
      }
    } else {
      html! {
        <button
          type="button".to_string()
          class="bg-blue-500 hover:bg-blue-800 text-white font-bold py-2 my-4 px-4 rounded"
        >
          {self.text.clone()}
        </button>
      }
    }
  }
}
