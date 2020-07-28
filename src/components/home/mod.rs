mod article;
mod header;

use header::Header;
use article::Article;
use yew::prelude::*;

pub struct Home {
  link: ComponentLink<Self>,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Home {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Home {
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
      <section class="rust-wallpaper h-screen w-full text-white">
        <Header />
        <h2>{"Bienvenidos Rustaceans 🦀"}</h2>
      </section>
    }
  }
}
