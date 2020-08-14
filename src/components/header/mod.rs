mod branding;
mod social;

use crate::components::header::branding::Branding;
use crate::components::header::social::Social;

use yew::prelude::*;

pub struct Header;

impl Component for Header {
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
      <header id="header">
        <div id="header-container">
          <Branding />
          <Social />
        </div>
      </header>
    }
  }
}
