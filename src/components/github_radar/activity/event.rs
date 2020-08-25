use crate::api::github::activity::types::Event;
use yew::prelude::*;

pub struct EventView {
    pub props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub event: Event,
}

impl Component for EventView {
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
        true
    }

    fn view(&self) -> Html {
        match &self.props.event {
            Event::PushEvent(data) => {
                let commit_date = data.created_at.clone();
                let commit_message = data.payload.commits.get(0).unwrap().message.clone();
                let actor = &data.actor;
                let repo_name = data.repo.name.clone();

                html! {
                    <li class="event push">
                        <header>
                            <h3>{"Commit"}</h3>
                            <small>{commit_date}</small>
                        </header>
                        <strong>{repo_name}</strong>
                        <br />
                        <span>{commit_message}</span>
                        <footer>
                            <img
                                src=actor.avatar_url.clone()
                                alt=format!("{} Avatar", actor.display_login.clone())
                                width="26"
                                height="26"
                            />
                            <span>{actor.display_login.clone()}</span>
                        </footer>
                    </li>
                }
            }
        }
    }
}
