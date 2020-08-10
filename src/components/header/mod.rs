mod branding;
mod social;

use crate::components::header::branding::Branding;
use crate::components::header::social::Social;

use yew::prelude::*;

pub struct Header {
  link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Header {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Header {
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
      <header id="header">
        <div id="header-container">
          <Branding />
          <Social />
        </div>
      </header>
    }
  }
}
