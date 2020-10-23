use yew::prelude::*;

pub struct Accordion {
    pub props: Props,
    pub state: State,
    pub link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub href: String,
    pub title: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct State {
    pub is_open: bool,
}

pub enum Msg {
    ToggleActive,
}

impl Component for Accordion {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: Props { ..props },
            state: State { is_open: false },
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleActive => {
                if self.state.is_open {
                    self.state.is_open = false;
                } else {
                    self.state.is_open = true;
                }

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let head_class_name = if self.state.is_open {
            "head active"
        } else {
            "head"
        };

        html! {
            <article class="accordion">
                <div class=head_class_name onclick=self.link.callback(|_| Msg::ToggleActive)>
                    <h2 class="title">
                        <a href={self.props.href.clone()}>{self.props.title.clone()}</a>
                    </h2>
                  <div class="icon"></div>
                </div>
                <div class="content">{self.props.children.clone()}</div>
            </article>
        }
    }
}
