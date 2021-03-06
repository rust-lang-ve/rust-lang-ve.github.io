use yew::prelude::*;

pub struct Title {
    href: String,
    text: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub href: String,
    pub text: String,
}

impl Component for Title {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            href: props.href,
            text: props.text,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
          <h2 class="title">
            <a href={format!("#{}", self.href.clone())}>
              {self.text.clone()}
            </a>
          </h2>
        }
    }
}
