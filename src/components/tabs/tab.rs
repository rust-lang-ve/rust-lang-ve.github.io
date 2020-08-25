use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    pub title: String,
    pub current_active_tab: usize,
    pub tab_key: usize,
    pub onclick: Callback<usize>,
}

pub struct Tab {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    SetActive,
}

impl Component for Tab {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: Props { ..props },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetActive => {
                self.props.onclick.emit(self.props.tab_key);
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let class_name = if self.props.current_active_tab == self.props.tab_key {
            String::from("nav-tab active")
        } else {
            String::from("nav-tab")
        };

        html! {
            <li class=class_name onclick=self.link.callback(|_| Msg::SetActive)>{&self.props.title.clone()}</li>
        }
    }
}
