use yew::prelude::*;

mod tab;
mod tab_content;

pub use tab::Tab;
pub use tab_content::TabContent;

#[derive(Clone, Properties)]
pub struct Props {
    children: Children,
}

pub struct Tabs {
    props: Props,
}

impl Component for Tabs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props: Props { ..props },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="nav-tabs">
                <ol class="nav-tabs-list">
                    { self.props.children.clone() }
                </ol>
            </div>
        }
    }
}
