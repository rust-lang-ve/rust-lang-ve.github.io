mod header;

use crate::components::button::Button;
use header::Header;
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
      <section class="h-screen w-full text-white bg-gray-900">
        <Header />
        <div class="flex flex-col items-center justify-center p-4 box-border">
          <img class="my-10" alt="Rust Venezuela Logo" src="https://avatars3.githubusercontent.com/u/68873317?s=200&v=4" height="200" width="200" />
          <h2 class="my-4">{"Bienvenidos Rustaceans 🦀"}</h2>
          <Button text=String::from("Visita nuestro GitHub") href="https://github.com/rust-lang-ve" />
        </div>
      </section>
    }
  }
}
