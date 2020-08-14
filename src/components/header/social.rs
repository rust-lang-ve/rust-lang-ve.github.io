use crate::components::icon::{
  Icon,
  icon_name::IconName
};

use yew::prelude::*;

pub struct Social;

impl Component for Social {
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
      <ul id="social">
        <li>
          <Icon icon_name=IconName::Telegram href="https://t.me/rustlangVE".to_string() />
        </li>
        <li>
          <Icon icon_name=IconName::GitHub href="https://github.com/rust-lang-ve".to_string() />
        </li>
      </ul>
    }
  }
}
