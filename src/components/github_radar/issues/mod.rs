use yew::prelude::*;

pub struct GitHubRadar {}

pub enum Msg {}

impl Component for GitHubRadar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section id="github-radar">
                /// Tabs Goes Here
            </section>
        }
    }
}
