use yew::prelude::*;

pub struct Branding {
  link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Branding {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Branding {
      link,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <div id="branding">
        <img
          alt="Rust Venezuela Logo"
          src="https://avatars3.githubusercontent.com/u/68873317?s=38&v=4"
          height="38"
          width="38"
        />
        <h1>{ "Rust Venezuela" }</h1>
      </div>
    }
  }
}
