use yew::prelude::*;

pub struct Social {
  link: ComponentLink<Self>,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Social {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Social {
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
      <ul id="social">
        <li>
          <a href="">
            <span class="icon telegram-icon s-28"></span>
          </a>
        </li>
        <li>
          <a href="">
            <span class="icon github-icon s-28"></span>
          </a>
        </li>
      </ul>
    }
  }
}
