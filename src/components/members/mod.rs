mod member_list;

use crate::components::members::member_list::MemberList;
use crate::components::title::Title;
use yew::prelude::*;

pub struct Members {}

pub enum Msg {}

impl Component for Members {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <section id="contributors">
            <div id="content">
              <Title text="Miembros" href="contributors" />
              <MemberList />
            </div>
          </section>
        }
    }
}
