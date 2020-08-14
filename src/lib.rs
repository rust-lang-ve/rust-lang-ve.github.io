#![recursion_limit = "512"]

mod components;

use crate::components::about::About;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::home::Home;
use crate::components::projects::Projects;
use crate::components::members::Members;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model;

impl Component for Model {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    // Should only return "true" if new properties are different to
    // previously received properties.
    // This component has no properties so we will always return "false".
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <Header />
        <main id="main-content">
          <Home />
          <Projects />
          <About />
          <Members />
          <Footer />
        </main>
      </div>
    }
  }
}

#[wasm_bindgen(start)]
pub fn run_app() {
  App::<Model>::new().mount_to_body();
}
