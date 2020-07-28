use yew::prelude::*;

pub struct Article {
  link: ComponentLink<Self>,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Article {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Article {
      link,
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
      <article class="bg-teal-900 box-border p-4 w-full text-white">
        <h2>{"Bienvenidos Rustaceans 🦀"}</h2>
        <p>
          { "Hello from the othersideee" }
        </p>
      </article>
    }
  }
}
