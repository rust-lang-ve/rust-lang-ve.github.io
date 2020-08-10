use yew::prelude::*;

pub struct Social {
  link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Social {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Social {
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
