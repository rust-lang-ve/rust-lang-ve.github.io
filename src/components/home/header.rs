use yew::prelude::*;

pub struct Header {
  link: ComponentLink<Self>,
}

pub enum Msg {
  ChildClicked
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Header {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Header {
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
      <header class="flex text-black bg-gray-100 p-4 box-border">
        <div class="flex">
          <img src="https://avatars3.githubusercontent.com/u/68873317?s=200&v=4" alt="Organization Logo" height="60" width="60" />
          <h1 class="box-border px-4">{ "Rust Venezuela" }</h1>
        </div>
      </header>
    }
  }
}
