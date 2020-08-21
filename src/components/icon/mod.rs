pub mod icon_name;

use crate::components::icon::icon_name::IconName;
use yew::prelude::*;

pub struct Icon {
    pub href: String,
    pub icon_name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub href: String,
    pub icon_name: IconName,
}

impl Component for Icon {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let icon_name: String = props.icon_name.to_string();

        Self {
            href: props.href,
            icon_name,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let class_name = format!("icon {} s-28", self.icon_name);

        html! {
          <a href=self.href.clone() target="_blank">
            <span class=class_name></span>
          </a>
        }
    }
}
