use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub children: Children,
    pub current_active_tab: usize,
    pub tab_key: usize,
}

pub struct TabContent {
    props: Props,
}

impl Component for TabContent {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.props.current_active_tab == self.props.tab_key {
            return html! {
                <div class="tab-content" id=self.props.id>
                    { self.props.children.clone() }
                </div>
            };
        }

        Html::default()
    }
}
