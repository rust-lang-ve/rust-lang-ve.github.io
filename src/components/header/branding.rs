use yew::prelude::*;

pub struct Branding;

impl Component for Branding {
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
