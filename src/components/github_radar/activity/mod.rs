use crate::api::common::RawFetchResponse;
use crate::api::github::activity::{get_activity, serde::deserialize, types::Event};
use crate::components::github_radar::activity::event::EventView;
use anyhow::Error;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchTask;

mod event;

pub struct State {
    activity: Vec<Event>,
    fetch_error: Option<Error>,
    fetch_error_message: String,
    fetch_loaded: bool,
}

pub struct ActivityView {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    FetchActivity,
    FetchActivityOk(Vec<Event>),
    FetchActivityError(Error),
}

impl Component for ActivityView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::FetchActivity);

        Self {
            state: State {
                activity: Vec::default(),
                fetch_error: None,
                fetch_error_message: String::default(),
                fetch_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchActivity => {
                self.state.fetch_loaded = false;

                let handler = self.link.callback(move |resp: RawFetchResponse<String>| {
                    let (_, data) = resp.into_parts();

                    match data {
                        Ok(body) => {
                            if let Ok(events) = deserialize(body) {
                                Msg::FetchActivityOk(events)
                            } else {
                                Msg::FetchActivityError(Error::msg(
                                    "Something happened deserializing object!",
                                ))
                            }
                        }
                        Err(err) => Msg::FetchActivityError(err),
                    }
                });

                self.task = Some(get_activity(handler));

                true
            }
            Msg::FetchActivityOk(activity) => {
                self.state.activity = activity;
                self.state.fetch_loaded = true;

                true
            }
            Msg::FetchActivityError(err) => {
                self.state.fetch_error_message = err.to_string();
                self.state.fetch_error = Some(err);
                self.state.fetch_loaded = true;

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let github_events: Vec<Html> = self
            .state
            .activity
            .iter()
            .map(|event| {
                html! {
                    <EventView event=event.to_owned() />
                }
            })
            .collect();

        if !self.state.fetch_loaded {
            html! {
                <span>{"Cargando Actividad en GitHub..."}</span>
            }
        } else if self.state.fetch_error.is_some() {
            ConsoleService::error(self.state.fetch_error_message.as_str());

            html! {
                <span>{"Tuvimos problemas cargando la informacion"}</span>
            }
        } else {
            html! {
                <div class="tab-content" id="activity">
                    <ul id="events" style="max-height: 500px; overflow-y: auto;">
                        {github_events}
                    </ul>
                </div>
            }
        }
    }
}
