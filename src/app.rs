use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "Rust Venezuela!" }</p>
        }
    }
}
