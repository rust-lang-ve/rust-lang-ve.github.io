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
      <section id="home">
        <div id="blur"></div>
        <div id="content">
          <img
            id="logo"
            alt="Rust Venezuela Logo"
            src="https://raw.githubusercontent.com/rust-lang-ve/design/master/assets/ferris.png"
            height="120"
            width="180"
          />
          <h2>{ "Organización de Rust en Venezuela" }</h2>
          <p id="introduction">
            { "Organización para programadores " }
            <a href="https://www.rust-lang.org/" target="_blank" class="link">{ "Rust" }</a>
            { "en Venezuela, abierta a programadores de otras comunidades, interesados tanto en el lenguaje" }
            <strong>{ "Rust" }</strong>
            { ", como en la programación en general." }
          </p>
          <button>
            <span>{ "Únete al grupo en Telegram!" }</span>
          </button>
        </div>
      </section>
    }
  }
}
