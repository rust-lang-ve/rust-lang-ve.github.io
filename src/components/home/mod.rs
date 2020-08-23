use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
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
                src="https://raw.githubusercontent.com/rust-lang-ve/design/main/assets/ferris.png"
                height="120"
                width="180"
              />
              <h2>{ "Organización de Rust en Venezuela" }</h2>
              <p id="introduction">
                { "Organización para programadores " }
                <a href="https://www.rust-lang.org/" target="_blank" class="link">{ "Rust" }</a>
                { " en Venezuela, abierta a programadores de otras comunidades, interesados tanto en el lenguaje" }
                <strong>{ " Rust" }</strong>
                { ", como en la programación en general." }
              </p>
            </div>
          </section>
        }
    }
}
